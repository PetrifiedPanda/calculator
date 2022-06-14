/*
This is the grammar used by this parser:
val_expr := add_sub_expr
add_sub_expr := mul_div_expr (add_sub_op mul_div_expr)*
mul_div_expr := unary_expr (mul_div_op unary_expr)*
unary_expr := un_op? exp_expr
exp_expr := atom ("^" atom)* 
atom := NUM | bracket_expr | VAR_NAME
bracket_expr := "(" val_expr ")"
add_sub_op := "+" | "-"
mul_div_op := "*" | "/"
un_op := "-"
var_expr := VAR_NAME "=" val_expr

translation_unit := var_expr | val_expr | quit

VAR_NAME := TODO: add more constraints
NUM := numeric literal that can be parsed
*/

use std::collections::HashMap;

use crate::tokenizer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current_token: Token,
    var_table: HashMap<String, f64>
}

pub enum ParserResult {
    Value(f64),
    VarAssign,
    Quit
}

impl Parser {
    pub fn new() -> Parser {
        Parser {tokens: Vec::new(), current_token: Token::Invalid, var_table: HashMap::new()}
    }

    pub fn set_tokens(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
        self.next_token();
    }

    fn next_token(&mut self) {
        if !self.tokens.is_empty() {
            self.current_token = self.tokens.remove(0)
        } else {
            self.current_token = Token::Invalid;
        }
    }

    fn accept(&mut self, expected_type: Token) {
        if self.current_token.same_type(&expected_type) {
            self.next_token(); 
        } else {
            panic!("Expected Token of type {:?} but got type {:?}", expected_type, self.current_token);
        }
    }

    fn accept_it(&mut self) {
        self.next_token();
    }

    pub fn get_var_table(&self) -> &HashMap<String, f64> {
        &self.var_table
    }

    pub fn parse_translation_unit(&mut self) -> ParserResult {
        match &self.current_token {
            Token::VarName(_) => {
                if self.tokens[0] != Token::Equals {
                    ParserResult::Value(self.parse_val_expr())
                } else {
                    self.parse_var_assignment();
                    ParserResult::VarAssign
                }
            },

            Token::Quit => {
               ParserResult::Quit
            },

            _ => ParserResult::Value(self.parse_val_expr())
        }
    }
    
    pub fn parse_var_assignment(&mut self) {
        let var_name = match &self.current_token {
            Token::VarName(name) => name.clone(),
            _ => panic!("Expected a VarName but got {:?}", self.current_token),
        };
        self.accept(Token::VarName("".to_string()));
        self.accept(Token::Equals);
        let var_value = self.parse_val_expr();
        self.register_variable(var_name, var_value);
    }

    pub fn parse_val_expr(&mut self) -> f64 {
        self.parse_add_sub_expr()
    }

    fn parse_add_sub_expr(&mut self) -> f64 {
        let mut result = self.parse_mul_div_expr();
        while self.current_token == Token::Add || self.current_token == Token::Sub {
            match &self.current_token {
                Token::Add => {
                    self.accept_it();
                    result += self.parse_mul_div_expr();
                },
                Token::Sub => {
                    self.accept_it();
                    result -= self.parse_mul_div_expr();
                }
                _ => unreachable!(),
            }
        }
        result
    }

    fn parse_mul_div_expr(&mut self) -> f64 {
        let mut result = self.parse_unary_expr();
        while self.current_token == Token::Mul || self.current_token == Token::Div {
            match self.current_token {
                Token::Mul => {
                    self.accept_it();
                    result *= self.parse_unary_expr();
                },
                Token::Div => {
                    self.accept_it();
                    result /= self.parse_unary_expr();
                },
                _ => unreachable!(),
            }
        }
        result
    }

    fn parse_unary_expr(&mut self) -> f64 {
        match self.current_token {
            Token::Sub => {
                self.accept_it();
                return - self.parse_exp_expr();
            },
            _ => {
                return self.parse_exp_expr();
            }
        }
    }

    fn parse_exp_expr(&mut self) -> f64 {
        let mut result = self.parse_atom();
        while self.current_token == Token::Exp {
            self.accept_it();
            result = result.powf(self.parse_atom());
        }
        result
    }

    fn parse_atom(&mut self) -> f64 {
        match &self.current_token {
            Token::VarName(spelling) => { 
                let result = self.var_table[spelling];
                self.accept_it();
                result
            },
            Token::Number(val) => {
                let res = *val;
                self.accept_it();
                res            
            },
            Token::LBracket => self.parse_bracket_expr(),

            _ => panic!("Expected {:?}, {:?} or {:?}, but got {:?}", Token::VarName("".to_string()), Token::Number(0.0), Token::LBracket, self.current_token)
        }
    }

    fn parse_bracket_expr(&mut self) -> f64 {
        self.accept(Token::LBracket);
        let result = self.parse_val_expr();
        self.accept(Token::RBracket);
        result
    }

    fn register_variable(&mut self, var_name: String, value: f64) {
        self.var_table.insert(var_name, value);
    }
}

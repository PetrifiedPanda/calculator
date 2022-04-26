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
use crate::tokenizer::TokenKind;

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
        Parser {tokens: Vec::new(), current_token: Token::new(TokenKind::Invalid, "".to_string()), var_table: HashMap::new()}
    }

    pub fn set_tokens(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
        self.next_token();
    }

    fn next_token(&mut self) {
        if !self.tokens.is_empty() {
            self.current_token = self.tokens.remove(0)
        } else {
            self.current_token = Token::new(TokenKind::Invalid, "".to_string());
        }
    }

    fn accept(&mut self, expected_type: TokenKind) {
        if self.current_token.kind == expected_type {
            self.next_token();
        } else {
            panic!("Expected Token of type {:?} but got type {:?}", expected_type, self.current_token.kind);
        }
    }

    fn accept_it(&mut self) {
        self.next_token();
    }

    pub fn get_var_table(&self) -> &HashMap<String, f64> {
        &self.var_table
    }

    pub fn parse_translation_unit(&mut self) -> ParserResult {
        match &self.current_token.kind {
            TokenKind::VarName => {
                if self.tokens[0].kind != TokenKind::Equals {
                    ParserResult::Value(self.parse_val_expr())
                } else {
                    self.parse_var_assignment();
                    ParserResult::VarAssign
                }
            },

            TokenKind::Quit => {
               ParserResult::Quit
            },

            _ => ParserResult::Value(self.parse_val_expr())
        }
    }
    
    pub fn parse_var_assignment(&mut self) {
        let var_name = self.current_token.spelling.clone(); // Find a way to move this
        self.accept(TokenKind::VarName);
        self.accept(TokenKind::Equals);
        let var_value = self.parse_val_expr();
        self.register_variable(var_name, var_value);
    }

    pub fn parse_val_expr(&mut self) -> f64 {
        self.parse_add_sub_expr()
    }

    fn parse_add_sub_expr(&mut self) -> f64 {
        let mut result = self.parse_mul_div_expr();
        while self.current_token.kind == TokenKind::Plus || self.current_token.kind == TokenKind::Minus {
            let op_type = &self.current_token.kind;
            if *op_type == TokenKind::Plus {
                self.accept_it();
                result += self.parse_mul_div_expr();
            } else {
                self.accept_it();
                result -= self.parse_mul_div_expr();
            }
        }
        result
    }

    fn parse_mul_div_expr(&mut self) -> f64 {
        let mut result = self.parse_unary_expr();
        while self.current_token.kind == TokenKind::Mult || self.current_token.kind == TokenKind::Div {
            let op_type = &self.current_token.kind;
            if *op_type == TokenKind::Mult {
                self.accept_it();
                result *= self.parse_unary_expr();
            } else {
                self.accept_it();
                result /= self.parse_unary_expr();
            }
        }
        result
    }

    fn parse_unary_expr(&mut self) -> f64 {
        if self.current_token.kind == TokenKind::Minus {
            self.accept_it();
            - self.parse_exp_expr()
        } else {
            self.parse_exp_expr()
        }
    }

    fn parse_exp_expr(&mut self) -> f64 {
        let mut result = self.parse_atom();
        while self.current_token.kind == TokenKind::Exp {
            self.accept_it();
            result = result.powf(self.parse_atom());
        }
        result
    }

    fn parse_atom(&mut self) -> f64 {
        match self.current_token.kind {
            TokenKind::VarName => { 
                let result = self.var_table[&self.current_token.spelling];
                self.accept_it();
                result
            },
            TokenKind::Number => {
                let result = self.current_token.spelling.trim().parse().expect("Invalid number");
                self.accept_it();
                result
            },
            TokenKind::LBracket => self.parse_bracket_expr(),

            _ => panic!("Expected {:?}, {:?} or {:?}, but got {:?}", TokenKind::VarName, TokenKind::Number, TokenKind::LBracket, self.current_token.kind)
        }
    }

    fn parse_bracket_expr(&mut self) -> f64 {
        self.accept(TokenKind::LBracket);
        let result = self.parse_val_expr();
        self.accept(TokenKind::RBracket);
        result
    }

    fn register_variable(&mut self, var_name: String, value: f64) {
        self.var_table.insert(var_name, value);
    }
}

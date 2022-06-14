mod tokenizer;
mod parser;

use std::io;
use std::collections::HashMap;

use parser::Parser;
use parser::ParserResult;
use tokenizer::Token;

const DEBUG: bool = false;

fn main() {
    let mut parser = Parser::new();

    let stdin = io::stdin();
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");
        let chars = input.trim().chars().collect::<Vec<char>>(); 
        let tokens = tokenizer::tokenize(chars);
        if DEBUG {
            _print_tokens(&tokens);
        }
        parser.set_tokens(tokens);
        let res = parser.parse_translation_unit();
        match res {
            ParserResult::Value(val) => println!("{}", val),
            ParserResult::VarAssign => {
                if DEBUG {
                    _print_hash_map(parser.get_var_table());
                }
            }
            ParserResult::Quit => {
                if DEBUG {
                    println!("Quitting");
                }
                break;
            }
        }
    }
}

fn _print_tokens(tokens: &[Token]) {
    for token in tokens {
        println!("{:?}", token);
    }
}

fn _print_hash_map(table: &HashMap<String, f64>) {
    for pair in table {
        println!("{}: {}", pair.0, pair.1);
    }
}

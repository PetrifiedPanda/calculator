mod tokenizer;
mod parser;

use std::io;
use std::collections::HashMap;

use parser::Parser;
use tokenizer::Token;

const DEBUG: bool = true;

fn main() {
    let mut parser = Parser::new();
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let chars = input.trim().chars().collect::<Vec<char>>();
        let mut is_var_decl = false;
        if chars.contains(&'=') {
            is_var_decl = true;
        }
        let tokens = tokenizer::tokenize(chars);
        if DEBUG {
            _print_tokens(&tokens);
        }
        parser.set_tokens(tokens);
        if is_var_decl {
            parser.parse_var_assignment();
            if DEBUG {
                _print_hash_map(parser.get_var_tarble());
            }
        } else {
            println!("{}", parser.parse_val_expr());
        }
    }
}

fn _print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        println!("kind: {:?}, spelling: {}", token.kind, token.spelling);
    }
}

fn _print_hash_map(table: &HashMap<String, f64>) {
    for pair in table {
        println!("{}: {}", pair.0, pair.1);
    }
}
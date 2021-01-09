#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Invalid,
    VarName,
    Number,
    Equals,
    Plus,
    Minus,
    Mult,
    Div,
    Exp,
    LBracket,
    RBracket
}

pub struct Token {
    pub kind: TokenKind,
    pub spelling: String,
}

impl Token {
    pub fn new(kind: TokenKind, spelling: String) -> Token {
        Token { kind: kind, spelling: spelling }
    }
}

// TODO: Add better checks for variable names
pub fn tokenize(chars: Vec<char>) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();

    if chars.len() > 0 {
        let mut start_index: usize = usize::MAX;
        for i in 0..chars.len() {
            let current_char = chars[i];
            let kind = match current_char {
                '=' => TokenKind::Equals,
                '+' => TokenKind::Plus,
                '-' => TokenKind::Minus,
                '*' => TokenKind::Mult,
                '/' => TokenKind::Div,
                '^' => TokenKind::Exp,
                '(' => TokenKind::LBracket,
                ')' => TokenKind::RBracket,
                _ => TokenKind::Invalid
            };

            if kind == TokenKind::Invalid {
                if start_index == usize::MAX {
                    start_index = i;
                }
            } else {
                handle_last_token(&mut result, &chars, &mut start_index, i);
                result.push(Token::new(kind, current_char.to_string()));
            }
        }

        if start_index != usize::MAX {
            handle_last_token(&mut result, &chars, &mut start_index, chars.len());
        }
    } else {
        result.push(Token::new(TokenKind::Invalid, "".to_string()));
    }

    return result;
}

fn handle_last_token(result: &mut Vec<Token>, chars: &Vec<char>, start_index: &mut usize, current_index: usize) {
    if *start_index != usize::MAX {
        let mut spelling = &chars[*start_index..current_index];
        if !all_spaces(&spelling) {
            spelling = remove_whitespace(spelling);
            if is_number(&spelling) {
                result.push(Token::new(TokenKind::Number, spelling.into_iter().collect()));
            } else {
                result.push(Token::new(TokenKind::VarName, spelling.into_iter().collect()));
            }
        }

        *start_index = usize::MAX;
    }
}

fn is_number(s: &[char]) -> bool {
    for c in s {
        if !c.is_digit(10) && *c != '.' {
            return false;
        }
    }
    return true;
}

fn all_spaces(s: &[char]) -> bool {
    for c in s {
        if *c != ' ' && *c != '\n' {
            return false;
        }
    }
    return true; 
}

fn remove_whitespace(string: &[char]) -> &[char] {
    let mut front_index = 0;
    while string[front_index] == ' ' {
        front_index += 1;
    }

    let mut back_index = string.len() - 1;
    while string[back_index] == ' ' || string[back_index] == '\n' {
        back_index -= 1;
    }

    return &string[front_index..back_index + 1];
}
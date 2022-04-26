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
        Token { kind, spelling }
    }
}

// TODO: Add better checks for variable names
pub fn tokenize(chars: Vec<char>) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();

    if !chars.is_empty() {
        let mut start_index: Option<usize> = None;
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
                if start_index.is_none() {
                    start_index = Some(i);
                }
            } else {
                handle_last_token(&mut result, &chars, &mut start_index, i);
                result.push(Token::new(kind, current_char.to_string()));
            }
        }

        if start_index.is_some() {
            handle_last_token(&mut result, &chars, &mut start_index, chars.len());
        }
    } else {
        result.push(Token::new(TokenKind::Invalid, "".to_string()));
    }

    result
}

// Tokenizes a multi-character token, whose end has been found through its succeeding single-character token
fn handle_last_token(result: &mut Vec<Token>, chars: &[char], start_index: &mut Option<usize>, current_index: usize) {
    if let Some(start) = *start_index {
        let mut spelling = &chars[start..current_index];
        if !all_spaces(spelling) {
            spelling = trim(spelling);
            if is_number(spelling) {
                result.push(Token::new(TokenKind::Number, spelling.iter().collect()));
            } else {
                result.push(Token::new(TokenKind::VarName, spelling.iter().collect()));
            }
        }

        *start_index = None;
    }
}

fn is_number(s: &[char]) -> bool {
    for c in s {
        if !c.is_digit(10) && *c != '.' {
            return false;
        }
    }
    true
}

fn all_spaces(s: &[char]) -> bool {
    for c in s {
        if !c.is_whitespace() {
            return false;
        }
    }
    true
}

fn trim(string: &[char]) -> &[char] {
    let mut front_index = 0;
    while front_index < string.len() && string[front_index].is_whitespace() {
        front_index += 1;
    }

    let mut back_index = string.len() - 1;
    while back_index >= 1 && string[back_index].is_whitespace() {
        back_index -= 1;
    }

    &string[front_index..back_index + 1]
}

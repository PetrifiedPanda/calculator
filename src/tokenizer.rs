#[derive(PartialEq, Debug)]
pub enum Token {
    Invalid,
    VarName(String),
    Number(f64),
    Equals,
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    LBracket,
    RBracket,
    Quit,
}

impl Token {
    pub fn same_type(&self, other: &Token) -> bool {
        match self {
            Token::VarName(_) => matches!(other, Token::VarName(_)),
            Token::Number(_) => matches!(other, Token::Number(_)),
            _ => self == other,
        }
    }
}


// TODO: Add better checks for variable names
pub fn tokenize(chars: Vec<char>) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();

    if !chars.is_empty() {
        let mut start_index: Option<usize> = None;
        for i in 0..chars.len() {
            let current_char = chars[i];
            let tok = match current_char {
                '=' => Token::Equals,
                '+' => Token::Add,
                '-' => Token::Sub,
                '*' => Token::Mul,
                '/' => Token::Div,
                '^' => Token::Exp,
                '(' => Token::LBracket,
                ')' => Token::RBracket,
                _ => Token::Invalid
            };
            
            match tok {
                Token::Invalid => {
                    if start_index.is_none() {
                        start_index = Some(i);
                    }
                },
                _ => {
                    handle_last_token(&mut result, &chars, &mut start_index, i);
                    result.push(tok);
                }
            }
        }

        if start_index.is_some() {
            handle_last_token(&mut result, &chars, &mut start_index, chars.len());
        }
    } else {
        result.push(Token::Invalid);
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
                let spell = spelling.iter().collect::<String>(); // TODO: collect might not be necessary
                result.push(Token::Number(spell.parse().expect("Invalid number")));
            } else if spelling.iter().collect::<String>().as_str() == "quit" {
                result.push(Token::Quit);
            } else {
                result.push(Token::VarName(spelling.iter().collect()));
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

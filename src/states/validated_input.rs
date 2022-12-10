use crate::{
    regex::{float_re, operation_re},
    token::{Op, Token},
};

use super::tokens::Tokens;

pub struct ValidatedInput(pub String);

impl ValidatedInput {
    pub fn tokenize(self) -> Result<Tokens, String> {
        let tokens = self.tokenize_string()?;
        ValidatedInput::validate_tokens(&tokens)?;
        Ok(Tokens(tokens))
    }

    fn tokenize_string(&self) -> Result<Vec<Token>, String> {
        let input = self.0.to_string();
        let mut tokens: Vec<Token> = vec![];
        let mut pre_token = String::new();
        let float_chars = "0123456789.";
        let minus_char = '-';
        let chars: Vec<char> = input.chars().collect();

        for (i, cur_char) in chars.iter().enumerate() {
            pre_token += cur_char.to_string().as_str();

            // If the cur_char is not the last char
            if i < chars.len() - 1 {
                let next_char = chars[i + 1];

                // Continue to keep the buffer open
                // if cur_char and next_char are both part of a float
                if float_chars.contains(*cur_char) && float_chars.contains(next_char) {
                    continue;
                }

                // or if cur_char and next_char are both *
                if *cur_char == '*' && next_char == '*' {
                    continue;
                }

                // or if cur_char is '-' and it's the first in the buffer and the next char is not an open
                if *cur_char == minus_char && pre_token.len() == 1 && next_char != '(' {
                    let Some(previous_token) = tokens.last() else {
                        continue;
                    };

                    match previous_token {
                        Token::Number(_) => {}
                        Token::Operation(_) => continue,
                        Token::Open => continue,
                        Token::Close => {}
                    }
                }
            }

            if float_re().is_match(&pre_token) {
                tokens.push(Token::Number(pre_token.parse::<f32>().unwrap()))
            } else if operation_re().is_match(&pre_token) {
                tokens.push(Token::Operation(Op::parse(pre_token.clone())?));
            } else if pre_token == "(" {
                tokens.push(Token::Open)
            } else if pre_token == ")" {
                tokens.push(Token::Close)
            } else {
                return Err(format!("Token is invalid: {}", pre_token));
            }

            pre_token = String::new();
        }

        Ok(tokens)
    }

    fn validate_tokens(tokens: &Vec<Token>) -> Result<(), String> {
        if tokens.is_empty() {
            return Err("No tokens to validate.".to_string());
        }

        if let Token::Close = tokens.get(0).expect("tokens.len() must be larger than 0.") {
            return Err("Cannot start with closing parenthesis.".to_string());
        }

        if let Token::Operation(op) = tokens.get(0).expect("tokens.len() must be larger than 0.") {
            return Err(format!("Cannot start with {:?}.", op));
        }

        if let Token::Open = tokens.last().expect("tokens.len() must be larger than 0.") {
            return Err("Cannot end with opening parenthesis.".to_string());
        }

        if let Token::Operation(op) = tokens.last().expect("tokens.len() must be larger than 0.") {
            return Err(format!("Cannot end with {:?}.", op));
        }

        for (i, token) in tokens.iter().enumerate() {
            let Some(next_token) = &tokens.get(i + 1) else {
                break;
            };

            if let Token::Number(num) = token {
                if let Token::Number(next_num) = next_token {
                    return Err(format!(
                        "Number {} cannot be followed by {}.",
                        num, next_num
                    ));
                }
            } else if let Token::Operation(op) = token {
                if let Token::Operation(next_op) = next_token {
                    return Err(format!(
                        "Operation {:?} cannot be followed by {:?}.",
                        op, next_op
                    ));
                }

                if let Token::Close = next_token {
                    return Err(format!(
                        "Operation {:?} cannot be followed by a closing parenthesis.",
                        op,
                    ));
                }
            } else if let Token::Open = token {
                if let Token::Operation(next_op) = next_token {
                    return Err(format!(
                        "Opening parenthesis cannot be followed by {:?}.",
                        next_op
                    ));
                }

                if let Token::Close = next_token {
                    return Err(
                        "Open parenthesis cannot be followed by a closing parenthesis".to_string(),
                    );
                }
            } else if let Token::Close = token {
                if let Token::Number(next_num) = next_token {
                    return Err(format!(
                        "Closing parenthesis cannot be followed by {}.",
                        next_num
                    ));
                }

                if let Token::Open = next_token {
                    return Err(
                        "Closing parenthesis cannot be followed by an opening parenthesis."
                            .to_string(),
                    );
                }
            } else {
                unreachable!("Not all Token variants have been matched.")
            }
        }
        Ok(())
    }
}

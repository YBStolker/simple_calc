use crate::components::regexs::*;

use super::token::{parse_operation, Token};

pub fn eval(input: &String) -> Result<f32, String> {
    let input = sanitize(input);
    validate(&input)?;
    let pre_tokens = pre_tokenize(&input);
    let mut tokens = tokenize(&pre_tokens)?;
    remove_redundant_parentheses(&mut tokens);
    validate_tokens(&tokens)?;

    while let Some(i) = find_next_operation(&mut tokens) {
        perform_operation_at(i, &mut tokens)?;
        remove_redundant_parentheses(&mut tokens);
    }

    match tokens[0] {
        Token::Number(result) => Ok(result),
        _ => Err(format!("Invalid result")),
    }
}

fn sanitize(input: &String) -> String {
    input.trim().replace(" ", "").replace("()", "")
}

/// Validates if there are character usage and parenthesis usage
fn validate(input: &String) -> Result<(), String> {
    if !validation_re().is_match(&input) {
        return Err(format!(
            "Input contains invalid chars. Valid chars: \"0123456789.+-*/() \""
        ));
    }

    let mut opened_count: i32 = 0;
    for (i, _char) in input.chars().enumerate() {
        if _char == '(' {
            opened_count += 1;
        } else if _char == ')' {
            opened_count -= 1;
        }

        if opened_count < 0 {
            return Err(format!("Invalid closing parenthesis at: {}", i));
        }
    }

    if opened_count > 0 {
        return Err(format!(
            "Invalid parenthesis, unclosed count: {}",
            opened_count
        ));
    }

    Ok(())
}

/// Expects sanitized input
fn pre_tokenize(input: &String) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];
    let mut buffer = String::new();
    let float_chars = "0123456789.";
    let chars: Vec<char> = input.chars().collect();

    for (i, cur_char) in chars.iter().enumerate() {
        buffer += cur_char.to_string().as_str();
        // If the cur_char is not the last char
        if i + 1 < chars.len() {
            let next_char = chars[i + 1];
            // if cur_char and next_char are both part of a float
            // or if cur_char and next_char are both *
            // then do not close the buffer
            if float_chars.contains(*cur_char) && float_chars.contains(next_char)
                || *cur_char == '*' && next_char == '*'
            {
                continue;
            }
        }

        // buffer is a complete token so it can be pushed to tokens.
        tokens.push(buffer.clone());
        // create a new buffer for the next token.
        buffer = String::new();
    }

    tokens
}

/// Validate the individual tokens for the validity of the float strings
/// and the operation strings
fn tokenize(pre_tokens: &Vec<String>) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];
    for pre_token in pre_tokens {
        if float_re().is_match(pre_token) {
            tokens.push(Token::Number(pre_token.parse::<f32>().unwrap()))
        } else if operation_re().is_match(pre_token) {
            tokens.push(Token::Operation(pre_token.clone()));
        } else if pre_token == "(" {
            tokens.push(Token::Open)
        } else if pre_token == ")" {
            tokens.push(Token::Close)
        } else {
            return Err(format!("Token is invalid: {}", pre_token));
        }
    }
    Ok(tokens)
}

fn validate_tokens(tokens: &Vec<Token>) -> Result<(), String> {
    if let Token::Close = &tokens[0] {
        return Err(format!("Cannot start with closing parenthesis."));
    }

    if let Token::Operation(op) = &tokens[0] {
        return Err(format!("Cannot start with {}", op));
    }

    if let Token::Open = &tokens[tokens.len() - 1] {
        return Err(format!("Cannot end with opening parenthesis."));
    }

    if let Token::Operation(op) = &tokens[tokens.len() - 1] {
        return Err(format!("Cannot end with {}", op));
    }

    for (i, token) in tokens.iter().enumerate() {
        if i + 1 < tokens.len() {
            let next_token = &tokens[i + 1];
            match token {
                Token::Number(num) => match next_token {
                    Token::Number(next_num) => {
                        return Err(format!("Number {} cannot be followed by {}", num, next_num))
                    }
                    _ => continue,
                },
                Token::Operation(op) => match next_token {
                    Token::Operation(next_op) => {
                        return Err(format!(
                            "Operation {} cannot be followed by {}.",
                            op, next_op
                        ))
                    }
                    Token::Close => {
                        return Err(format!(
                            "Operation {} cannot be followed by a closing parenthesis.",
                            op,
                        ))
                    }
                    _ => continue,
                },
                Token::Open => match next_token {
                    Token::Operation(op) => {
                        return Err(format!("Opening parenthesis cannot be followed by {}.", op));
                    }
                    Token::Close => {
                        return Err(format!(
                            "Open parenthesis cannot be followed by a closing parenthesis."
                        ))
                    }
                    _ => continue,
                },
                Token::Close => match next_token {
                    Token::Number(next_num) => {
                        return Err(format!(
                            "Closing parenthesis cannot be followed by {}.",
                            next_num
                        ))
                    }
                    Token::Open => {
                        return Err(format!(
                            "Closing parenthesis cannot be followed by an opening parenthesis."
                        ))
                    }
                    _ => continue,
                },
            }
        } else {
            match token {
                Token::Open => return Err(format!("Cannot end with an opening parenthesis.")),
                Token::Operation(op) => return Err(format!("Cannot end with {}.", op)),
                _ => continue,
            }
        }
    }
    Ok(())
}

fn has_redundant_parentheses(tokens: &Vec<Token>) -> bool {
    for (i, token) in tokens.iter().enumerate() {
        if let Token::Open = token {
            if let Token::Close = tokens[i + 2] {
                return true;
            }
        }
    }
    false
}

fn remove_redundant_parentheses(tokens: &mut Vec<Token>) -> () {
    while has_redundant_parentheses(tokens) {
        let mut to_remove: Vec<usize> = vec![];

        for (i, token) in tokens.iter().enumerate() {
            if let Token::Open = token {
                if let Token::Close = tokens[i + 2] {
                    to_remove.push(i);
                    to_remove.push(i + 2);
                }
            }
        }

        to_remove.reverse();
        for i in to_remove {
            tokens.remove(i);
        }
    }
}

fn prioritize_next_operation(cur: &String, next: &String) -> bool {
    if ["+", "-"].contains(&cur.as_str()) && ["*", "/", "**"].contains(&next.as_str()) {
        true
    } else if ["*", "/"].contains(&cur.as_str()) && "**" == next {
        true
    } else {
        false
    }
}

fn find_next_operation(tokens: &Vec<Token>) -> Option<usize> {
    for (i, token) in tokens.iter().enumerate() {
        if let Token::Operation(op) = token {
            if i + 2 < tokens.len() {
                if let Token::Operation(next_op) = &tokens[i + 2] {
                    if !prioritize_next_operation(op, next_op) {
                        return Some(i);
                    }
                } else if let Token::Close = &tokens[i + 2] {
                    return Some(i);
                }
            } else {
                return Some(i);
            }
        }
    }

    None
}

fn perform_operation_at(i: usize, tokens: &mut Vec<Token>) -> Result<(), String> {
    if let Token::Number(left) = &tokens[i - 1] {
        if let Token::Operation(operation) = &tokens[i] {
            if let Token::Number(right) = &tokens[i + 1] {
                let result = parse_operation(operation)?(*left, *right);
                tokens[i - 1] = Token::Number(result);
                tokens.remove(i + 1);
                tokens.remove(i);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_tests_validate() -> Result<(), String> {
        validate(&String::from("2+2"))?;
        validate(&String::from("2-2"))?;
        validate(&String::from("2*2"))?;
        validate(&String::from("2/2"))?;
        validate(&String::from("2**2"))?;
        validate(&String::from("((2+2)+(2+2))"))?;
        assert!(validate(&String::from("2+2p")).is_err());
        assert!(validate(&String::from("((2+2)")).is_err());
        assert!(validate(&String::from("(2+2))")).is_err());
        assert!(validate(&String::from(")(2+2)")).is_err());
        Ok(())
    }

    #[test]
    fn it_tests_sanitize() {
        assert_eq!("2+2", sanitize(&String::from("2 + 2\n")));
    }

    #[test]
    fn it_tests_tokenize() {
        assert_eq!(pre_tokenize(&String::from("2+2")), vec!["2", "+", "2"]);
        assert_eq!(
            pre_tokenize(&String::from("2+2**2")),
            vec!["2", "+", "2", "**", "2"]
        );
        assert_eq!(
            pre_tokenize(&String::from("(2.2+2.2)**2")),
            vec!["(", "2.2", "+", "2.2", ")", "**", "2"]
        );
        // tokenize() does not check for valid tokens
        assert_eq!(
            pre_tokenize(&String::from("22..88..88******2")),
            vec!["22..88..88", "******", "2"]
        );
    }

    #[test]
    fn it_tests_validate_tokens() -> Result<(), String> {
        tokenize(&vec![
            String::from("2"),
            String::from("+"),
            String::from("2"),
        ])?;
        tokenize(&vec![
            String::from("("),
            String::from("2"),
            String::from("+"),
            String::from("2"),
            String::from(")"),
        ])?;
        Ok(())
    }

    #[test]
    fn it_tests_eval() -> Result<(), String> {
        assert_eq!(eval(&"(3+3)*3".to_string())?, (3f32 + 3f32) * 3f32);
        assert_eq!(eval(&"((((3+3)*3)))".to_string())?, (3f32 + 3f32) * 3f32);
        assert_eq!(eval(&"(3+3)**3".to_string())?, (3f32 + 3f32).powf(3f32));
        assert_eq!(eval(&"(3+3)**3".to_string())?, (3f32 + 3f32).powf(3f32));
        assert_eq!(eval(&"(18*3)-6".to_string())?, (18f32 * 3f32) - 6f32);
        assert_eq!(
            eval(&"(3+3)**3**(1/3)".to_string())?,
            (3f32 + 3f32).powf(3f32).powf(1f32 / 3f32)
        );
        assert_eq!(
            eval(&"(3+3)**3**(1/3)".to_string())?,
            (3f32 + 3f32).powf(3f32).powf(1f32 / 3f32)
        );
        Ok(())
    }
}

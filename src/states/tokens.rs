use crate::token::{perform_operation, Op, Token};

pub struct Tokens(pub Vec<Token>);

impl Tokens {
    pub fn evaluate(mut self) -> f32 {
        while let Some(i) = self.find_next_operation() {
            self.perform_operation_at(i);
            self.remove_redundant_parentheses();
        }

        if self.0.len() != 1 {
            unreachable!("At this point there should only be 1 token left.")
        }

        let Token::Number(num) = self.0.first().unwrap() else {
            unreachable!("The token must be a number.")
        };

        *num
    }

    fn find_next_operation(&mut self) -> Option<usize> {
        for (i, token) in self.0.iter().enumerate() {
            // We are only looking for tokens, so if it is not a token continue.
            let Token::Operation(cur_op) = token else {
                continue;
            };

            // i + 2 is where we would expect the next operation.
            // if this is the last operation in the list we can safely perform it.
            let Some(maybe_next_op) = self.0.get(i + 2) else {
                return Some(i);
            };

            let next_op = match maybe_next_op {
                // "5 * ( 5 - 2 )". TODO check if i + 1 is an Open.
                //    ^ cur_op
                Token::Number(_) => continue,

                // "5 + 2 * 5" return the '*' and check if it should be prioritized
                //    ^ cur_op
                Token::Operation(op) => op,

                // "5 * 5 ( 5 - 2 )'is not valid syntax at this point and should have been caught earlier
                //    ^ cur_op
                Token::Open => unreachable!("Token validation should prevent this from executing"),

                // "( 5 + 2 ) * 5"
                //      ^ cur_op
                Token::Close => return Some(i),
            };

            if prioritize_next_operation(cur_op, next_op) {
                continue;
            }

            return Some(i);
        }

        None
    }

    fn perform_operation_at(&mut self, i: usize) {
        let Some(left) = self.0.get(i - 1) else {
            unreachable!("Could not find left");
        };

        let Some(operation) = self.0.get(i) else {
            unreachable!("Could not find operation");
        };

        let Some(right) = self.0.get(i + 1) else {
            unreachable!("Could not find right");
        };

        let Token::Number(left) = left else {
            unreachable!("Invalid token at the left place");
        };

        let Token::Operation(operation) = operation else {
            unreachable!("Invalid token at the operation place");
        };

        let Token::Number(right) = right else {
            unreachable!("Invalid token at the right place");
        };

        let result = perform_operation(*left, operation, *right);

        self.0[i - 1] = Token::Number(result); // set the left token to the result
        self.0.remove(i); // remove the operation token
        self.0.remove(i); // remove the right token
    }

    fn remove_redundant_parentheses(&mut self) {
        while has_redundant_parentheses(&self.0) {
            let mut to_remove: Vec<usize> = vec![];

            for (i, token) in self.0.iter().enumerate() {
                let Token::Open = token else {
                    continue;
                };

                let Token::Close = self.0[i + 2] else {
                    continue;
                };

                to_remove.push(i);
                to_remove.push(i + 2);
            }

            to_remove.reverse();
            for i in to_remove {
                self.0.remove(i);
            }
        }
    }
}

fn prioritize_next_operation(cur: &Op, next: &Op) -> bool {
    match cur {
        Op::Add | Op::Sub => match next {
            Op::Add | Op::Sub => false,
            Op::Mul | Op::Div | Op::Pow => true,
        },
        Op::Mul | Op::Div => match next {
            Op::Add | Op::Sub | Op::Mul | Op::Div => false,
            Op::Pow => true,
        },
        Op::Pow => false,
    }
}

fn has_redundant_parentheses(tokens: &[Token]) -> bool {
    for (i, token) in tokens.iter().enumerate() {
        let Token::Open = token else {
            continue;
        };

        if let Token::Close = tokens[i + 2] {
            return true;
        };
    }
    false
}

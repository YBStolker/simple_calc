mod regex;
mod states;
mod token;

use crate::states::input::Input;

pub fn run() {
    println!("Simply calcing...");
    println!("Type 'exit' to exit.");
    loop {
        println!("Give your input:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line for some reason");

        if input.trim() == "exit" {
            break;
        }

        match eval(&input) {
            Ok(num) => println!("> {num}\n"),
            Err(e) => println!("error: {e}\n"),
        };
    }
}

fn eval(input: impl Into<String>) -> Result<f32, String> {
    let input = Input(input.into());
    let sanitized_input = input.sanitize();
    let validated_input = sanitized_input.validate()?;
    let tokens = validated_input.tokenize()?;
    Ok(tokens.evaluate())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_positive() {
        assert_eq!(eval("12+3"), Ok(15_f32));
        assert_eq!(eval("12-3"), Ok(9_f32));
        assert_eq!(eval("12*3"), Ok(36_f32));
        assert_eq!(eval("12/3"), Ok(4_f32));
        assert_eq!(eval("12**3"), Ok(1728_f32));
    }

    #[test]
    fn test_eval_negative() {
        assert_eq!(eval("-12+3"), Ok(-9_f32));
        assert_eq!(eval("-12-3"), Ok(-15_f32));
        assert_eq!(eval("-12*3"), Ok(-36_f32));
        assert_eq!(eval("-12/3"), Ok(-4_f32));
        assert_eq!(eval("-12**3"), Ok(-1728_f32));
    }

    #[test]
    fn test_eval() {
        assert_eq!(eval("2*(-12+3)"), Ok(-18_f32));
        assert_eq!(eval("0.1**-2"), Ok(100_f32));
        assert_eq!(eval("-0.1*2"), Ok(-0.2_f32));
    }
}

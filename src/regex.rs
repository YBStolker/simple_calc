use regex::Regex;

pub fn float_re() -> Regex {
    Regex::new(r"^\-?[0-9]+$|^\-?[0-9]+\.[0-9]+$").unwrap()
}

pub fn operation_re() -> Regex {
    Regex::new(r"^(\+|\-|\*|/|\*\*)$").unwrap()
}

pub fn validation_re() -> Regex {
    Regex::new(r"^[0-9.+\-*/()]+$").unwrap()
}

// -- Tests --

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_tests_float_re() {
        // Legal
        assert!(float_re().is_match("5"));
        assert!(float_re().is_match("5.5"));

        assert!(float_re().is_match("-5"));
        assert!(float_re().is_match("-5.5"));

        // Illegal
        assert!(!float_re().is_match("5."));
        assert!(!float_re().is_match(".5"));
        assert!(!float_re().is_match("5,5"));
        assert!(!float_re().is_match("5p"));
        assert!(!float_re().is_match("5.5f32"));
        assert!(!float_re().is_match("five"));

        assert!(!float_re().is_match("-5."));
        assert!(!float_re().is_match("-.5"));
        assert!(!float_re().is_match("-5,5"));
        assert!(!float_re().is_match("-5p"));
        assert!(!float_re().is_match("-5.5f32"));
        assert!(!float_re().is_match("-five"));
    }

    #[test]
    fn it_tests_operation_re() {
        // Legal
        assert!(operation_re().is_match("+"));
        assert!(operation_re().is_match("-"));
        assert!(operation_re().is_match("*"));
        assert!(operation_re().is_match("/"));
        assert!(operation_re().is_match("**"));

        // Illegal
        assert!(!operation_re().is_match("++"));
        assert!(!operation_re().is_match("/+"));
        assert!(!operation_re().is_match("+/"));
        assert!(!operation_re().is_match("*+"));
        assert!(!operation_re().is_match("+*"));
        assert!(!operation_re().is_match("-+"));
        assert!(!operation_re().is_match("*/"));
        assert!(!operation_re().is_match("//"));
        assert!(!operation_re().is_match("/*"));
        assert!(!operation_re().is_match("***"));
    }

    #[test]
    fn it_tests_validation_re() {
        // Legal
        assert!(validation_re().is_match("5+5-5*(5/5.5)"));
        assert!(validation_re().is_match("5+-5"));
        assert!(validation_re().is_match("5--5"));
        assert!(validation_re().is_match("5*-5"));
        assert!(validation_re().is_match("5/-5"));
        assert!(validation_re().is_match("5**-5"));
        assert!(validation_re().is_match("(5+5)-5"));
        assert!(validation_re().is_match("5+(-5+5)"));

        // Illegal
        assert!(!validation_re().is_match("5+5-5* (5/5.5)"));
        assert!(!validation_re().is_match("5+5-5*(5/5,5)"));
        assert!(!validation_re().is_match("5+5-5*(5/5.5);"));
    }
}

use regex::Regex;

pub fn float_re() -> Regex {
    Regex::new(r"^[0-9]+\.[0-9]+$|^[0-9]+$").unwrap()
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
        assert!(float_re().is_match("5"));
        assert!(float_re().is_match("5.5"));
        assert!(!float_re().is_match("5."));
        assert!(!float_re().is_match(".5"));
        assert!(!float_re().is_match("5,5"));
        assert!(!float_re().is_match("5p"));
        assert!(!float_re().is_match("5.5f32"));
        assert!(!float_re().is_match("five"));
    }

    #[test]
    fn it_tests_operation_re() {
        assert!(operation_re().is_match("+"));
        assert!(operation_re().is_match("-"));
        assert!(operation_re().is_match("*"));
        assert!(operation_re().is_match("/"));
        assert!(operation_re().is_match("**"));
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
        assert!(validation_re().is_match("5+5-5*(5/5.5)"));
        assert!(!validation_re().is_match("5+5-5* (5/5.5)"));
        assert!(!validation_re().is_match("5+5-5*(5/5,5)"));
        assert!(!validation_re().is_match("5+5-5*(5/5.5);"));
    }
}

use crate::regex::validation_re;

use super::validated_input::ValidatedInput;

pub struct SanitizedInput(pub String);

impl SanitizedInput {
    pub fn validate(self) -> Result<ValidatedInput, String> {
        if !validation_re().is_match(&self.0) {
            return Err(format!(
                "Input contains invalid chars. Valid chars: \"0123456789.+-*/() \""
            ));
        }
    
        let mut opened_count: i32 = 0;
        for (i, _char) in self.0.chars().enumerate() {
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
    
        Ok(ValidatedInput(self.0))
    }
}

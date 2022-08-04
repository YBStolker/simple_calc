#[derive(Debug)]
pub enum Token {
    Number(f32),
    Operation(String),
    Open,
    Close,
}

pub fn add(left: f32, right: f32) -> f32 {
    left + right
}

pub fn sub(left: f32, right: f32) -> f32 {
    left - right
}

pub fn mul(left: f32, right: f32) -> f32 {
    left * right
}

pub fn div(left: f32, right: f32) -> f32 {
    left / right
}

pub fn pow(left: f32, right: f32) -> f32 {
    left.powf(right)
}

pub fn parse_operation(operation: &String) -> Result<Box<dyn Fn(f32, f32) -> f32>, String> {
    match operation.as_str() {
        "+" => Ok(Box::new(add)),
        "-" => Ok(Box::new(sub)),
        "*" => Ok(Box::new(mul)),
        "/" => Ok(Box::new(div)),
        "**" => Ok(Box::new(pow)),
        _ => Err(format!("Not a valid operation: {}", operation)),
    }
}

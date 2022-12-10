#[derive(Debug)]
pub enum Token {
    Number(f32),
    Operation(Op),
    Open,
    Close,
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Op {
    pub fn parse(op: String) -> Result<Self, String> {
        match op.as_str() {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            "**" => Ok(Op::Pow),
            _ => Err(format!("Unknown operation {op}")),
        }
    }
}

pub fn perform_operation(
    left: f32,
    operation: &Op,
    right: f32,
) -> f32 {
    match operation {
        Op::Add => left + right,
        Op::Sub => left - right,
        Op::Mul => left * right,
        Op::Div => left / right,
        Op::Pow => left.powf(right),
    }
}

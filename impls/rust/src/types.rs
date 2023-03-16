// use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum MalType {
    Nil,
    Bool(bool),
    Int(i32),
    Symbol(String),
    String(String),
    List(Vec<MalType>),
    // Keyword(String),
    Vector(Vec<MalType>),
    // Map(HashMap<MalType, MalType>),
    Op(IntOp),
}

#[derive(Debug, Copy, Clone)]
pub enum IntOp {
    Plus,
    Minus,
    Times,
    Div,
}

impl IntOp {
    pub fn exec(self, a: i32, b: i32) -> i32 {
        use IntOp::*;
        match self {
            Plus => a + b,
            Minus => a - b,
            Times => a * b,
            Div => a / b,
        }
    }
}

impl std::str::FromStr for IntOp {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IntOp::*;
        match s {
            "+" => Ok(Plus),
            "-" => Ok(Minus),
            "*" => Ok(Times),
            "/" => Ok(Div),
            _ => Err("bad IntOp".to_string()),
        }
    }
}

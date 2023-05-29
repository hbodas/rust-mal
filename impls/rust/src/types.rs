// use std::collections::HashMap;

use std::str::FromStr;

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

#[derive(Clone, Debug)]
pub enum IntOp {
    Plus,
    Minus,
    Times,
    Div,
}

impl FromStr for IntOp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IntOp::*;
        match s {
            "+" => Ok(Plus),
            "-" => Ok(Minus),
            "*" => Ok(Times),
            "/" => Ok(Div),
            _ => Err(format!("bad IntOp")),
        }
    }
}

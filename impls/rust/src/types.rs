#[derive(Debug, Clone)]
pub enum MalType {
    Nil,
    Bool(bool),
    Int(i32),
    Symbol(String),
    String(String),
    List(Vec<MalType>),
}

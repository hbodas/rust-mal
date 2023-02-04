use crate::types::MalType;

pub fn pr_str(t: MalType) -> String {
    match t {
        MalType::Nil => "nil".to_string(),
        MalType::Bool(b) => if b { "true" } else { "false" }.to_string(),
        MalType::Int(n) => n.to_string(),
        MalType::Symbol(s) => s,
        MalType::String(s) => vec!["\"".to_string(), s, "\"".to_string()].join(""),
        MalType::List(v) => vec![
            "(".to_string(),
            v.into_iter()
                .map(pr_str)
                .reduce(|acc, e| format!("{} {}", acc, e))
                .unwrap_or("".to_string()),
            ")".to_string(),
        ]
        .join(""),
    }
}

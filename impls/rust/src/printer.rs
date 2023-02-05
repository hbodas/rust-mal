use crate::types::MalType;

pub fn pr_str(t: MalType, print_readably: bool) -> String {
    // println!("pr_str {:?}", t);
    let replace_fn = |s: String| {
        s.replace("\\", "\\\\")
            .replace("\n", "\\n")
            .replace("\"", "\\\"")
    };

    match t {
        MalType::Nil => "nil".to_string(),
        MalType::Bool(b) => if b { "true" } else { "false" }.to_string(),
        MalType::Int(n) => n.to_string(),
        MalType::Symbol(s) => s,
        MalType::String(s) => vec![
            "\"".to_string(),
            if print_readably { replace_fn(s) } else { s },
            "\"".to_string(),
        ]
        .join(""),
        MalType::List(v) => vec![
            "(".to_string(),
            v.into_iter()
                .map(|s| pr_str(s, print_readably))
                .reduce(|acc, e| format!("{} {}", acc, e))
                .unwrap_or("".to_string()),
            ")".to_string(),
        ]
        .join(""),
        _ => "???".to_string(),
    }
}

use crate::types::MalType;

use regex::Regex;
use std::fmt;

type Token = String;
const DEBUG: bool = true;

macro_rules! dprintln {
    () => {
       if (DEBUG) {print!("\n")};
    };
    ($($arg:tt)*) => {{
        if (DEBUG) {println!($($arg)*)};
    }};
}

#[derive(Debug, Clone)]
pub struct Reader {
    tokens: Vec<Token>,
    position: usize,
}

#[derive(Debug, Clone)]
pub enum ReaderError {
    EOFError,
    _NoTokens,
    _EndRound,
    _EndSquare,
    _EndCurly,
}

impl fmt::Display for ReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "ReplError: reached EOF"),
        }
    }
}

// (S1 DEF) TODO: parens matching
// (S1 DEF) TODO: reader macros
// (S1 DEF) TODO: more types: keyword, hashmap

impl Reader {
    fn new(tokens: Vec<Token>) -> Result<Self, ReaderError> {
        if tokens.len() == 0 {
            Err(ReaderError::_NoTokens)
        } else {
            Ok(Reader {
                tokens,
                position: 0,
            })
        }
    }

    fn next(&mut self) -> Result<Token, ReaderError> {
        match self.tokens.get(self.position) {
            None => Err(ReaderError::EOFError),
            Some(s) => {
                self.position += 1;
                Ok(s.clone())
            }
        }
    }

    fn peek(&self) -> Result<Token, ReaderError> {
        dprintln!("peek {:?} , {:?}", self.tokens, self.position);
        if self.tokens.len() == self.position {
            Err(ReaderError::EOFError)
        } else {
            Ok(self.tokens[self.position].clone())
        }
    }

    fn read_form(&mut self) -> Result<MalType, ReaderError> {
        dprintln!("read_form {:?}", self);
        let peeked: &str = &self.peek()?;
        match peeked {
            "(" => Ok(MalType::List(self.read_list("(")?)),
            "[" => Ok(MalType::Vector(self.read_list("[")?)),
            _ => Ok(self.read_atom()?),
        }
    }

    fn read_list(&mut self, open: &str) -> Result<Vec<MalType>, ReaderError> {
        dprintln!("read_list {:?}", self);
        self.next()?;
        let mut ret = Vec::new();

        loop {
            match (self.read_form(), open) {
                (Ok(t), _) => ret.push(t),
                (Err(ReaderError::_EndRound), "(") => return Ok(ret),
                (Err(ReaderError::_EndSquare), "[") => return Ok(ret),
                _ => return Err(ReaderError::EOFError),
            }
        }
    }

    fn read_atom(&mut self) -> Result<MalType, ReaderError> {
        let token = self.next()?;
        dprintln!("read_atom {:?}", token);

        let number_re = Regex::new(r"^[1-9][0-9]*$").unwrap();
        let string_re = Regex::new(r#"^"(.*)"$"#).unwrap();

        if token == *")" {
            Err(ReaderError::_EndRound) // passed up to terminate read_list
        } else if token == *"]" {
            Err(ReaderError::_EndSquare)

        // } else if token == *"}" {
        // Err(ReaderError::_EndCurly)
        } else if token == *"nil" {
            Ok(MalType::Nil)
        } else if token == *"true" {
            Ok(MalType::Bool(true))
        } else if token == *"false" {
            Ok(MalType::Bool(false))
        } else if number_re.is_match(&token) {
            Ok(MalType::Int(token.parse::<i32>().unwrap()))
        } else if string_re.is_match(&token) {
            let string_capture = string_re.captures(&token).unwrap().get(1).unwrap().as_str();
            Ok(MalType::String(
                string_capture
                    .replace("\\\"", "\"")
                    .replace("\\n", "\n")
                    .replace("\\\\", "\\")
                    .to_string(),
            ))
        } else {
            Ok(MalType::Symbol(token))
        }
    }
}

pub fn read_str(s: String) -> Result<MalType, ReaderError> {
    dprintln!("read_str {}", s);
    let mut r: Reader = Reader::new(tokenize(s))?;
    r.read_form()
}

fn tokenize(s: String) -> Vec<Token> {
    dprintln!("tokenize {}", s);
    let mut tokens: Vec<String> = Vec::new();
    let re = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#)
        .unwrap();
    let comment_re = Regex::new("^;").unwrap();

    // skip the first element
    let captures_iter = re.captures_iter(&s);
    for y in captures_iter {
        let token = y
            .get(1)
            .expect("how did you get here?")
            .as_str()
            .to_string();
        if !comment_re.is_match(&token) {
            tokens.push(token);
        }
    }

    dprintln!("tokenize {:?}", tokens);
    tokens
}

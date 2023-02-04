use crate::types::MalType;

use regex::Regex;
use std::fmt;

type Token = String;

#[derive(Debug, Clone)]
pub struct Reader {
    tokens: Vec<Token>,
    position: usize,
}

#[derive(Debug, Clone)]
pub enum ReaderError {
    EOFError,
}

impl fmt::Display for ReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ReaderError: reached EOF")
    }
}

impl Reader {
    fn new(tokens: Vec<Token>) -> Self {
        Reader {
            tokens,
            position: 0,
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
        // println!("{:?} , {:?}", self.tokens, self.position);
        if self.tokens.len() == self.position {
            Err(ReaderError::EOFError)
        } else {
            Ok(self.tokens[self.position].clone())
        }
    }

    fn read_form(&mut self) -> Result<Option<MalType>, ReaderError> {
        // println!("read_form {:?}", self);
        let peeked: &str = &self.peek()?;
        match peeked {
            "(" => Ok(Some(MalType::List(self.read_list()?))),
            _ => match self.read_atom()? {
                None => Ok(None),
                Some(t) => Ok(Some(t)),
            },
        }
    }

    fn read_list(&mut self) -> Result<Vec<MalType>, ReaderError> {
        // println!("read_list {:?}", self);
        self.next()?;
        let mut ret = Vec::new();
        while let Some(t) = self.read_form()? {
            ret.push(t);
        }
        // println!("{:?}", ret);
        Ok(ret)
    }

    fn read_atom(&mut self) -> Result<Option<MalType>, ReaderError> {
        // println!("read_atom {:?}", self);
        let token = self.next()?;
        // println!("token {token}");

        let number_re = Regex::new(r"^[1-9][0-9]*$").unwrap();

        // TODO: nil, true, false and strings
        if token == *")" {
            Ok(None) // this option is passed up to terminate read_list
        } else if number_re.is_match(&token) {
            Ok(Some(MalType::Int(token.parse::<i32>().unwrap())))
        } else {
            Ok(Some(MalType::Symbol(token)))
        }
    }
}

pub fn read_str(s: String) -> Result<MalType, ReaderError> {
    let mut r = Reader::new(tokenize(s));
    match r.read_form() {
        Ok(t) => Ok(t.unwrap()),
        Err(e) => Err(e),
    }
}

fn tokenize(s: String) -> Vec<Token> {
    let mut tokens: Vec<String> = Vec::new();
    let re = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#)
        .unwrap();

    // skip the first element
    let captures_iter = re.captures_iter(&s);
    for y in captures_iter {
        tokens.push(
            y.get(1)
                .expect("how did you get here?")
                .as_str()
                .to_string(),
        );
    }

    tokens.pop();
    // println!("{:?}", tokens);
    tokens
}

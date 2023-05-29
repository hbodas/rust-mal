use std::collections::HashMap;

use crate::types::MalType;

#[derive(Debug)]
pub enum EnvError {
    NoMatchingEnvironment,
    NoKeyInEnvironment,
}

impl std::error::Error for EnvError {}

impl std::fmt::Display for EnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoMatchingEnvironment => write!(f, "No matching environment"),
            Self::NoKeyInEnvironment => write!(f, "Key not found in matched environment"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Env<'a> {
    pub outer: Option<&'a Env<'a>>,
    pub data: HashMap<String, MalType>,
}

impl<'a> Env<'a> {
    pub fn new(outer: Option<&'a Env>) -> Self {
        Self {
            outer,
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: MalType) {
        self.data.insert(key.to_string(), value);
    }

    pub fn find(&self, key: &str) -> Result<&Self, EnvError> {
        match (self.data.get(key), self.outer) {
            (Some(_), _) => Ok(&self),
            (None, Some(e)) => e.find(key),
            (None, None) => Err(EnvError::NoMatchingEnvironment),
        }
    }

    pub fn get(&self, key: &str) -> Result<MalType, EnvError> {
        let e = self.find(key)?;
        e.data.get(key).ok_or(EnvError::NoKeyInEnvironment).cloned()
    }
}

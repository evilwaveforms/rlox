use crate::{interpreter::Data, scanner::Token};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Environment {
    pub values: HashMap<String, Data>,
}

#[derive(Debug)]
pub enum Error {
    Undefined(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Undefined(ref lexeme) => write!(f, "Undefined variable: '{}'.", lexeme),
        }
    }
}

impl Environment {
    pub fn define(&mut self, name: String, value: Data) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &Token) -> Result<Data, Error> {
        match self.values.get(&name.lexeme) {
            Some(val) => Ok(val.clone()),
            None => Err(Error::Undefined(name.lexeme.clone())),
        }
    }
}

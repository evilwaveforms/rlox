use crate::{interpreter::Data, scanner::Token};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug)]
pub struct Environment {
    pub enclosing: Option<Rc<RefCell<Environment>>>,
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
            Some(val) => return Ok(val.clone()),
            _ => (),
        };

        if self.enclosing.is_some() {
            return match self.enclosing.as_mut().unwrap().borrow_mut().get(&name) {
                Ok(val) => Ok(val.clone()),
                Err(_) => Err(Error::Undefined(name.lexeme.clone())),
            };
        };
        Err(Error::Undefined(name.lexeme.clone()))
    }

    pub fn assign(&mut self, name: &Token, value: &Data) {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            return;
        }

        if self.enclosing.is_some() {
            self.enclosing
                .as_mut()
                .unwrap()
                .borrow_mut()
                .assign(&name, &value);
            return;
        };
        Error::Undefined(name.lexeme.clone());
    }
}

use std::fmt;
use crate::{
    expr::{Binary, Expr, Grouping, Unary},
    scanner::Literal,
    scanner::TokenType,
};

#[derive(Debug, PartialEq)]
pub enum Data {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           Data::Number(num) => write!(f, "{}", num),
           Data::Str(str) => write!(f, "{}", str),
           Data::Bool(bool) => write!(f, "{}", bool),
           Data::Nil => write!(f, "nil"),
       }
    }
}

#[derive(Debug)]
pub enum Error {
    ValueError,
}

pub fn interpret(expr: Expr) {
    let value = evaluate(expr);
    println!("{:?}", value);
}

pub fn evaluate(expr: Expr) -> Result<Data, Error> {
    match expr {
        Expr::Binary(expr) => evaluate_binary(*expr),
        Expr::Grouping(grouping) => evaluate_grouping(*grouping),
        Expr::Literal(lit) => evaluate_literal(lit),
        Expr::Unary(unary) => evaluate_unary(*unary),
    }
}

fn evaluate_binary(expr: Binary) -> Result<Data, Error> {
    let left = evaluate(expr.left)?;
    let right = evaluate(expr.right)?;

    // I know
    match expr.operator.ttype {
        TokenType::Plus => match (left, right) {
            (Data::Number(left), Data::Number(right)) => Ok(Data::Number(left + right)),
            (Data::Str(left), Data::Str(right)) => Ok(Data::Str(left + &right)),
            _ => Err(Error::ValueError),
        },
        TokenType::Minus => match (left, right) {
            (Data::Number(left), Data::Number(right)) => Ok(Data::Number(left - right)),
            _ => Err(Error::ValueError),
        },
        TokenType::Slash => match (left, right) {
            (Data::Number(left), Data::Number(right)) => Ok(Data::Number(left / right)),
            _ => Err(Error::ValueError),
        },
        TokenType::Star => match (left, right) {
            (Data::Number(left), Data::Number(right)) => Ok(Data::Number(left * right)),
            _ => Err(Error::ValueError),
        },
        TokenType::Greater => match (left, right) {
            (Data::Number(left), Data::Number(right)) => Ok(Data::Bool(left > right)),
            _ => Err(Error::ValueError),
        },
        TokenType::GreaterEqual => match (left, right) {
            (Data::Number(left), Data::Number(right)) => Ok(Data::Bool(left >= right)),
            _ => Err(Error::ValueError),
        },
        TokenType::Less => match (left, right) {
            (Data::Number(left), Data::Number(right)) => Ok(Data::Bool(left < right)),
            _ => Err(Error::ValueError),
        },
        TokenType::LessEqual => match (left, right) {
            (Data::Number(left), Data::Number(right)) => Ok(Data::Bool(left <= right)),
            _ => Err(Error::ValueError),
        },
        TokenType::BangEqual => Ok(Data::Bool(!is_equal(left, right))),
        TokenType::EqualEqual => Ok(Data::Bool(is_equal(left, right))),
        _ => Err(Error::ValueError),
    }
}

fn evaluate_literal(literal: Literal) -> Result<Data, Error> {
    match literal {
        Literal::Str(str) => Ok(Data::Str(str)),
        Literal::Number(num) => Ok(Data::Number(num)),
        _ => Err(Error::ValueError),
    }
}

fn evaluate_grouping(grouping: Grouping) -> Result<Data, Error> {
    evaluate(grouping.expression)
}

fn evaluate_unary(unary: Unary) -> Result<Data, Error> {
    let right = evaluate(unary.right)?;

    match unary.operator.ttype {
        TokenType::Minus => match right {
            Data::Number(right) => Ok(Data::Number(-right)),
            _ => Err(Error::ValueError),
        },
        TokenType::Bang => Ok(Data::Bool(is_truthy(right))),
        _ => return Err(Error::ValueError),
    }
}

fn is_truthy(data: Data) -> bool {
    match data {
        Data::Nil => false,
        Data::Bool(val) => val,
        _ => true,
    }
}

fn is_equal(a: Data, b: Data) -> bool {
    if a == Data::Nil && b == Data::Nil { return true }
    if a == Data::Nil { return false }
    a == b
}

fn stringify(data: Data) {}

use crate::{
    expr::{Binary, Expr, Grouping, Unary},
    scanner::Literal,
    scanner::TokenType,
};

pub enum Data {
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
}

#[derive(Debug)]
pub enum Error {
    ValueError,
}

pub fn evaluate(expr: Expr) -> Result<Data, Error> {
    match expr {
        Expr::Binary(_) => todo!(),
        Expr::Grouping(grouping) => evaluate_grouping(*grouping),
        Expr::Literal(lit) => evaluate_literal(lit),
        Expr::Unary(unary) => evaluate_unary(*unary),
    }
}

fn evaluate_binary(expr: Binary) -> Result<Data, Error> {
    let left = evaluate(expr.left)?;
    let right = evaluate(expr.right)?;

    match expr.operator.ttype {
        TokenType::Minus => todo!(),
        TokenType::Slash => todo!(),
        TokenType::Star => todo!(),
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
        TokenType::Bang => {
             Ok(Data::Bool(is_truthy(right)))
        },
        _ => return Err(Error::ValueError),
    }
}

fn is_truthy(data: Data) -> bool {
    match data {
        Data::Nil => false,
        Data::Bool(val) => val,
        _ => true
    }
}

fn stringify(data: Data) {}

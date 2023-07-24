use crate::{
    expr::{Expr, Grouping, Unary},
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
    let right = evaluate(unary.right);

    match unary.operator.ttype {
        TokenType::Minus => todo!(), //Ok(Data::Number(right)),
        _ => return Err(Error::ValueError),
    }
}

fn stringify(data: Data) {}

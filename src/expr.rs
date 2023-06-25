use crate::scanner::Literal;
use crate::scanner::Token;

pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Literal),
    Unary(Box<Unary>),
}

pub struct Binary {
    left: Expr,
    operator: Token,
    right: Expr,
}

pub struct Grouping {
    expression: Expr,
}

pub struct Unary {
    operator: Token,
    right: Expr,
}

use crate::scanner::Literal;
use crate::scanner::Token;

pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Literal),
    Unary(Box<Unary>),
}

pub struct Binary {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

pub struct Grouping {
    pub expression: Expr,
}

pub struct Unary {
    pub operator: Token,
    pub right: Expr,
}

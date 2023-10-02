use crate::scanner::Literal;
use crate::scanner::Token;

#[derive(Clone, Debug)]
pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Literal),
    Unary(Box<Unary>),
    Variable(Box<Variable>)
}

#[derive(Clone, Debug)]
pub struct Binary {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

#[derive(Clone, Debug)]
pub struct Grouping {
    pub expression: Expr,
}

#[derive(Clone, Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Expr,
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: Token,
}

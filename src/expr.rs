use crate::scanner::Literal;
use crate::scanner::Token;

pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Literal),
    Unary(Box<Unary>),
}

pub trait Visitor {
    fn visit_binary_expr(&mut self);
    fn visit_grouping_expr(&mut self);
    fn visit_literal_expr(&mut self);
    fn visit_unary_expr(&mut self);
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

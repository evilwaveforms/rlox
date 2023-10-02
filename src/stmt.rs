use crate::{expr::Expr, scanner::Token};

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression(Expression),
    Print(Print),
    Var(Var),
}

#[derive(Clone, Debug)]
pub struct Expression {
    pub expression: Expr,
}

#[derive(Clone, Debug)]
pub struct Print {
    pub expression: Expr,
}

#[derive(Clone, Debug)]
pub struct Var {
    pub name: Token,
    pub initializer: Option<Expr>,
}

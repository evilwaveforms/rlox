use crate::{expr::Expr, scanner::Token};

#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Block),
    Expression(Expression),
    Print(Print),
    Var(Var),
    If(If),
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

#[derive(Clone, Debug)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct If {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

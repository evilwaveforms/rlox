use crate::{expr::Expr, scanner::Token};

#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Block),
    Expression(Expression),
    Print(Print),
    Var(Var),
    If(If),
    While(Box<While>),
    Function(Box<Function>),
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

#[derive(Clone, Debug)]
pub struct While {
    pub condition: Expr,
    pub body: Stmt,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Vec<Stmt>,
}

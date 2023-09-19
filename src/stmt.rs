use crate::expr::Expr;

pub enum Stmt {
    Expression(Expression),
    Print(Print),
}

pub struct Expression {
    pub expression: Expr,
}

pub struct Print {
    pub expression: Expr,
}

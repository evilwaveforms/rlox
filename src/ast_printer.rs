use crate::expr::*;
use crate::scanner::Literal;

impl Expr {
    pub fn print(&self) -> String {
        match self {
            Expr::Binary(expr) => parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right]),
            Expr::Grouping(expr) => parenthesize(&String::from("group"), &[&expr.expression]),
            Expr::Literal(lit) => match lit {
                Literal::Str(str) => {
                    return str.to_string();
                }
                Literal::Number(num) => {
                    return num.to_string();
                }
                Literal::Identifier(identifier) => {
                    return identifier.to_string();
                }
            },
            Expr::Unary(expr) => parenthesize(&expr.operator.lexeme, &[&expr.right]),
        }
    }
}

fn parenthesize(name: &String, exprs: &[&Expr]) -> String {
    let mut parenthesized: String = String::from("");

    parenthesized.push_str("(");
    parenthesized.push_str(name);
    for expr in exprs {
        parenthesized.push_str(" ");
        parenthesized.push_str(&expr.print());
    }
    parenthesized.push_str(")");
    return parenthesized;
}

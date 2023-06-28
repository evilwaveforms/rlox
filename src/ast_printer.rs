use crate::expr::*;

// TODO: Try visitor pattern if makes sense, fix this mess
impl Expr {
    pub fn print(&self) -> String {
        match self {
            Expr::Binary(expr) => parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right]),
            Expr::Grouping(expr) => parenthesize(&String::from("group"), &[&expr.expression]),
            Expr::Literal(lit) => {
                return String::from("666");
                // return lit.to_string()
            }
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

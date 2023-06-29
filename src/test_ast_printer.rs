#[cfg(test)]
use super::*;

#[test]
fn test_expr_printing() {
    let minus = scanner::Token {
        lexeme: String::from("-"),
        line: 0,
        literal: Some(scanner::Literal::Str(String::from(""))),
        ttype: scanner::TokenType::Minus,
    };
    let num1 = scanner::Literal::Number(666.666);
    let unary = expr::Unary {
        operator: minus,
        right: expr::Expr::Literal(num1),
    };
    let star = scanner::Token {
        lexeme: String::from("*"),
        line: 1,
        literal: Some(scanner::Literal::Str(String::from(""))),
        ttype: scanner::TokenType::Star,
    };
    let num2 = scanner::Literal::Number(45.67);
    let grouping = expr::Grouping {
        expression: expr::Expr::Literal(num2),
    };
    let binary = expr::Binary {
        left: expr::Expr::Unary(Box::new(unary)),
        operator: star,
        right: expr::Expr::Grouping(Box::new(grouping)),
    };
    let expr = expr::Expr::Binary(Box::new(binary));
    print!("{}", expr.print());
    assert_eq!(expr.print(), "(* (- 666.666) (group 45.67))")
}

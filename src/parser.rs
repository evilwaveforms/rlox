use crate::expr::Binary;
use crate::expr::Expr;
use crate::expr::Grouping;
use crate::expr::Unary;
use crate::scanner;
use crate::scanner::Literal;
use crate::scanner::Token;
use crate::scanner::TokenType;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn expression(&self) -> Expr {
        return self.equality();
    }

    fn equality(&self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.matching(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op: Token = self.previous();
            let r: Expr = self.comparison();
            expr = Expr::Binary(Box::new(Binary{left: expr, operator: op, right: r}));
        }
        return expr;
    }

    fn comparison(&self) -> Expr {
        let mut expr: Expr = self.term();

        while self.matching(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let op: Token = self.previous();
            let r: Expr = self.term();
            expr = Expr::Binary(Box::new(Binary{left: expr, operator: op, right: r}));
        }
        return expr;
    }

    fn term(&self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.matching(&[TokenType::Minus, TokenType::Plus]) {
            let op: Token = self.previous();
            let r: Expr = self.factor();
            expr = Expr::Binary(Box::new(Binary{left: expr, operator: op, right: r}));
        }
        return expr;
    }

    fn factor(&self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.matching(&[TokenType::Slash, TokenType::Star]) {
            let op: Token = self.previous();
            let r: Expr = self.unary();
            expr = Expr::Binary(Box::new(Binary{left: expr, operator: op, right: r}));
        }
        return expr;
    }

    fn unary(&self) -> Expr {
        if self.matching(&[TokenType::Bang, TokenType::Minus]) {
            let op: Token = self.previous();
            let r: Expr = self.unary();
            return Expr::Unary(Box::new(Unary{operator: op, right: r}));
        }
        return self.primary();
    }

    fn primary(&self) -> Expr {
        if self.matching(&[TokenType::False]) { return Expr::Literal(Literal::Str(String::from("false"))); }
        if self.matching(&[TokenType::True]) { return Expr::Literal(Literal::Str(String::from("true"))); }
        if self.matching(&[TokenType::Nil]) { return Expr::Literal(Literal::Str(String::from("null"))); }
        if self.matching(&[TokenType::Number, TokenType::String]) {
            return Expr::Literal(self.previous().literal.unwrap());
        }

        if self.matching(&[TokenType::LeftParen]) {
            let expr: Expr = self.expression();
            self.consume(&TokenType::RightParen, "Except ')' after expression.");
            return Expr::Grouping(Box::new(Grouping{expression: expr}));
        }
    }

    fn matching(&self, ttypes: &[TokenType]) -> bool {
        for ttype in ttypes {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn consume(&self, ttype: &TokenType, message: &str) {
        // TODO
        if self.check(ttype) { return self.advance(); }
        return self.error(self.peek(), message);
        
    }

    fn check(&self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        };
        return &self.peek().ttype == ttype;
    }

    fn advance(&self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        };
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().ttype == TokenType::Eof;
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current];
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1];
    }

    fn error(&self, token: Token, message: &str) {
        scanner::error(token, &message); 
    }
}

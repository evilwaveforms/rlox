use crate::expr::Binary;
use crate::expr::Expr;
use crate::expr::Grouping;
use crate::expr::Unary;
use crate::scanner;
use crate::scanner::Literal;
use crate::scanner::Token;
use crate::scanner::TokenType;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

#[derive(Debug)]
pub enum Error {
    ParseError,
}

impl Parser {
    fn expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = match self.comparison() {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };

        while self.matching(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op: Token = self.previous();
            let r: Expr = match self.comparison() {
                Ok(r) => r,
                Err(e) => return Err(e),
            };
            expr = Expr::Binary(Box::new(Binary {
                left: expr,
                operator: op,
                right: r,
            }));
        }
        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = match self.term() {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };

        while self.matching(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op: Token = self.previous();
            let r: Expr = match self.term() {
                Ok(expr) => expr,
                Err(e) => return Err(e),
            };
            expr = Expr::Binary(Box::new(Binary {
                left: expr,
                operator: op,
                right: r,
            }));
        }
        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = match self.factor() {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };

        while self.matching(&[TokenType::Minus, TokenType::Plus]) {
            let op: Token = self.previous();
            let r: Expr = match self.factor() {
                Ok(r) => r,
                Err(e) => return Err(e),
            };
            expr = Expr::Binary(Box::new(Binary {
                left: expr,
                operator: op,
                right: r,
            }));
        }
        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let expr: Expr = match self.unary() {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };

        while self.matching(&[TokenType::Slash, TokenType::Star]) {
            let op: Token = self.previous();
            match self.unary() {
                Ok(r) => {
                    return Ok(Expr::Binary(Box::new(Binary {
                        left: expr,
                        operator: op,
                        right: r,
                    })))
                }
                Err(e) => return Err(e),
            };
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.matching(&[TokenType::Bang, TokenType::Minus]) {
            let op: Token = self.previous();
            match self.unary() {
                Ok(r) => {
                    return Ok(Expr::Unary(Box::new(Unary {
                        operator: op,
                        right: r,
                    })))
                }
                Err(e) => return Err(e),
            };
        }
        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.matching(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal::Str(String::from("false"))));
        }
        if self.matching(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::Str(String::from("true"))));
        }
        if self.matching(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Str(String::from("null"))));
        }
        if self.matching(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(self.previous().literal.unwrap()));
        }

        if self.matching(&[TokenType::LeftParen]) {
            match self.expression() {
                Ok(expr) => {
                    self.consume(&TokenType::RightParen, "Except ')' after expression.");
                    return Ok(Expr::Grouping(Box::new(Grouping { expression: expr })));
                }
                Err(e) => return Err(Error::ParseError),
            };
        }
        Err(self.error(self.peek(), "Expect expression."))
    }

    fn matching(&mut self, ttypes: &[TokenType]) -> bool {
        for ttype in ttypes {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn consume(&mut self, ttype: &TokenType, message: &str) -> Result<Token, Error> {
        if self.check(ttype) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek(), message))
    }

    fn check(&self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        };
        &self.peek().ttype == ttype
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        };
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn error(&self, token: Token, message: &str) -> Error {
        scanner::error(token, &message);
        Error::ParseError
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().ttype == TokenType::Semicolon {
                return;
            }

            match self.peek().ttype {
                TokenType::Class => return,
                TokenType::Fun => return,
                TokenType::Var => return,
                TokenType::For => return,
                TokenType::If => return,
                TokenType::While => return,
                TokenType::Print => return,
                TokenType::Return => return,
                _ => (),
            }
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<Expr, Error> {
        match self.expression() {
            Ok(expr) => return Ok(expr),
            Err(e) => return Err(e),
        };
    }
}

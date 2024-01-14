use crate::expr::Assign;
use crate::expr::Binary;
use crate::expr::Call;
use crate::expr::Expr;
use crate::expr::Grouping;
use crate::expr::Logical;
use crate::expr::Unary;
use crate::expr::Variable;
use crate::scanner;
use crate::scanner::Literal;
use crate::scanner::Token;
use crate::scanner::TokenType;
use crate::stmt::Block;
use crate::stmt::Expression;
use crate::stmt::If;
use crate::stmt::Print;
use crate::stmt::Stmt;
use crate::stmt::Var;
use crate::stmt::While;
use crate::stmt::Function;

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
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Stmt, Error> {
        if self.matching(&[TokenType::Fun]) {
            return Ok(self.function("function".to_string()))?;
        }
        if self.matching(&[TokenType::Var]) {
            return Ok(self.var_declaration())?;
        }
        return self.statement();
        // TODO: check that synchronize works
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
        if self.matching(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.matching(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.matching(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.matching(&[TokenType::While]) {
            return self.while_statement();
        }

        if self.matching(&[TokenType::LeftBrace]) {
            let statements = self.block();
            return Ok(Stmt::Block(Block { statements }));
        }
        self.expression_statement()
    }

    fn for_statement(&mut self) -> Result<Stmt, Error> {
        // TODO: Make this nice
        self.consume(&TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let mut initializer = None;
        if self.matching(&[TokenType::Semicolon]) {
            initializer = None;
        } else if self.matching(&[TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition = None;
        if !self.check(&TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(&TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let mut increment = None;
        if !self.check(&TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(&TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if increment.is_some() {
            let b = vec![
                body,
                Stmt::Expression(Expression {
                    expression: increment.unwrap(),
                }),
            ];
            body = Stmt::Block(Block { statements: b });
        }
        if condition.is_none() {
            condition = Some(Expr::Literal(Literal::Str(String::from("true"))));
        }
        body = Stmt::While(Box::new(While {
            condition: condition.unwrap(),
            body,
        }));

        let mut s = vec![];
        if initializer.is_some() {
            s.push(initializer.unwrap());
            s.push(body);
        }

        body = Stmt::Block(Block { statements: s });
        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        let mut else_branch = None;

        if self.matching(&[TokenType::Else]) {
            let stmt = self.statement()?;
            else_branch = Some(Box::new(stmt));
        }
        Ok(Stmt::If(If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        }))
    }

    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let expr: Expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(Print { expression: expr }))
    }

    fn var_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self
            .consume(&TokenType::Identifier, "Expect variable name.")
            .unwrap();

        let mut initializer: Option<Expr> = None;
        if self.matching(&[TokenType::Equal]) {
            match self.expression() {
                Ok(init) => initializer = Some(init),
                _ => (),
            };
        }

        self.consume(
            &TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var(Var { name, initializer }))
    }

    fn while_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;
        Ok(Stmt::While(Box::new(While { condition, body })))
    }

    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let expr: Expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Expression(Expression { expression: expr }))
    }

    fn function(&mut self, kind: String) -> Result<Stmt, Error> {
        let name = self.consume(&TokenType::Identifier, &("Expect ".to_string() + &kind + " name."))?;
        let mut parameters: Vec<Token> = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    self.error(self.peek()?, "Can't have more than 255 parameters.");
                }
                parameters.push(self.consume(&TokenType::Identifier, "Expect parameter name.")?);
                if self.matching(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(&TokenType::RightParen, "Expect ')' after parameters.");
        self.consume(&TokenType::LeftBrace, &("Expect '{' before ".to_string() + &kind + " body."));
        let body: Vec<Stmt> = self.block();
        Ok(Stmt::Function(Box::new(Function { name, parameters, body })))
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration().unwrap());
        }
        self.consume(&TokenType::RightBrace, "Expect '}' after block.");
        statements
    }

    fn assignment(&mut self) -> Result<Expr, Error> {
        let expr: Expr = self.or()?;

        if self.matching(&[TokenType::Equal]) {
            let equals: Token = self.previous();
            let val: Expr = self.assignment()?;

            if let Expr::Variable(var) = expr {
                let name: Token = var.name;
                return Ok(Expr::Assign(Box::new(Assign { name, value: val })));
            }
            return Err(self.error(equals, "Invalid Assignment target."));
        }
        return Ok(expr);
    }

    fn or(&mut self) -> Result<Expr, Error> {
        let mut expr = self.and()?;

        while self.matching(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(Logical {
                left: expr,
                operator,
                right,
            }));
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, Error> {
        let mut expr = self.equality()?;

        while self.matching(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(Logical {
                left: expr,
                operator,
                right,
            }));
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = self.comparison()?;

        while self.matching(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let op: Token = self.previous();
            let r: Expr = self.comparison()?;
            expr = Expr::Binary(Box::new(Binary {
                left: expr,
                operator: op,
                right: r,
            }));
        }
        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = self.term()?;

        while self.matching(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let op: Token = self.previous();
            let r: Expr = self.term()?;
            expr = Expr::Binary(Box::new(Binary {
                left: expr,
                operator: op,
                right: r,
            }));
        }
        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr: Expr = self.factor()?;

        while self.matching(&[TokenType::Minus, TokenType::Plus]) {
            let op: Token = self.previous();
            let r: Expr = self.factor()?;

            expr = Expr::Binary(Box::new(Binary {
                left: expr,
                operator: op,
                right: r,
            }));
        }
        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let expr: Expr = self.unary()?;

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
        return self.call();
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, Error> {
        let mut arguments: Vec<Expr> = Vec::new();

        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    self.error(self.peek()?, "Can't have more than 255 arguments.");
                }

                arguments.push(self.expression()?);
                if self.matching(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(&TokenType::RightParen, "Expect ')' after arguments.")?;
        Ok(Expr::Call(Box::new(Call {
            callee,
            paren,
            arguments,
        })))
    }

    fn call(&mut self) -> Result<Expr, Error> {
        let mut expr = self.primary()?;

        loop {
            if self.matching(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
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
        if self.matching(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(Box::new(Variable {
                name: self.previous(),
            })));
        }

        if self.matching(&[TokenType::LeftParen]) {
            match self.expression() {
                Ok(expr) => {
                    self.consume(&TokenType::RightParen, "Except ')' after expression.")?;
                    return Ok(Expr::Grouping(Box::new(Grouping { expression: expr })));
                }
                Err(e) => return Err(e),
            };
        }
        match self.peek() {
            Ok(token) => Err(self.error(token, "Expect expression.")),
            Err(e) => Err(e),
        }
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
        match self.peek() {
            Ok(token) => Err(self.error(token, message)),
            Err(e) => Err(e),
        }
    }

    fn check(&self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        };
        match self.peek() {
            Ok(token) => &token.ttype == ttype,
            Err(_) => true,
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        };
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        if self.current >= self.tokens.len() {
            true;
        }
        match self.peek() {
            Ok(token) => token.ttype == TokenType::Eof,
            Err(e) => true,
        }
    }

    fn peek(&self) -> Result<Token, Error> {
        match self.tokens.get(self.current) {
            Some(token) => return Ok(token.clone()),
            None => Err(Error::ParseError),
        }
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn error(&mut self, token: Token, message: &str) -> Error {
        scanner::error(token, &message);
        self.synchronize();
        Error::ParseError
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().ttype == TokenType::Semicolon {
                return;
            }
            let token = match self.peek() {
                Ok(token) => token,
                Err(e) => return,
            };

            match token.ttype {
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }
}

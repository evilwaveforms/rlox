use std::fmt;
use std::str;

#[rustfmt::skip]
#[derive(Debug)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    // Literals.
    Identifier, String, Number,
    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof,
}

#[derive(Debug)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
}

#[derive(Debug)]
pub struct Token {
    pub lexeme: String,
    line: usize,
    literal: Option<Literal>,
    ttype: TokenType,
}

pub struct Scanner {
    pub source: Vec<u8>,
    pub list: Vec<Token>,
    pub current: usize,
    pub start: usize,
    pub line: usize,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.ttype,
            self.lexeme,
            self.literal.as_ref().unwrap().to_string()
        )
    }
}

impl Scanner {
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        char::from(self.source[self.current - 1])
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        };
        if char::from(self.source[self.current]) != expected {
            return false;
        };
        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        };
        char::from(self.source[self.current])
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return char::from(self.source[self.current + 1]);
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            error(self.line, "Unterminated string.");
            return;
        }
        self.advance();

        let text = &self.source[self.start + 1..self.current - 1];
        let value = str::from_utf8(text).unwrap().to_string();
        self.make_token(TokenType::String, Literal::Str(value));
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let text = &self.source[self.start..self.current];
        let value = Literal::Number(
            str::from_utf8(text)
                .unwrap()
                .to_string()
                .parse::<f64>()
                .unwrap(),
        );
        self.make_token(TokenType::Number, value)
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => match self.match_token('=') {
                true => self.add_token(TokenType::BangEqual),
                false => self.add_token(TokenType::Bang),
            },
            '=' => match self.match_token('=') {
                true => self.add_token(TokenType::EqualEqual),
                false => self.add_token(TokenType::Equal),
            },
            '<' => match self.match_token('=') {
                true => self.add_token(TokenType::LessEqual),
                false => self.add_token(TokenType::Less),
            },
            '>' => match self.match_token('=') {
                true => self.add_token(TokenType::GreaterEqual),
                false => self.add_token(TokenType::Greater),
            },
            '/' => match self.match_token('/') {
                true => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                false => self.add_token(TokenType::Slash),
            },
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            c => {
                if is_digit(c) {
                    self.number();
                } else {
                    error(self.line, "Unexpected character.");
                }
            }
        };
    }

    fn make_token(&mut self, ttype: TokenType, literal: Literal) {
        let text = &self.source[self.start..self.current];
        let t = Token {
            lexeme: str::from_utf8(text).unwrap().to_string(),
            line: self.line,
            literal: Some(literal),
            ttype,
        };
        println!("Lexeme: {:?}", t.lexeme);
        println!("Token type: {:?}", t.ttype);
        println!("Literal: {:?}", t.literal);
        return self.list.push(t);
    }

    fn add_token(&mut self, ttype: TokenType) {
        let l = Literal::Str(String::from("test"));
        self.make_token(ttype, l);
    }
}

fn error(line: usize, message: &str) {
    report(line, "".to_string(), message);
}

fn report(line: usize, position: String, message: &str) {
    println!("[line: {}] Error: {} : {}", line, position, message);
}

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

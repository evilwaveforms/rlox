use std::fmt;
use std::str;

#[rustfmt::skip]
#[derive(Debug, PartialEq, Copy, Clone, strum_macros::Display)]
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

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub lexeme: String,
    pub line: usize,
    pub literal: Option<Literal>,
    pub ttype: TokenType,
}

pub struct Scanner {
    pub source: Vec<u8>,
    pub list: Vec<Token>,
    pub current: usize,
    pub start: usize,
    pub line: usize,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Identifier(id) => write!(f, "{}", id),
            Literal::Str(str) => write!(f, "{}", str),
            Literal::Number(num) => write!(f, "{}", num),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Line: {}, Token type: {}, Lexeme: {} Literal: {}",
            self.line,
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
            report(self.line, "", "Unterminated string.");
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

    fn identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let text_str = str::from_utf8(text).unwrap().to_string();
        self.keyword_or_identifier(&text_str);
    }

    fn keyword_or_identifier(&mut self, s: &str) {
        match s {
            "and" => self.add_token(TokenType::And, Some(s)),
            "class" => self.add_token(TokenType::Class, Some(s)),
            "else" => self.add_token(TokenType::Else, Some(s)),
            "false" => self.add_token(TokenType::False, Some(s)),
            "for" => self.add_token(TokenType::For, Some(s)),
            "fun" => self.add_token(TokenType::Fun, Some(s)),
            "if" => self.add_token(TokenType::If, Some(s)),
            "nil" => self.add_token(TokenType::Nil, Some(s)),
            "or" => self.add_token(TokenType::Or, Some(s)),
            "print" => self.add_token(TokenType::Print, Some(s)),
            "return" => self.add_token(TokenType::Return, Some(s)),
            "super" => self.add_token(TokenType::Super, Some(s)),
            "this" => self.add_token(TokenType::This, Some(s)),
            "true" => self.add_token(TokenType::True, Some(s)),
            "var" => self.add_token(TokenType::Var, Some(s)),
            "while" => self.add_token(TokenType::While, Some(s)),
            _ => {
                self.add_token(TokenType::Identifier, Some(s));
            }
        }
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => match self.match_token('=') {
                true => self.add_token(TokenType::BangEqual, None),
                false => self.add_token(TokenType::Bang, None),
            },
            '=' => match self.match_token('=') {
                true => self.add_token(TokenType::EqualEqual, None),
                false => self.add_token(TokenType::Equal, None),
            },
            '<' => match self.match_token('=') {
                true => self.add_token(TokenType::LessEqual, None),
                false => self.add_token(TokenType::Less, None),
            },
            '>' => match self.match_token('=') {
                true => self.add_token(TokenType::GreaterEqual, None),
                false => self.add_token(TokenType::Greater, None),
            },
            '/' => match self.match_token('/') {
                true => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                false => self.add_token(TokenType::Slash, None),
            },
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            c => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    report(self.line, "", "Unexpected character.");
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
        println!("Token {:?}", t);
        return self.list.push(t);
    }

    fn add_token(&mut self, ttype: TokenType, s: Option<&str>) {
        let l = Literal::Str(String::from(s.unwrap_or("")));
        self.make_token(ttype, l);
    }
}

pub fn error(token: Token, message: &str) {
    if token.ttype == TokenType::Eof {
        report(token.line, " at end", message);
    }
    report(token.line, &format!(" at ' {} '", token.lexeme), &message);
}

fn report(line: usize, position: &str, message: &str) {
    println!("[line: {}] Error: {} : {}", line, position, message);
}

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

fn is_alpha(c: char) -> bool {
    return c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_';
}

fn is_alphanumeric(c: char) -> bool {
    return is_alpha(c) || is_digit(c);
}

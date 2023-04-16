use std::fmt;
use std::str;

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

pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
}

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
        write!(f, "{}", self)
    }

}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.ttype, self.lexeme, self.literal.as_ref().unwrap().to_string())
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
        self.current+=1;
        char::from(self.source[self.current - 1])
    }

    fn scan_token(&mut self) {
        let c : char = self.advance();
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
            _ => error(self.line, "Unexpected character."),
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
         println!("{} is the lexeme", t.lexeme);
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

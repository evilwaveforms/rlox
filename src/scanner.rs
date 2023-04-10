use std::fmt;

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
    lexeme: String,
    line: usize,
    literal: Option<Literal>,
    ttype: TokenType,
}

pub struct Scanner {
    pub source: Vec<u8>,
    pub list: Vec<Token>,
    pub current: usize,
    pub start: usize,
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
    pub fn scan_tokens(&self) {
        // for token in tokens.iter() {
        while self.current < self.source.len() {

        }
    }

    fn is_at_end(&self) -> bool {
        true
    }

    fn advance(&mut self) -> char{
        self.current+=1;
        // https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings
        // return self.source.chars().nth(self.current - 1).unwrap();
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
            _ => unreachable!()
        };
    }

     fn make_token(&self, ttype: TokenType, literal: u8) {
     }
    
    fn add_token(&self, ttype: TokenType) {
        self.make_token(ttype, 0);
    }

    }

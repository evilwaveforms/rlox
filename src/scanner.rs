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

pub struct Token {
    lexeme: String,
    line: usize,
    literal: String,
    ttype: TokenType,
}

pub struct Scanner<'a> {
    pub source: &'a String,
    pub list: Vec<Token>,
    pub current: usize,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }

}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.ttype, self.lexeme, self.literal)
    }
}


impl Scanner<'_> {
    pub fn scan_tokens(&self) {
        // for token in tokens.iter() {
        while self.current < self.source.chars().count() {

        }
    }

    fn is_at_end(&self) -> bool {
        return true;
    }

    fn advance(&mut self) -> char{
        self.current+=1;
        // NOTE: Refactor, horrible.. horrible..
        // https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings
        return self.source.chars().nth(self.current - 1).unwrap();
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

    // fn make_token(&self, ttype: TokenType, literal: String) {
    // }
    //
    fn add_token(&self, ttype: TokenType) {
        // let literal = String::from("");
        // self.make_token(ttype, literal);
    }

    }

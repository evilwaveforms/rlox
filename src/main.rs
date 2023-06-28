use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
mod ast_printer;
mod expr;
pub mod scanner;

fn main() {
    test_expr();

    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        print!("Usage: rlox [script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
    dbg!(args);
}

fn run_file(path: &String) {
    let file = File::open(&path).expect("Unable to open file");
    let lines = io::BufReader::new(&file).lines();
    for (i, line) in lines.enumerate() {
        run(i + 1, line.unwrap().into_bytes());
    }
}

fn run_prompt() {
    loop {
        let mut line = String::new();
        print!(">");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut line)
            .expect("Error: Could not read a line");
        line = line.trim().to_string();
        if line.is_empty() {
            break;
        }
        run(0, line.into_bytes());
    }
}

fn run(idx: usize, source: Vec<u8>) {
    let v: Vec<scanner::Token> = vec![];
    let mut scanner = scanner::Scanner {
        source,
        list: v,
        current: 0,
        start: 0,
        line: idx,
    };
    scanner.scan_tokens();
}

fn test_expr() {
    let minus = scanner::Token {
        lexeme: String::from("-"),
        line: 0,
        literal: Some(scanner::Literal::Str(String::from(""))),
        ttype: scanner::TokenType::Minus,
    };
    let num1 = scanner::Literal::Number(123.0);
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
}

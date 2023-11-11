use environment::Environment;
use interpreter::Data;
use std::collections::HashMap;
use std::env;
use std::io::Write;

mod ast_printer;
mod environment;
mod expr;
mod interpreter;
mod parser;
pub mod scanner;
mod stmt;
mod test_ast_printer;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();

    let values: HashMap<String, Data> = HashMap::new();
    let env = Environment {
        values,
        enclosing: None,
    };
    let mut interpreter = interpreter::Interpreter {
        env,
        repl: false,
    };

    if args.len() > 2 {
        print!("Usage: rlox [script]");
    } else if args.len() == 2 {
        run_file(&args[1], &mut interpreter);
    } else {
        run_prompt(&mut interpreter);
    }
    dbg!(args);
}

fn run_file(path: &String, interpreter: &mut interpreter::Interpreter) {
    let source = std::fs::read(&path).expect("Unable to open file");
    let v: Vec<scanner::Token> = vec![];
    let mut scanner = scanner::Scanner {
        source,
        list: v,
        current: 0,
        start: 0,
        line: 0,
    };
    scanner.scan_tokens();
    let mut parser = parser::Parser {
        tokens: scanner.list,
        current: 0,
    };

    match parser.parse() {
        Ok(stmt) => {
            interpreter.interpret(stmt);
        }
        Err(e) => println!("{:?}", e),
    };
}

fn run_prompt(interpreter: &mut interpreter::Interpreter) {
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
        run(0, line.into_bytes(), true, interpreter);
    }
}

fn run(idx: usize, source: Vec<u8>, repl: bool, interpreter: &mut interpreter::Interpreter) {
    let v: Vec<scanner::Token> = vec![];
    let mut scanner = scanner::Scanner {
        source,
        list: v,
        current: 0,
        start: 0,
        line: idx,
    };
    scanner.scan_tokens();
    let mut parser = parser::Parser {
        tokens: scanner.list,
        current: 0,
    };

    match parser.parse() {
        Ok(stmt) => {
            interpreter.interpret(stmt);
        }
        Err(e) => println!("{:?}", e),
    };
}

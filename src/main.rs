use environment::Environment;
use interpreter::Data;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

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
        env: env.clone(),
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
    let file = File::open(&path).expect("Unable to open file");
    let lines = io::BufReader::new(&file).lines();
    for (i, line) in lines.enumerate() {
        run(i + 1, line.unwrap().into_bytes(), false, interpreter);
    }
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

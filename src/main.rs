use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
pub mod scanner;


fn main() {
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
        run(i+1, line.unwrap().into_bytes());
    }
}

fn run_prompt() {
    loop {
        let mut line = String::new();
        print!(">");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
        line = line.trim().to_string();
        if line.is_empty() { break; }
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



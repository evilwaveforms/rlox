use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::str;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args.len());
    if args.len() > 2 {
        print!("Usage: rlox [script]");
    } else if args.len() == 2 {
        run_file(&args[0]);
    } else {
        run_prompt();
    }
    dbg!(args);
}

fn run_file(path: &String) {
    let mut file_content = Vec::new();
    let mut file = File::open(&path).expect("Unable to open file");
    file.read_to_end(&mut file_content).expect("Unable to read");
    run(&file_content);
}

fn run_prompt() {
    loop {
        let mut line = String::new();
        print!(">");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
        line = line.trim().to_string();
        if line.is_empty() { break; }
        run(&line.as_bytes().to_vec());
    }
}

fn run(source: &Vec<u8>) {
    // let scanner = Scanner { source };
    let s = match str::from_utf8(source) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("result: {}\n", s);
}

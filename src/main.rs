mod interpreter;
mod lexer;

use interpreter::interpret;
use lexer::parse;
use std::fs::read_to_string;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let source = read_to_string(path).expect("Could not read file");
        let tokens = parse(source);
        interpret(tokens);
    } else {
        println!("Usage: cargo run <file.bf>");
        std::process::exit(1);
    }
}

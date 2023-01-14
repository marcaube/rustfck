use rustfck::interpreter::Interpreter;
use rustfck::lexer::Lexer;
use std::{env, fs};

fn main() {
    let filename = env::args().nth(1).unwrap();
    let program = fs::read(filename).expect("Please provide a source file to execute!");

    let tokens = Lexer::new(program).tokenize();

    Interpreter::new(tokens).eval();
}

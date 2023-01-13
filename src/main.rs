use rustfck::interpreter::Interpreter;
use std::{env, fs};

fn main() {
    let program =
        fs::read(env::args().nth(1).unwrap()).expect("Please provide a source file to execute!");

    let mut interpreter = Interpreter::new(program);
    interpreter.run();
}

use std::{env::args, io::Read};

use potigol_interpreter::{interpreter::walker::Interpreter, lexer::tokenize, parser};

fn main() {
    let args = args();
    let mut filepath = None;

    for (i, arg) in args.enumerate() {
        if i > 1 {
            panic!("Expected only one argument");
        }
        if i == 1 {
            filepath = Some(arg);
        }
    }
    let filepath = filepath.expect("Expected filename to interpret");

    let mut file = std::fs::File::open(filepath).expect("File doesn't exist");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read from file");

    let tokens = tokenize(&contents).unwrap();

    let ast = parser::Parser::new(tokens.clone()).parse().unwrap();

    let mut interp = Interpreter::new();

    interp.interpret(ast);
}
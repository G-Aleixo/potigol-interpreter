use potigol_interpreter::{interpreter::walker::Interpreter, lexer::tokenize, parser};

fn main() {
    let tokens = tokenize(include_str!("../../test.poti")).unwrap();

    println!("{tokens:#?}");
    println!("{:#?}", parser::Parser::new(tokens.clone()).parse().unwrap());

    let mut interp = Interpreter::new();

    println!("{:?}", interp.interpret_single(&parser::Parser::new(tokens).parse().unwrap()[0]))
}
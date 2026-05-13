use potigol_interpreter::{lexer::tokenize, parser};


fn main() {
    let res = tokenize("var x := 1.5\n").unwrap();
    println!("{res:#?}");

    let mut parser = parser::Parser::new(res);

    let result = parser.parse().unwrap();

    println!("{result:#?}");
}
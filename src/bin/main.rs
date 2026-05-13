use potigol_interpreter::{lexer::tokenize, parser};


fn main() {
    let res = tokenize("1.5").unwrap();
    println!("{res:#?}");

    let mut parser = parser::Parser::new(res);

    let result = parser.parse().unwrap();

    println!("{result:#?}");

    let tokens = tokenize("var x := 2 * 2 + 3 ^ 7 > 2").unwrap();

    println!("{tokens:?}");
    println!("{:?}", parser::Parser::new(tokens).parse().unwrap());
}
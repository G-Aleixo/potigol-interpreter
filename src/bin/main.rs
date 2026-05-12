use potigol_interpreter::lexer::tokenize;


fn main() {
    let res = tokenize("var x := 1.5\nimprima \"o valor de x é {x}\" ' rwqrds").unwrap();
    println!("{res:#?}");
}
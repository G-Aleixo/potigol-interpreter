use potigol_interpreter::lexer::lexer::tokenize;


fn main() {
    let res = tokenize("imprima \"hello world!\"\n imprima \"dnv\"\n imprima 1.33").unwrap();
    println!("{res:#?}");
    println!("hhsda");
}
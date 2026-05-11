#[derive(Debug)]
pub enum Token {
    Identifier(Identifier),
    Type(Type),
    String(String),
    Character(char),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    NewLine,
    Comma,
    Period,
    Unknown(char),
    Keyword(Keyword),
    Operation(Operation),
    BlockDelimeter(BlockDelimeter),
}

#[derive(Debug)]
pub struct Identifier {
    pub symbol: String
}

#[derive(Debug)]
pub struct Type {
    pub symbol: String
}

#[derive(Debug)]
pub struct Keyword {
    pub keyword: String
}

#[derive(Debug)]
pub struct Operation {
    pub operation: String
}

#[derive(Debug)]
pub struct BlockDelimeter {
    pub delimeter: String,
    pub is_close: bool
}
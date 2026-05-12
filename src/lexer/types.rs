#[derive(Debug, PartialEq)]
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
    Colon,
    Unknown(char),
    Keyword(Keyword),
    Operation(Operation),
    BlockDelimeter(BlockDelimeter),
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub symbol: String
}

#[derive(Debug, PartialEq)]
pub struct Type {
    pub symbol: String
}

#[derive(Debug, PartialEq)]
pub struct Keyword {
    pub keyword: String
}

#[derive(Debug, PartialEq)]
pub struct Operation {
    pub operation: String
}

#[derive(Debug, PartialEq)]
pub struct BlockDelimeter {
    pub delimeter: String,
    pub is_close: bool
}
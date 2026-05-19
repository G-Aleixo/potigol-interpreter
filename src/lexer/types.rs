#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Type(String),
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
    Keyword(String),
    Operation(String),
    BlockDelimeter(String, bool),
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
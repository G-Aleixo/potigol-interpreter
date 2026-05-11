#[derive(Debug)]
pub enum Token<'a> {
    Identifier(Identifier),
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    NewLine,
    Comma,
    Unknown(char),
    Keyword(Keyword),
    Operation(Operation<'a>),
    BlockDelimeter(BlockDelimeter<'a>),
}

#[derive(Debug)]
pub struct Identifier {
    pub symbol: String
}

#[derive(Debug)]
pub struct Keyword {
    pub keyword: String
}

#[derive(Debug)]
pub struct Operation<'a> {
    pub operation: &'a str
}

#[derive(Debug)]
pub struct BlockDelimeter<'a> {
    pub delimeter: &'a str,
    pub is_close: bool
}
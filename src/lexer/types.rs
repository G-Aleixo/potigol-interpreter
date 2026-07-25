#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'s> {
    Identifier(&'s str),
    Type(&'s str),
    StringStart,
    StringFragment(&'s str),
    ExprStart,
    ExprEnd,
    StringEnd,
    Character(char),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    NewLine,
    Comma,
    Period,
    Colon,
    Unknown(char),
    Keyword(&'s str),
    Operation(&'s str),
    // the "fim" block delimeter is a keyword
    BlockDelimeter(char, bool),
}
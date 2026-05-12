pub enum Expr {
    Literal(Value),
    Variable(String),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Call(String, Vec<Expr>),
    Lambda(Vec<String>, Box<Expr>), // argument names, expression
    List(Vec<Expr>),
    Tuple(Vec<Expr>),
}

pub enum Stmt {
    ConstAssignment(String, Expr),
    VarAssignment(String, Expr),
    ExprStmt(Expr),
}

pub enum BinOp {
    Plus,
    Minus,
    Mult,
    Div,
    IntDiv,
    Mod,
    Pow,
}

pub enum Value {
    Integer(i64),
    Float(f64),
    String(String)
}
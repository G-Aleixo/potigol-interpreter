#[derive(Clone)]
pub enum Expr {
    Literal(Value),
    String(Vec<StringPart>),
    Variable(String),
    Binary(Box<Expr>, BinOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Ternary(Box<Expr>, Vec<Stmt>, Vec<Stmt>),
    Call(String, Vec<Expr>),
    Lambda(Vec<String>, Box<Expr>), // argument names, expression
    List(Vec<Expr>),
    Tuple(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ConstAssignment(String, Expr),
    VarAssignment(String, Expr),
    ExprStmt(Expr),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Plus,
    Minus,
    Mult,
    Div,
    IntDiv,
    Mod,
    Pow,
    And,
    Or,
    DotAccess,
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Index,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
    Write,
    Print,
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum StringPart {
    Fragment(String),
    Expr(Expr)
}

impl From<&String> for BinOp {
    fn from(value: &String) -> Self {
        match value.as_ref() {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Mult,
            "/" => Self::Div,
            "^" => Self::Pow,
            "div" => Self::IntDiv,
            "mod" => Self::Mod,
            "e" => Self::And,
            "ou" => Self::Or,
            "." => Self::DotAccess,
            "==" => Self::Equal,
            "<>" => Self::NotEqual,
            ">" => Self::Greater,
            ">=" => Self::GreaterOrEqual,
            "<" => Self::Less,
            "<=" => Self::LessOrEqual,
            "[" => Self::Index,
            v => panic!("Invalid infix operator {v}"),
        }
    }
}

impl From<&String> for UnaryOp {
    fn from(value: &String) -> Self {
        match value.as_ref() {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "não" => Self::Not,
            "imprima" => Self::Write,
            "escreva" => Self::Print,
            v => panic!("Invalid suffix operator {v}"),
        }
    }
}

// for S-expression debugging stuff
impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(value) => write!(f, "{value}"),
            Expr::String(string) => write!(f, "{string:?}"),
            Expr::Variable(var) => write!(f, "{var}"),
            Expr::Binary(expr1, bin_op, expr2) => write!(f, "({bin_op} {expr1:?} {expr2:?})"),
            Expr::Unary(unary_op, expr) => write!(f, "({unary_op} {expr:?})"),
            Expr::Ternary(condition, branch1, branch2) => {
                write!(f, "{condition:?} {branch1:?} {branch2:?}")
            }
            Expr::Call(_, _exprs) => todo!(),
            Expr::Lambda(_items, _expr) => todo!(),
            Expr::List(_exprs) => todo!(),
            Expr::Tuple(_exprs) => todo!(),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(int) => write!(f, "{int}"),
            Value::Float(float) => write!(f, "{float}"),
            Value::Boolean(bool) => write!(f, "{bool}"),
        }
    }
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Plus => write!(f, "+"),
            BinOp::Minus => write!(f, "-"),
            BinOp::Mult => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::IntDiv => write!(f, "div"),
            BinOp::Mod => write!(f, "mod"),
            BinOp::Pow => write!(f, "^"),
            BinOp::And => write!(f, "&"),
            BinOp::Or => write!(f, "|"),
            BinOp::DotAccess => write!(f, "."),
            BinOp::Equal => write!(f, "=="),
            BinOp::NotEqual => write!(f, "<>"),
            BinOp::Greater => write!(f, ">"),
            BinOp::GreaterOrEqual => write!(f, ">="),
            BinOp::Less => write!(f, "<"),
            BinOp::LessOrEqual => write!(f, "<="),
            BinOp::Index => write!(f, "["),
        }
    }
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Plus => write!(f, "+"),
            UnaryOp::Minus => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
            UnaryOp::Write => write!(f, "imprima"),
            UnaryOp::Print => write!(f, "escreva"),
        }
    }
}

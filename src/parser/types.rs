#[derive(Clone, PartialEq)]
pub enum Expr<'s> {
    Literal(Value),
    String(Vec<StringPart<'s>>),
    Identifier(&'s str),
    Binary {
        lhs: Box<Expr<'s>>,
        op: BinOp,
        rhs: Box<Expr<'s>>
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr<'s>>
    },
    Ternary {
        cond: Box<Expr<'s>>,
        if_true: Vec<Stmt<'s>>,
        if_false: Vec<Stmt<'s>>
    },
    While {
        cond: Box<Expr<'s>>,
        stmts: Vec<Stmt<'s>>
    },
    RangeFor {
        control: Box<Expr<'s>>, 
        start: Box<Expr<'s>>,
        end: Box<Expr<'s>>,
        step: Option<Box<Expr<'s>>>,
        stmts: Vec<Stmt<'s>>
    },
    IterFor {
        control: Box<Expr<'s>>,
        iterator: Box<Expr<'s>>,
        stmts: Vec<Stmt<'s>>
    },
    Call(&'s str, Vec<Expr<'s>>),
    Lambda(Vec<&'s str>, Box<Expr<'s>>), // argument names, Expr<'s>ession
    List(Vec<Expr<'s>>),
    Tuple(Vec<Expr<'s>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt<'s> {
    ConstAssignment(Expr<'s>),
    VarAssignment(Expr<'s>),
    ExprStmt(Expr<'s>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    ConstAssignment,
    VarAssignment,
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
    Call,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
    Write,
    Print,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Character(char)
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringPart<'s> {
    Fragment(&'s str),
    Expr(Expr<'s>)
}

impl TryFrom<&str> for BinOp {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "=" => Ok(Self::ConstAssignment),
            ":=" => Ok(Self::VarAssignment),
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "*" => Ok(Self::Mult),
            "/" => Ok(Self::Div),
            "^" => Ok(Self::Pow),
            "div" => Ok(Self::IntDiv),
            "mod" => Ok(Self::Mod),
            "e" => Ok(Self::And),
            "ou" => Ok(Self::Or),
            "." => Ok(Self::DotAccess),
            "==" => Ok(Self::Equal),
            "<>" => Ok(Self::NotEqual),
            ">" => Ok(Self::Greater),
            ">=" => Ok(Self::GreaterOrEqual),
            "<" => Ok(Self::Less),
            "<=" => Ok(Self::LessOrEqual),
            "[" => Ok(Self::Index),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for UnaryOp {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "não" => Ok(Self::Not),
            "imprima" => Ok(Self::Write),
            "escreva" => Ok(Self::Print),
            _ => Err(()),
        }
    }
}

// for S-Expr<'s>ession debugging stuff
impl<'s> std::fmt::Debug for Expr<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(value) => write!(f, "{value}"),
            Expr::String(string) => write!(f, "{string:?}"),
            Expr::Identifier(var) => write!(f, "{var}"),
            Expr::Binary{ lhs, op, rhs} => write!(f, "({op} {lhs:?} {rhs:?})"),
            Expr::Unary{op, expr} => write!(f, "({op} {expr:?})"),
            Expr::Ternary{cond, if_true, if_false} => {
                write!(f, "{cond:?} {if_true:?} {if_false:?}")
            }
            Expr::While{cond, stmts} => {
                write!(f, "{cond:?} {stmts:?}")
            }
            Expr::RangeFor{control, start, end, step, stmts} => {
                write!(f, "{control:?} {start:?} {end:?} {step:?} {stmts:?}")
            }
            Expr::IterFor { control, iterator, stmts } => {
                write!(f, "{control:?} {iterator:?} {stmts:?}")
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
            Value::Character(char) => write!(f, "{char}")
        }
    }
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::ConstAssignment => write!(f, "="),
            BinOp::VarAssignment => write!(f, ":="),
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
            BinOp::Call => write!(f, "(")
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

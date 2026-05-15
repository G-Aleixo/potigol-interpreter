use std::collections::HashMap;

use crate::parser::{self, Expr};

pub struct Enviroment<'a> {
    parent: Option<&'a Enviroment<'a>>,
    const_vars: HashMap<String, Value>,
    vars: HashMap<String, Value>
}

impl<'a> Enviroment<'a> {
    pub fn resolve(&self, varname: &String) -> Option<&Value> {
        if let Some(value) = self.const_vars.get(varname).or(match &self.parent {
            Some(parent) => parent.resolve(varname),
            None => None
        }) {
            return Some(value);
        }
        self.vars.get(varname).or(match &self.parent {
            Some(parent) => parent.resolve(varname),
            None => None
        })
    }

    pub fn new_child(&self) -> Enviroment {
        Enviroment { parent: Some(self), const_vars: HashMap::new(), vars: HashMap::new() }
    }
}

pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Lambda(Vec<String>, Box<Expr>),
    List(Vec<Value>),
    Tuple(Vec<Value>),
}

impl From<parser::Value> for Value {
    fn from(value: parser::Value) -> Self {
        match value {
            parser::Value::Integer(int) => Self::Integer(int),
            parser::Value::Float(float) => Self::Float(float),
            parser::Value::String(string) => Self::String(string),
            parser::Value::Boolean(bool) => Self::Boolean(bool),
        }
    }
}

impl std::ops::Add<Self> for &Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Integer(num1) => match rhs {
                Value::Integer(num2) => Value::Integer(num1 + num2),
                Value::Float(num2) => Value::Float(*num1 as f64 + num2),
                Value::String(str) => Value::String(format!("{num1}{str}")),
                Value::Boolean(_) => panic!("Cannot add integer and boolean"),
                Value::Lambda(_, _) => panic!("Cannot add integer and lambda expression"),
                Value::List(_) => panic!("Cannot add integer and list"),
                Value::Tuple(_) => panic!("Cannot add integer and tuple"),
            },
            Value::Float(num1) => match rhs {
                Value::Integer(num2) => Value::Integer(*num1 as i64 +  num2),
                Value::Float(num2) => Value::Float(num1 + num2),
                Value::String(str) => Value::String(format!("{num1}{str}")),
                Value::Boolean(_) => panic!("Cannot add float and boolean"),
                Value::Lambda(_, _) => panic!("Cannot add float and lambda expression"),
                Value::List(_) => panic!("Cannot add float and list"),
                Value::Tuple(_) => panic!("Cannot add float and tuple"),
            },
            Value::String(str1) => match rhs {
                Value::Integer(num) => Value::String(format!("{str1}{num}")),
                Value::Float(num) => Value::String(format!("{str1}{num}")),
                Value::String(str2) => Value::String(format!("{str1}{str2}")),
                Value::Boolean(bool) => Value::String(format!("{str1}{bool}")),
                Value::Lambda(_, _) => panic!("Cannot concatenate string and lambda expression"),
                Value::List(_) => panic!("Cannot concatenate string and list"),
                Value::Tuple(_) => panic!("Cannot concatenate string and tuple"),
            },
            Value::Boolean(_) => panic!("Cannot add boolean"),
            Value::Lambda(items, expr) => todo!(),
            Value::List(values) => todo!(),
            Value::Tuple(values) => todo!(),
        }
    }
}
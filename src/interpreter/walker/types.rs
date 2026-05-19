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

    pub fn empty() -> Enviroment<'static> {
        Enviroment { parent: None, const_vars: HashMap::new(), vars: HashMap::new() }
    }

    pub fn new_child(&self) -> Enviroment {
        Enviroment { parent: Some(self), const_vars: HashMap::new(), vars: HashMap::new() }
    }
}

#[derive(Clone, Debug)]
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

impl std::ops::Add<Self> for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Value::Integer(num1), Value::Integer(num2)) => Value::Integer(num1 + num2),
            (Value::Integer(num1), Value::Float(num2)) => Value::Float(num1 as f64 + num2),
            (Value::Integer(num1), Value::String(str)) => Value::String(format!("{num1}{str}")),
            (Value::Integer(_), _) => panic!("Cannot sum integer {self:?} with {rhs:?}"),
            (Value::Float(num1), Value::Integer(num2)) => Value::Float(num1 + num2 as f64),
            (Value::Float(num1), Value::Float(num2)) => Value::Float(num1 + num2),
            (Value::Float(num1), Value::String(str)) => Value::String(format!("{num1}{str}")),
            (Value::Float(_), _) => panic!("Cannot sum float {self:?} with {rhs:?}"),
            (Value::String(str), Value::Integer(num)) => Value::String(format!("{str}{num}")),
            (Value::String(str), Value::Float(num)) => Value::String(format!("{str}{num}")),
            (Value::String(str), Value::String(str2)) => Value::String(format!("{str}{str2}")),
            (Value::String(str), Value::Boolean(bool)) => Value::String(format!("{str}{bool}")),
            (Value::String(_), _) => panic!("Cannot sum string {self:?} with {rhs:?}"),
            (Value::Boolean(_), _) => panic!("Cannot sum boolean {self:?} with {rhs:?}"),
            _ => panic!("Cannot sum {self:?} with {rhs:?}")
        }
    }
}
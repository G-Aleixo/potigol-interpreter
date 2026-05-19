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
            (Value::Integer(num1), Value::String(str)) => todo!(),
            (Value::Integer(_), _) => panic!("Cannot sum {self:?} with {rhs:?}"),
            (Value::Float(num1), Value::Integer(num2)) => Value::Float(num1 + num2 as f64),
            (Value::Float(num1), Value::Float(num2)) => Value::Float(num1 + num2),
            (Value::Float(num1), Value::String(str)) => todo!(),
            (Value::Float(_), _) => panic!("Cannot sum {self:?} with {rhs:?}"),
            (Value::String(_), Value::Integer(_)) => todo!(),
            (Value::String(_), Value::Float(_)) => todo!(),
            (Value::String(_), Value::String(_)) => todo!(),
            (Value::String(_), Value::Boolean(_)) => todo!(),
            (Value::String(_), _) => panic!("Cannot sum {self:?} with {rhs:?}"),
            (Value::Boolean(_), Value::Integer(_)) => todo!(),
            (Value::Boolean(_), Value::Float(_)) => todo!(),
            (Value::Boolean(_), Value::String(_)) => todo!(),
            (Value::Boolean(_), Value::Boolean(_)) => todo!(),
            (Value::Boolean(_), Value::Lambda(items, expr)) => todo!(),
            (Value::Boolean(_), Value::List(values)) => todo!(),
            (Value::Boolean(_), Value::Tuple(values)) => todo!(),
            _ => todo!()
        }
    }
}
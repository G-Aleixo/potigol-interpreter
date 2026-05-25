use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::parser::{self, Expr};

#[derive(Clone)]
pub struct Enviroment {
    parent: Option<Rc<RefCell<Enviroment>>>,
    const_vars: HashMap<String, Value>,
    vars: HashMap<String, Value>
}

impl Enviroment {
    pub fn resolve(&self, varname: &String) -> Option<Value> {
        // Check constants
        if let Some(value) = self.const_vars.get(varname) {
            return Some(value.clone());
        }
        // Check mutable vars
        if let Some(value) = self.vars.get(varname) {
            return Some(value.clone());
        }
        // Check parent recursively
        if let Some(parent) = &self.parent {
            return parent.borrow().resolve(varname);
        }
        None
    }

    pub fn set_var(&mut self, varname: &String, value: Value) {
        self.vars.insert(varname.to_string(), value);
    }
    
    pub fn assign_var(&mut self, varname: &String, value: Value) -> Result<(), String>{
        if self.vars.contains_key(varname) {
            self.vars.insert(varname.to_string(), value);
            Ok(())
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().assign_var(varname, value)?;
            Ok(())
        } else {
            Err(format!("Undeclared variable {}", varname))
        }
    }

    pub fn empty() -> Enviroment {
        Enviroment { parent: None, const_vars: HashMap::new(), vars: HashMap::new() }
    }

    pub fn new_child(&self) -> Enviroment {
        Enviroment { parent: Some(Rc::new(RefCell::new(self.clone()))), const_vars: HashMap::new(), vars: HashMap::new() }
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
    None,
}

impl Value {
    pub fn int_div(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Integer(num1), Value::Integer(num2)) => Value::Integer(num1 / num2),
            _ => panic!("Cannot int div {self:?} with {other:?}")
        }
    }
    pub fn pow(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Integer(num1), Value::Integer(num2)) => Value::Float((*num1 as f64).powi(*num2 as i32)),
            (Value::Integer(num1), Value::Float(num2)) => Value::Float((*num1 as f64).powf(*num2)),
            (Value::Integer(_), _) => panic!("Cannot pow integer {self:?} with {other:?}"),
            (Value::Float(num1), Value::Integer(num2)) => Value::Float(num1.powf(*num2 as f64)),
            (Value::Float(num1), Value::Float(num2)) => Value::Float(num1.powf(*num2)),
            (Value::Float(_), _) => panic!("Cannot pow float {self:?} with {other:?}"),
            _ => panic!("Cannot pow {self:?} with {other:?}")
        }
    }
}

impl std::fmt::Display for &Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(num) => write!(f, "{num}"),
            Value::Float(num) => write!(f, "{num}"),
            Value::String(str) => write!(f, "{str}"),
            Value::Boolean(bool) => write!(f, "{bool}"),
            Value::Lambda(items, expr) => todo!(),
            Value::List(values) => {
                write!(f, "[")?;
                for i in 0..values.len() {
                    write!(f, "{}", &values[i])?;
                    if i != values.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            },
            Value::Tuple(values) => {
                write!(f, "(")?;
                for i in 0..values.len() {
                    write!(f, "{}", &values[i])?;
                    if i != values.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")
            },
            Value::None => {
                write!(f, "")
            },
        }
    }
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

impl From<&Value> for bool {
    fn from(value: &Value) -> Self {
        match value {
            Value::Integer(num) => *num != 0,
            Value::Float(num) => *num != 0.0,
            Value::String(str) => !str.is_empty(),
            Value::Boolean(bool) => *bool,
            Value::Lambda(_, _) => true,
            Value::List(values) => !values.is_empty(),
            Value::Tuple(values) => !values.is_empty(),
            Value::None => false,
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

impl std::ops::Sub<Self> for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Value::Integer(num1), Value::Integer(num2)) => Value::Integer(num1 - num2),
            (Value::Integer(num1), Value::Float(num2)) => Value::Float(num1 as f64 - num2),
            (Value::Integer(num1), Value::String(str)) => Value::String(format!("{num1}{str}")),
            (Value::Integer(_), _) => panic!("Cannot sub integer {self:?} with {rhs:?}"),
            (Value::Float(num1), Value::Integer(num2)) => Value::Float(num1 - num2 as f64),
            (Value::Float(num1), Value::Float(num2)) => Value::Float(num1 - num2),
            (Value::Float(num1), Value::String(str)) => Value::String(format!("{num1}{str}")),
            (Value::Float(_), _) => panic!("Cannot sub float {self:?} with {rhs:?}"),
            (Value::String(str), Value::String(str2)) => Value::String(str.chars().filter(|c| !str2.contains(*c)).collect()),
            (Value::String(_), _) => panic!("Cannot sub string {self:?} with {rhs:?}"),
            _ => panic!("Cannot sub {self:?} with {rhs:?}")
        }
    }
}

impl std::ops::Mul<Self> for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Value::Integer(num1), Value::Integer(num2)) => Value::Integer(num1 * num2),
            (Value::Integer(num1), Value::Float(num2)) => Value::Float(num1 as f64 * num2),
            (Value::Integer(_), _) => panic!("Cannot mul integer {self:?} with {rhs:?}"),
            (Value::Float(num1), Value::Integer(num2)) => Value::Float(num1 * num2 as f64),
            (Value::Float(num1), Value::Float(num2)) => Value::Float(num1 * num2),
            (Value::Float(_), _) => panic!("Cannot mul float {self:?} with {rhs:?}"),
            (Value::String(str), Value::Integer(num)) => Value::String(str.repeat(num as usize)),
            (Value::String(_), _) => panic!("Cannot mul string {self:?} with {rhs:?}"),
            _ => panic!("Cannot sum {self:?} with {rhs:?}")
        }
    }
}

impl std::ops::Div<Self> for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Value::Integer(num1), Value::Integer(num2)) => Value::Float(num1 as f64 / num2 as f64),
            (Value::Integer(num1), Value::Float(num2)) => Value::Float(num1 as f64 / num2),
            (Value::Integer(_), _) => panic!("Cannot div integer {self:?} with {rhs:?}"),
            (Value::Float(num1), Value::Integer(num2)) => Value::Float(num1 / num2 as f64),
            (Value::Float(num1), Value::Float(num2)) => Value::Float(num1 / num2),
            (Value::Float(_), _) => panic!("Cannot div float {self:?} with {rhs:?}"),
            _ => panic!("Cannot div {self:?} with {rhs:?}")
        }
    }
}

impl std::ops::Rem<Self> for Value {
    type Output = Value;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Value::Integer(num1), Value::Integer(num2)) => Value::Integer(num1 % num2),
            (Value::Integer(num1), Value::Float(num2)) => Value::Float(num1 as f64 % num2),
            (Value::Integer(_), _) => panic!("Cannot mod integer {self:?} with {rhs:?}"),
            (Value::Float(num1), Value::Integer(num2)) => Value::Float(num1 % num2 as f64),
            (Value::Float(num1), Value::Float(num2)) => Value::Float(num1 % num2),
            (Value::Float(_), _) => panic!("Cannot mod float {self:?} with {rhs:?}"),
            _ => panic!("Cannot mod {self:?} with {rhs:?}")
        }
    }
}

impl std::ops::BitAnd<Self> for Value {
    type Output = Value;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Value::Boolean(bool1), Value::Boolean(bool2)) => Value::Boolean(bool1 && bool2),
            _ => panic!("Cannot and {self:?} with {rhs:?}")
        }
    }
}

impl std::ops::BitOr<Self> for Value {
    type Output = Value;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Value::Boolean(bool1), Value::Boolean(bool2)) => Value::Boolean(bool1 || bool2),
            _ => panic!("Cannot or {self:?} with {rhs:?}")
        }
    }
}

impl std::ops::Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Value::Integer(num) => Value::Integer(-num),
            Value::Float(num) => Value::Float(-num),
            _ => panic!("Cannot negate {self:?}")
        }
    }
}

impl std::ops::Not for Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        match self {
            Value::Boolean(bool) => Value::Boolean(!bool),
            _ => panic!("Cannot invert {self:?}")
        }
    }
}

impl PartialEq<Self> for Value {
    fn eq(&self, rhs: &Self) -> bool {
        //TODO: implement actual equality later
        format!("{self:?}") == format!("{rhs:?}")
    }
}

impl PartialOrd<Self> for Value {
    fn ge(&self, rhs: &Self) -> bool {
        match (self.clone(), rhs.clone()) {
            (Value::Integer(num1), Value::Integer(num2)) => num1 >= num2,
            (Value::Integer(num1), Value::Float(num2)) => num1 as f64 >= num2,
            (Value::Integer(_), Value::String(_)) => todo!(),
            (Value::Float(num1), Value::Integer(num2)) => num1 >= num2 as f64,
            (Value::Float(num1), Value::Float(num2)) => num1 >= num2,
            (Value::String(_), Value::String(_)) => todo!(),
            _ => panic!("Cannot compare {self:?} with {rhs:?}")
        }
    }

    fn le(&self, rhs: &Self) -> bool {
        match (self.clone(), rhs.clone()) {
            (Value::Integer(num1), Value::Integer(num2)) => num1 <= num2,
            (Value::Integer(num1), Value::Float(num2)) => num1 as f64 <= num2,
            (Value::Integer(_), Value::String(_)) => todo!(),
            (Value::Float(num1), Value::Integer(num2)) => num1 <= num2 as f64,
            (Value::Float(num1), Value::Float(num2)) => num1 <= num2,
            (Value::String(_), Value::String(_)) => todo!(),
            _ => panic!("Cannot compare {self:?} with {rhs:?}")
        }
    }

    fn gt(&self, rhs: &Self) -> bool {
        match (self.clone(), rhs.clone()) {
            (Value::Integer(num1), Value::Integer(num2)) => num1 > num2,
            (Value::Integer(num1), Value::Float(num2)) => num1 as f64 > num2,
            (Value::Integer(_), Value::String(_)) => todo!(),
            (Value::Float(num1), Value::Integer(num2)) => num1 > num2 as f64,
            (Value::Float(num1), Value::Float(num2)) => num1 > num2,
            (Value::String(_), Value::String(_)) => todo!(),
            _ => panic!("Cannot compare {self:?} with {rhs:?}")
        }
    }


    fn lt(&self, rhs: &Self) -> bool {
        match (self.clone(), rhs.clone()) {
            (Value::Integer(num1), Value::Integer(num2)) => num1 < num2,
            (Value::Integer(num1), Value::Float(num2)) => (num1 as f64) < num2,
            (Value::Integer(_), Value::String(_)) => todo!(),
            (Value::Float(num1), Value::Integer(num2)) => num1 < num2 as f64,
            (Value::Float(num1), Value::Float(num2)) => num1 < num2,
            (Value::String(_), Value::String(_)) => todo!(),
            _ => panic!("Cannot compare {self:?} with {rhs:?}")
        }
    }

    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        if self.lt(rhs) {
            return Some(std::cmp::Ordering::Less)
        } else if self.gt(rhs) {
            return Some(std::cmp::Ordering::Greater)
        } else if self.eq(rhs) {
            return Some(std::cmp::Ordering::Equal)
        }

        None
    }
}
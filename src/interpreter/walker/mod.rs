pub mod types;

use std::io::{BufRead, Write};

use types::*;

use crate::parser::{BinOp, Expr, Stmt, UnaryOp};

pub struct Interpreter {
    envs: Vec<Enviroment>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::loaded()
    }
}

impl Interpreter {
    pub fn empty() -> Interpreter {
        Interpreter {
            envs: vec![Enviroment::empty()],
        }
    }

    pub fn loaded() -> Interpreter {
        let mut interp = Self::empty();

        interp.envs[0].new_override(&"leia_texto".to_string(), |_| {
            let mut stdin = std::io::stdin().lock();
            let mut buf = String::new();
            stdin.read_line(&mut buf).expect("Failed to read from stdin");
            Value::String(buf.trim_end().to_string())
        }).expect("Failed to inject \"leia_texto\" override");

        interp.envs[0].new_override(&"leia_inteiro".to_string(), |_| {
            let mut stdin = std::io::stdin().lock();
            let mut buf = String::new();
            stdin.read_line(&mut buf).expect("Failed to read from stdin");
            match buf.trim().parse() {
                Ok(num) => Value::Integer(num),
                Err(e) => panic!("{e}"),
            }
        }).expect("Failed to inject \"leia_inteiro\" override");

        interp
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.interpret_single(&statement);
        }
    }

    pub fn interpret_single(&mut self, statement: &Stmt) -> Value {
        match statement {
            Stmt::ConstAssignment(varname, expr) => self.evaluate_const_assignment(varname, expr),
            Stmt::VarAssignment(varname, expr) => self.evaluate_var_assignment(varname, expr),
            Stmt::ExprStmt(expr) => self.evaluate_expr_stmt(expr),
        }
    }

    fn execute_print(&mut self, expr: &Expr) -> Value {
        let value = self.evaluate_expression(expr);
        let mut stdout = std::io::stdout().lock();
        println!("{}", &value);
        stdout.flush().expect("Could not flush to stdout");
        Value::None
    }

    fn execute_write(&mut self, expr: &Expr) -> Value {
        let value = self.evaluate_expression(expr);
        let mut stdout = std::io::stdout().lock();
        print!("{}", &value);
        stdout.flush().expect("Could not flush to stdout");
        Value::None
    }

    fn evaluate_const_assignment(&mut self, _varname: &String, _expr: &Expr) -> Value {
        todo!("Const assignment not implemented");
    }

    fn evaluate_var_assignment(&mut self, varname: &String, expr: &Expr) -> Value {
        let value = self.evaluate_expression(expr);
        self.set_var(varname, value.clone());
        value
    }

    fn evaluate_expr_stmt(&mut self, expr: &Expr) -> Value {
        self.evaluate_expression(expr)
    }

    #[allow(unused_variables)]
    fn evaluate_expression(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal(value) => value.clone().into(),
            Expr::Variable(varname) => self
                .get_var(varname)
                .unwrap_or_else(|| panic!("Variable {varname} not defined"))
                .clone(),
            Expr::Binary(expr1, bin_op, expr2) => self.evaluate_bin_op(expr1, bin_op, expr2),
            Expr::Unary(unary_op, expr) => self.evaluate_unary_op(expr, unary_op),
            Expr::Ternary(expr, stmts_true, stmts_false) => {
                let expr_result = bool::from(&self.evaluate_expression(expr));
                let mut value = Value::None;
                if expr_result {
                    for i in 0..stmts_true.len() {
                        if i == stmts_true.len() - 1 {
                            value = self.interpret_single(&stmts_true[i]);
                        } else {
                            self.interpret_single(&stmts_true[i]);
                        }
                    }
                } else {
                    for i in 0..stmts_false.len() {
                        if i == stmts_false.len() - 1 {
                            value = self.interpret_single(&stmts_false[i]);
                        } else {
                            self.interpret_single(&stmts_false[i]);
                        }
                    }
                }

                value
            }
            Expr::Call(func_name, exprs) => {
                // get the function statement body
                // create a new enviroment and append it to the env stack
                // run the function body
                // return the value
                todo!()
            }
            // copy over to the Value format
            Expr::Lambda(items, expr) => Value::Lambda(items.to_vec(), expr.clone()),
            Expr::List(exprs) => Value::List(
                exprs
                    .iter()
                    .map(|expr| self.evaluate_expression(expr))
                    .collect(),
            ),
            Expr::Tuple(exprs) => Value::Tuple(
                exprs
                    .iter()
                    .map(|expr| self.evaluate_expression(expr))
                    .collect(),
            ),
        }
    }

    fn evaluate_bin_op(&mut self, expr1: &Expr, op: &BinOp, expr2: &Expr) -> Value {
        match op {
            BinOp::Plus => self.evaluate_expression(expr1) + self.evaluate_expression(expr2),
            BinOp::Minus => self.evaluate_expression(expr1) - self.evaluate_expression(expr2),
            BinOp::Mult => self.evaluate_expression(expr1) * self.evaluate_expression(expr2),
            BinOp::Div => self.evaluate_expression(expr1) / self.evaluate_expression(expr2),
            BinOp::IntDiv => self
                .evaluate_expression(expr1)
                .int_div(&self.evaluate_expression(expr2)),
            BinOp::Mod => self.evaluate_expression(expr1) % self.evaluate_expression(expr2),
            BinOp::Pow => self
                .evaluate_expression(expr1)
                .pow(&self.evaluate_expression(expr2)),
            BinOp::And => self.evaluate_expression(expr1) & self.evaluate_expression(expr2),
            BinOp::Or => self.evaluate_expression(expr1) | self.evaluate_expression(expr2),
            BinOp::DotAccess => todo!(),
            BinOp::Equal => {
                Value::Boolean(self.evaluate_expression(expr1) == self.evaluate_expression(expr2))
            }
            BinOp::NotEqual => {
                Value::Boolean(self.evaluate_expression(expr1) != self.evaluate_expression(expr2))
            }
            BinOp::Greater => {
                Value::Boolean(self.evaluate_expression(expr1) > self.evaluate_expression(expr2))
            }
            BinOp::GreaterOrEqual => {
                Value::Boolean(self.evaluate_expression(expr1) >= self.evaluate_expression(expr2))
            }
            BinOp::Less => {
                Value::Boolean(self.evaluate_expression(expr1) < self.evaluate_expression(expr2))
            }
            BinOp::LessOrEqual => {
                Value::Boolean(self.evaluate_expression(expr1) <= self.evaluate_expression(expr2))
            }
            BinOp::Index => todo!(),
        }
    }

    fn evaluate_unary_op(&mut self, expr: &Expr, op: &UnaryOp) -> Value {
        match op {
            UnaryOp::Plus => self.evaluate_expression(expr), // do literally nothing lol
            UnaryOp::Minus => -self.evaluate_expression(expr),
            UnaryOp::Not => !self.evaluate_expression(expr),
            UnaryOp::Print => self.execute_print(expr),
            UnaryOp::Write => self.execute_write(expr),
        }
    }

    pub fn get_var(&self, varname: &String) -> Option<Value> {
        self.envs.last().unwrap().resolve(varname)
    }

    fn set_var(&mut self, varname: &String, value: Value) {
        if self.get_var(varname).is_some() {
            self.envs
                .last_mut()
                .unwrap()
                .assign_var(varname, value)
                .unwrap();
        } else {
            self.envs.last_mut().unwrap().set_var(varname, value);
        }
    }
}

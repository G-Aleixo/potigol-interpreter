pub mod types;

use types::*;

use crate::parser::{BinOp, Expr, Stmt, UnaryOp};

pub struct Interpreter {
    envs: Vec<Enviroment>
}


impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { envs: vec![Enviroment::empty()] }
    }
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.interpret_single(&statement);
        }
    }

    pub fn interpret_single(&mut self, statement: &Stmt) -> Value{
        match statement {
            Stmt::ConstAssignment(varname, expr) => self.evaluate_const_assignment(varname, expr),
            Stmt::VarAssignment(varname, expr) => self.evaluate_var_assignment(varname, expr),
            Stmt::ExprStmt(expr) => self.evaluate_expr_stmt(&expr),
        }
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
    fn evaluate_expression(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal(value) => value.clone().into(),
            Expr::Variable(varname) => self.get_var(varname).expect(&format!("Variable {varname} not defined")).clone(),
            Expr::Binary(expr1, bin_op, expr2) => self.evaluate_bin_op(expr1, bin_op, expr2),
            Expr::Unary(unary_op, expr) => self.evaluate_unary_op(expr, unary_op),
            Expr::Ternary(expr, stmts, stmts1) => {
                // check the expression and run corresponding body
                // return the value of the final statement
                todo!()
            },
            Expr::Call(func_name, exprs) => {
                // get the function statement body
                // create a new enviroment and append it to the env stack
                // run the function body
                // return the value
                todo!()
            },
            // copy over to the Value format
            Expr::Lambda(items, expr) => Value::Lambda(items.to_vec(), expr.clone()),
            Expr::List(exprs) => Value::List(exprs.iter().map(|expr| self.evaluate_expression(expr)).collect()),
            Expr::Tuple(exprs) => Value::Tuple(exprs.iter().map(|expr| self.evaluate_expression(expr)).collect()),
        }
    }

    fn evaluate_bin_op(&self, expr1: &Expr, op: &BinOp, expr2: &Expr) -> Value {
        match op {
            BinOp::Plus => self.evaluate_expression(expr1) + self.evaluate_expression(expr2),
            BinOp::Minus => self.evaluate_expression(expr1) - self.evaluate_expression(expr2),
            BinOp::Mult => self.evaluate_expression(expr1) * self.evaluate_expression(expr2),
            BinOp::Div => self.evaluate_expression(expr1) / self.evaluate_expression(expr2),
            BinOp::IntDiv => self.evaluate_expression(expr1).int_div(&self.evaluate_expression(expr2)),
            BinOp::Mod => self.evaluate_expression(expr1) % self.evaluate_expression(expr2),
            BinOp::Pow => self.evaluate_expression(expr1).pow(&self.evaluate_expression(expr2)),
            BinOp::And => self.evaluate_expression(expr1) & self.evaluate_expression(expr2),
            BinOp::Or => self.evaluate_expression(expr1) | self.evaluate_expression(expr2),
            BinOp::DotAccess => todo!(),
            BinOp::Equal => Value::Boolean(self.evaluate_expression(expr1) == self.evaluate_expression(expr2)),
            BinOp::NotEqual => Value::Boolean(self.evaluate_expression(expr1) != self.evaluate_expression(expr2)),
            BinOp::Greater => Value::Boolean(self.evaluate_expression(expr1) > self.evaluate_expression(expr2)),
            BinOp::GreaterOrEqual => Value::Boolean(self.evaluate_expression(expr1) >= self.evaluate_expression(expr2)),
            BinOp::Less => Value::Boolean(self.evaluate_expression(expr1) < self.evaluate_expression(expr2)),
            BinOp::LessOrEqual => Value::Boolean(self.evaluate_expression(expr1) <= self.evaluate_expression(expr2)),
            BinOp::Index => todo!(),
        }
    }

    fn evaluate_unary_op(&self, expr: &Expr, op: &UnaryOp) -> Value{
        match op {
            UnaryOp::Plus => self.evaluate_expression(expr), // do literally nothing lol
            UnaryOp::Minus => -self.evaluate_expression(expr),
            UnaryOp::Not => !self.evaluate_expression(expr),
        }
    }

    pub fn get_var(&self, varname: &String) -> Option<Value> {
        self.envs.last().unwrap().resolve(varname)
    }

    fn set_var(&mut self, varname: &String, value: Value) {
        if self.get_var(varname).is_some() {
            self.envs.last_mut().unwrap().assign_var(varname, value).unwrap();
        } else {
            self.envs.last_mut().unwrap().set_var(varname, value);
        }
    }
}
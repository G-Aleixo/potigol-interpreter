pub mod types;

use types::*;

use crate::parser::{BinOp, Expr, Stmt};

pub struct Interpreter<'a> {
    envs: Vec<Enviroment<'a>>
}


impl Interpreter<'_> {
    pub fn new() -> Interpreter<'static> {
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

    fn evaluate_const_assignment(&mut self, varname: &String, expr: &Expr) -> Value {
        todo!("Const assignment not implemented");
    }
    fn evaluate_var_assignment(&mut self, varname: &String, expr: &Expr) -> Value{
        todo!("Var assignment not implemented");
    }
    fn evaluate_expr_stmt(&mut self, expr: &Expr) -> Value {
        self.evaluate_expression(expr)
    }

    fn evaluate_expression(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal(value) => value.clone().into(),
            Expr::Variable(varname) => self.get_var(varname).expect(&format!("Variable {varname} not defined")).clone(),
            Expr::Binary(expr1, bin_op, expr2) => self.evaluate_bin_op(expr1, bin_op, expr2),
            Expr::Unary(unary_op, expr) => todo!(),
            Expr::Ternary(expr, stmts, stmts1) => todo!(),
            Expr::Call(_, exprs) => todo!(),
            Expr::Lambda(items, expr) => todo!(),
            Expr::List(exprs) => todo!(),
            Expr::Tuple(exprs) => todo!(),
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

    fn get_var(&self, varname: &String) -> Option<&Value> {
        self.envs.last().unwrap().resolve(varname)
    }
}
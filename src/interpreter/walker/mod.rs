pub mod types;

use types::*;

use crate::parser::{BinOp, Expr, Stmt};

struct Interpreter<'a> {
    envs: Vec<Enviroment<'a>>
}


impl Interpreter<'_> {
    pub fn interpret(&mut self, statements: Vec<Stmt>) {

    }

    pub fn interpret_single(&mut self, statement: &Stmt) {
        match statement {
            Stmt::ConstAssignment(varname, expr) => self.evaluate_const_assignment(varname, expr),
            Stmt::VarAssignment(varname, expr) => self.evaluate_var_assignment(varname, expr),
            Stmt::ExprStmt(expr) => self.evaluate_expr_stmt(&expr),
        }
    }

    fn evaluate_const_assignment(&mut self, varname: &String, expr: &Expr) {
        todo!("Const assignment not implemented");
    }
    fn evaluate_var_assignment(&mut self, varname: &String, expr: &Expr) {
        todo!("Var assignment not implemented");
    }
    fn evaluate_expr_stmt(&mut self, expr: &Expr) {
        let _ = self.evaluate_expression(expr);
    }

    fn evaluate_expression(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal(value) => todo!(),
            Expr::Variable(varname) => self.get_var(varname).expect(&format!("Variable {varname} not defined")),
            Expr::Binary(expr1, bin_op, expr2) => self.evaluate_bin_op(expr1, bin_op, expr2),
            Expr::Unary(unary_op, expr) => todo!(),
            Expr::Ternary(expr, stmts, stmts1) => todo!(),
            Expr::Call(_, exprs) => todo!(),
            Expr::Lambda(items, expr) => todo!(),
            Expr::List(exprs) => todo!(),
            Expr::Tuple(exprs) => todo!(),
        }
    }

    fn evaluate_bin_op(&mut self, expr1: &Expr, op: BinOp, expr2: &Expr) -> Value {
        match op {
            BinOp::Plus => self.evaluate_expression(expr1) + self.evaluate_expression(expr2),
            BinOp::Minus => todo!(),
            BinOp::Mult => todo!(),
            BinOp::Div => todo!(),
            BinOp::IntDiv => todo!(),
            BinOp::Mod => todo!(),
            BinOp::Pow => todo!(),
            BinOp::And => todo!(),
            BinOp::Or => todo!(),
            BinOp::DotAccess => todo!(),
            BinOp::Equal => todo!(),
            BinOp::NotEqual => todo!(),
            BinOp::Greater => todo!(),
            BinOp::GreaterOrEqual => todo!(),
            BinOp::Less => todo!(),
            BinOp::LessOrEqual => todo!(),
            BinOp::Index => todo!(),
        }
    }

    fn get_var(&self, varname: &String) -> Option<&Value> {
        self.envs.last().unwrap().resolve(varname)
    }
}
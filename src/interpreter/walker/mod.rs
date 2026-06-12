pub mod types;

use std::io::{BufRead, Write};

use types::*;

use crate::parser::{BinOp, Expr, Stmt, StringPart, UnaryOp};

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

        interp.envs[0].new_override(&"leia_numero".to_string(), |_| {
            let mut stdin = std::io::stdin().lock();
            let mut buf = String::new();
            stdin.read_line(&mut buf).expect("Failed to read from stdin");
            match buf.trim().parse() {
                Ok(num) => Value::Float(num),
                Err(e) => panic!("{e}"),
            }
        }).expect("Failed to inject \"leia_numero\" override");

        interp
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.interpret_single(&statement);
        }
    }

    pub fn interpret_single(&mut self, statement: &Stmt) -> Value {
        match statement {
            Stmt::ConstAssignment(expr) => match expr {
                Expr::Binary(expr1, binop, expr2) if *binop == BinOp::ConstAssignment => {
                    self.evaluate_assignment(expr1, expr2, TypeInfo::new(true))
                }
                Expr::Binary(_, binop, _) => {
                    panic!("const assignment had invalid operation {binop}")
                }
                _ => {
                    panic!("const assignment was not a binary expression")
                }
                
            }
            Stmt::VarAssignment(expr) => match expr {
                Expr::Binary(expr1, binop, expr2) if *binop == BinOp::VarAssignment => {
                    self.evaluate_assignment(expr1, expr2, TypeInfo::new(false))
                }
                Expr::Binary(_, binop, _) => {
                    panic!("var assignment had invalid operation {binop}")
                }
                _ => {
                    panic!("var assignment was not a binary expression")
                }
                
            },
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

    fn evaluate_assignment(&mut self, variables: &Expr, expr: &Expr, type_info: TypeInfo) -> Value {
        // verify targets are all variables
        let value = self.evaluate_expression(expr);
        //TODO: handle multiple targets
        if let Expr::Variable(varname) = variables {
            self.set_var(varname, &ValueInfo::new(value.clone(), type_info));
        }
        else {
            panic!("invalid var assignment target {:?}", variables);
        }
        value
    }

    fn evaluate_expr_stmt(&mut self, expr: &Expr) -> Value {
        self.evaluate_expression(expr)
    }

    #[allow(unused_variables)]
    fn evaluate_expression(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal(value) => value.clone().into(),
            Expr::String(parts) => self.evaluate_string(parts),
            Expr::Variable(varname) => self
                .get_var(varname)
                .unwrap_or_else(|| panic!("Variable {varname} not defined"))
                .value()
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
            Expr::While(cond, stmts) => {
                let mut result = Value::None;
                while bool::from(&self.evaluate_expression(cond)) {
                    for i in 0..stmts.len() {
                        if i == stmts.len() - 1 {
                            result = self.interpret_single(&stmts[i]);
                        } else {
                            self.interpret_single(&stmts[i]);
                        }
                    }
                }

                result
            },
            Expr::For(control_var, start, stop, step, stmts) => {
                let result = Value::None;

                let start = self.evaluate_expression(start);
                let stop = self.evaluate_expression(stop);
                let step = self.evaluate_expression(step);

                self.set_var(control_var, &ValueInfo::new(start.clone(), TypeInfo::new(false)));

                if start <= stop {
                    while *self.get_var(control_var).expect("control variable not found").value() <= stop {
                        for stmt in stmts {
                            self.interpret_single(stmt);
                        }

                        self.set_var(control_var, &ValueInfo::new(
                            self.get_var(control_var).expect("control variable not found").value().clone() + step.clone(),
                            TypeInfo::new(false)));
                    }
                } else {
                    while *self.get_var(control_var).expect("control variable not found").value() >= stop {
                        for stmt in stmts {
                            self.interpret_single(stmt);
                        }

                        self.set_var(control_var, &ValueInfo::new(
                            self.get_var(control_var).expect("control variable not found").value().clone() + step.clone(),
                            TypeInfo::new(false)));
                    }
                }

                result
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
            BinOp::Comma => todo!(),
            BinOp::ConstAssignment => self.evaluate_assignment(expr1, expr2, TypeInfo::new(true)),
            BinOp::VarAssignment => self.evaluate_assignment(expr1, expr2, TypeInfo::new(false)),
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

    fn evaluate_string(&mut self, parts: &Vec<StringPart>) -> Value {
        let mut string = String::new();

        for part in parts {
            match part {
                StringPart::Fragment(str) => string.push_str(str),
                StringPart::Expr(expr) => string.push_str(&format!("{}", &self.evaluate_expression(expr))),
            };
        }

        Value::String(string)
    }

    pub fn get_var(&self, varname: &String) -> Option<ValueInfo> {
        self.envs.last().unwrap().resolve(varname)
    }

    fn set_var(&mut self, varname: &String, value: &ValueInfo) {
        if self.get_var(varname).is_some() {
            self.envs
                .last_mut()
                .unwrap()
                .set_var(varname, value.clone());
        } else {
            self.envs.last_mut().unwrap().assign_var(varname, value.clone());
        }
    }
}

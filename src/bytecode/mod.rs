pub mod types;

pub use types::*;

use crate::parser::Stmt;

pub struct Compiler {
    bytecode: Vec<Bytecode>,
    statements: Vec<Stmt>,
}

impl Compiler {
    pub fn new(statements: Vec<Stmt>) -> Compiler {
        Compiler {
            statements,
            bytecode: vec![],
        }
    }


    pub fn compile(&mut self) -> Vec<Bytecode> {
        // traverse self.statements, generating bytecode for each of them
        for statement in self.statements.clone().iter() {
            self.compile_statement(&statement);
        }

        self.bytecode.clone()
    }

    fn compile_statement(&mut self, statement: &Stmt) {

    }

    fn emit(&mut self, bytecode: Bytecode) {
        self.bytecode.push(bytecode);
    }
}
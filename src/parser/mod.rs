pub mod types;

use std::clone;

pub use types::*;
use crate::lexer::{self, Keyword, Token};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    UnexpectedEOF,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { 
            tokens,
            pos: 0
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = vec![];
        
        while !self.is_eot() {
            stmts.push(self.parse_stmt()?);
        }
    
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.peek() {
            Some(token) => {
                match token {
                    Token::Unknown(tok) => panic!("Found unknown token {tok} while parsing"),
                    Token::Keyword(keyword) => {
                        match keyword {
                            keyword if keyword.keyword == "var" => {
                                self.expect(Token::Keyword(Keyword { keyword: "var".to_string() }))?;

                                let token = self.next().unwrap();
                                if let Token::Identifier(ident) = token {
                                    let name = ident.symbol.clone();

                                    self.expect(Token::Operation(lexer::Operation { operation: ":=".to_string()}))?;
                                    let value = self.parse_expr()?;
                                    if !self.is_eot() { self.expect(Token::NewLine)?; }

                                    return Ok(Stmt::VarAssignment(name, value))
                                };
                                return Err(ParseError::UnexpectedToken)
                            }
                            keyword => { panic!("Unknown keyword \"{keyword:?}\" found") }
                        }
                    },
                    _ => self.parse_expr_stmt()
                }
            }
            None => Err(ParseError::UnexpectedEOF)
        }
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        todo!()
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let token = self.next().unwrap();
    
        if let Token::Float(token) = token {
            return Ok(Expr::Literal(Value::Float(*token)));
        }

        Err(ParseError::UnexpectedToken)
    }

    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn expect(&mut self, token: Token) -> Result<&Token, ParseError>{
        if let Some(next_token) = self.next() {
            if token != *next_token {
                return Err(ParseError::UnexpectedToken);
            } else {
                return Ok(next_token);
            }
        }
        Err(ParseError::UnexpectedEOF)
    }

    fn consume_while<F: Fn(&Token) -> bool>(&mut self, f: F) {
        while f(self.next().unwrap()) {}
    }

    fn is_eot(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}
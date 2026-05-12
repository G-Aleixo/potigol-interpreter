pub mod types;

pub use types::*;
use crate::lexer::{Keyword, Token};

enum ParseError {
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
                    Token::Identifier(identifier) => todo!(),
                    Token::Type(_) => todo!(),
                    Token::String(_) => todo!(),
                    Token::Character(_) => todo!(),
                    Token::Integer(_) => todo!(),
                    Token::Float(_) => todo!(),
                    Token::Boolean(_) => todo!(),
                    Token::NewLine => todo!(),
                    Token::Comma => todo!(),
                    Token::Period => todo!(),
                    Token::Colon => todo!(),
                    Token::Unknown(_) => todo!(),
                    Token::Keyword(keyword) => todo!(),
                    Token::Operation(operation) => todo!(),
                    Token::BlockDelimeter(block_delimeter) => todo!(),
                }
            }
            None => Err(ParseError::UnexpectedEOF)
        }
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

    fn is_eot(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}
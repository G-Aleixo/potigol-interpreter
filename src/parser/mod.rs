pub mod types;

pub use types::*;
use crate::lexer::Token;

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
                            keyword if keyword == "var" => {
                                self.expect(Token::Keyword("var".to_string()))?;

                                let token = self.next().unwrap();
                                if let Token::Identifier(ident) = token {
                                    let name = ident.clone();

                                    self.expect(Token::Operation(":=".to_string()))?;
                                    let value = self.parse_expr()?;
                                    if !self.is_eot() { self.expect(Token::NewLine)?; }

                                    return Ok(Stmt::VarAssignment(name, value))
                                };
                                Err(ParseError::UnexpectedToken)
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
        Ok(Stmt::ExprStmt(self.expr_bp(0)?))
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let token = self.next().expect("Unexpected EOF");
    
        if let Token::Float(token) = token {
            return Ok(Expr::Literal(Value::Float(*token)));
        }

        Err(ParseError::UnexpectedToken)
    }

    fn expr_bp(&mut self, min_bp: u8) -> Result<Expr, ParseError>{
        let token = match self.next() {
            Some(tok) => tok,
            None => return Err(ParseError::UnexpectedEOF)
        };
        let mut lhs = match token {
            Token::Identifier(ident) => Expr::Variable(ident.clone()),
            Token::String(str) => Expr::Literal(Value::String(str.clone())),
            Token::Character(char) => Expr::Literal(Value::String(char.to_string())),
            Token::Integer(int) => Expr::Literal(Value::Integer(*int)),
            Token::Float(float) => Expr::Literal(Value::Float(*float)),
            Token::Boolean(bool) => Expr::Literal(Value::Boolean(*bool)),
            
            
            Token::Keyword(keyword) => {
                let kw = keyword.clone();
                let ((), r_bp) = prefix_binding_power(&kw);
                let rhs = self.expr_bp(r_bp)?;

                Expr::Unary((&kw).into(), Box::new(rhs))
            },
            Token::Operation(op) => {
                let op = op.clone();
                let ((), r_bp) = prefix_binding_power(&op);
                let rhs = self.expr_bp(r_bp)?;

                Expr::Unary((&op).into(), Box::new(rhs))
            },
            _ => return Err(ParseError::UnexpectedToken)
        };

        loop {
            let op = match self.peek() {
                Some(tok) => match tok {
                    Token::Period => ".".to_string(),
                    Token::Keyword(keyword) => keyword.clone(),
                    Token::Operation(op) => op.clone(),
                    _ => return Err(ParseError::UnexpectedToken)
                }
                None => break
            };

            let (l_bp, r_bp) = infix_binding_power(&op);

            if l_bp < min_bp {
                break;
            }

            self.next();

            let rhs = self.expr_bp(r_bp)?;

            lhs = Expr::Binary(Box::new(lhs), (&op).into(), Box::new(rhs));
        }

        Ok(lhs)
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

fn infix_binding_power(op: &str) -> (u8, u8) {
    match op {
        "ou"        => (1, 2),
        "e"         => (3, 4),
        // "não"  => ((), 5)
        "==" | "<>" |
        ">" | ">=" |
        "<" | "<=" => (7, 8),
        "+" | "-"   => (9, 10),
        "div" | "mod" |
        "*" | "/"   => (11, 12),
        // unary "+" "-" => ((), 13)
        "^"         => (16, 15),
        "."         => (18, 17),
        _ => todo!()
    }
}

fn prefix_binding_power(op: &str) -> ((), u8) {
    match op {
        "não" => ((), 5),
        "+" | "-" => ((), 13),

        op => panic!("Invalid op {op:?}")
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer;
    use super::*;

    #[test]
    #[should_panic = "Unexpected EOF"]
    fn incomplete_var_assignment() {
        let mut parser = Parser::new(lexer::tokenize("var abc := ").unwrap());
        parser.parse().unwrap();
    }

    #[test]
    fn two_plus_two() {
        let mut parser = Parser::new(lexer::tokenize("2 + 2").unwrap());

        assert_eq!(format!("{:?}", parser.parse().unwrap()[0]), "ExprStmt((+ 2 2))");
    }

    #[test]
    fn precedence() {
        let mut parser = Parser::new(lexer::tokenize("2 + 2 * 4 ^ 1.2").unwrap());

        assert_eq!(format!("{:?}", parser.parse().unwrap()[0]), "ExprStmt((+ 2 (* 2 (^ 4 1.2))))");
    }

    #[test]
    fn handed_precedence() {
        let mut parser = Parser::new(lexer::tokenize("2 * 2 * 2 + 3 ^ 2 ^ 1").unwrap());

        assert_eq!(format!("{:?}", parser.parse().unwrap()[0]), "ExprStmt((+ (* (* 2 2) 2) (^ 3 (^ 2 1))))");
    }
}
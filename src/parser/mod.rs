pub mod types;

pub use types::*;
use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token),
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
        
        while !self.is_eot() && !self.is_terminal() {
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
                                    let value = self.parse_expr(0)?;
                                    // if !self.is_eot() {
                                    //     //TODO: really fix this, check for the end of expression correctly
                                    //     if let Err(_) = self.expect(Token::NewLine) {
                                    //         self.expect(Token::Keyword("var".to_string())).unwrap();
                                    //     }
                                    // }

                                    return Ok(Stmt::VarAssignment(name, value))
                                };
                                Err(ParseError::UnexpectedToken(self.peek().unwrap_or(&Token::Unknown('~')).clone()))
                            }
                            keyword if keyword == "se" => {
                                self.parse_expr_stmt()
                            }
                            keyword if keyword == "imprima" => {
                                self.parse_write()
                            }
                            keyword if keyword == "escreva" => {
                                self.parse_print()
                            }

                            keyword => { panic!("Unknown keyword {keyword:?} found") }
                        }
                    },
                    _ => self.parse_expr_stmt()
                }
            }
            None => Err(ParseError::UnexpectedEOF)
        }
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        Ok(Stmt::ExprStmt(self.parse_expr(0)?))
    }

    fn parse_print(&mut self) -> Result<Stmt, ParseError> {
        self.expect(Token::Keyword("escreva".to_string()))?;

        let expr = self.parse_expr(0)?;

        Ok(Stmt::PrintStatement(expr))
    }

    fn parse_write(&mut self) -> Result<Stmt, ParseError> {
        self.expect(Token::Keyword("imprima".to_string()))?;

        let expr = self.parse_expr(0)?;

        Ok(Stmt::WriteStatement(expr))
    }

    fn parse_expr(&mut self, min_bp: u8) -> Result<Expr, ParseError>{
        let mut token = match self.next() {
            Some(tok) => tok,
            None => return Err(ParseError::UnexpectedEOF)
        };

        while *token == Token::NewLine {
            token = match self.next() {
                Some(tok) => tok,
                None => return Err(ParseError::UnexpectedEOF)
            };
        }

        let mut lhs = match token {
            Token::Identifier(ident) => Expr::Variable(ident.clone()),
            Token::String(str) => Expr::Literal(Value::String(str.clone())),
            Token::Character(char) => Expr::Literal(Value::String(char.to_string())),
            Token::Integer(int) => Expr::Literal(Value::Integer(*int)),
            Token::Float(float) => Expr::Literal(Value::Float(*float)),
            Token::Boolean(bool) => Expr::Literal(Value::Boolean(*bool)),
            
            Token::BlockDelimeter(block, false) if block == "(" => {
                let lhs = self.parse_expr(0)?;
                self.expect(Token::BlockDelimeter(")".to_string(), true))?;
                lhs
            }

            Token::Keyword(keyword) if keyword == "se" => {
                let cond = self.parse_expr(0)?;
                self.expect(Token::Keyword("então".to_string()))?;
                let then_stmts = self.parse_block_until_keyword(&["senãose", "senão", "fim"])?;

                let mut elifs: Vec<(Expr, Vec<Stmt>)> = Vec::new();
                while self.check_keyword("senãose") {
                    self.next(); // consume 'elif'
                    let elif_cond = self.parse_expr(0)?;
                    self.expect(Token::Keyword("então".to_string()))?;
                    let elif_then = self.parse_block_until_keyword(&["senãose", "senão", "fim"])?;
                    elifs.push((elif_cond, elif_then));
                }

                let mut else_stmts: Vec<Stmt> = if self.check_keyword("senão") {
                    self.next();
                    self.parse_block_until_keyword(&["fim"])?
                } else {
                    Vec::new()
                };

                self.expect(Token::Keyword("fim".to_string()))?;

                // desugar elif chain
                for (elif_cond, elif_then) in elifs.into_iter().rev() {
                    let nested_if = Expr::Ternary(Box::new(elif_cond), elif_then, else_stmts);
                    else_stmts = vec![Stmt::ExprStmt(nested_if)];
                }

                Expr::Ternary(Box::new(cond), then_stmts, else_stmts)
            }
            
            Token::Keyword(keyword) => {
                let kw = keyword.clone();
                let ((), r_bp) = prefix_binding_power(&kw);
                let rhs = self.parse_expr(r_bp)?;

                Expr::Unary((&kw).into(), Box::new(rhs))
            },
            Token::Operation(op) => {
                let op = op.clone();
                let ((), r_bp) = prefix_binding_power(&op);
                let rhs = self.parse_expr(r_bp)?;

                Expr::Unary((&op).into(), Box::new(rhs))
            },
            tok => return Err(ParseError::UnexpectedToken(tok.clone()))
        };

        while let Some(tok) = self.peek() {
            let op = match tok {
                Token::Period => ".".to_string(),
                Token::Keyword(keyword) => keyword.clone(),
                Token::Operation(op) => op.clone(),
                Token::BlockDelimeter(block, _) => block.clone(),
                Token::NewLine => {self.next(); continue},
                tok => return Err(ParseError::UnexpectedToken(tok.clone()))
            };


            if let Some((l_bp, ())) = postfix_binding_power(&op) {
                if l_bp < min_bp {
                    break;
                }

                self.next();

                lhs = if op == "[" {
                    let rhs = self.parse_expr(0)?;
                    self.expect(Token::BlockDelimeter("]".to_owned(), true))?;
                    Expr::Binary(Box::new(lhs), (&op).into(), Box::new(rhs))
                } else {
                    Expr::Unary((&op).into(), Box::new(lhs))
                };
                continue;
            }

            if let Some((l_bp, r_bp)) = infix_binding_power(&op) {
                if l_bp < min_bp {
                    break;
                }

                self.next();

                let rhs = self.parse_expr(r_bp)?;

                lhs = Expr::Binary(Box::new(lhs), (&op).into(), Box::new(rhs));
                continue;
            }

            break;
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
                return Err(ParseError::UnexpectedToken(next_token.clone()));
            } else {
                return Ok(next_token);
            }
        }
        Err(ParseError::UnexpectedEOF)
    }

    // fn consume_while<F: Fn(&Token) -> bool>(&mut self, f: F) {
    //     while f(self.next().unwrap()) {}
    // }

    fn is_terminal(&self) -> bool {
        self.is_eot() | match self.peek().unwrap() {
            Token::Keyword(keyword) => matches!(keyword.as_ref(),
                "senão" |
                "senãose" |
                "fim"
            ),
            _ => false
        }
    }

    fn is_eot(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn parse_block_until_keyword(&mut self, terminators: &[&str]) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        while let Some(tok) = self.peek() {
            // stop if next token is any of the terminators
            if let Token::Keyword(k) = tok &&
                terminators.iter().any(|t| t == k) {
                    break;
            }
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    // --- new helper: check next token is a specific keyword (by string) ---
    fn check_keyword(&self, kw: &str) -> bool {
        matches!(self.peek(), Some(Token::Keyword(k)) if k == kw)
    }
}

fn infix_binding_power(op: &str) -> Option<(u8, u8)> {
    Some(match op {
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
        // index "[" => (17, ())
        "."         => (20, 19),
        _ => { return None }
    })
}

fn prefix_binding_power(op: &str) -> ((), u8) {
    match op {
        "não" => ((), 5),
        "+" | "-" => ((), 13),

        op => panic!("Invalid op {op:?}")
    }
}

fn postfix_binding_power(op: &str) -> Option<(u8, ())> {
    Some(match op {
        "[" => (17, ()),
        _ => {return None}
    })
}

#[cfg(test)]
mod tests {
    use crate::lexer;
    use super::*;

    #[test]
    fn incomplete_var_assignment() {
        let mut parser = Parser::new(lexer::tokenize("var abc := ").unwrap());
        let res = parser.parse();
        assert!(res.is_err());
        assert_eq!(res.err().unwrap(), ParseError::UnexpectedEOF);
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

    #[test]
    fn parenthesis() {
        let mut parser = Parser::new(lexer::tokenize("7.2 * (2 - 6)").unwrap());

        assert_eq!(format!("{:?}", parser.parse().unwrap()[0]), "ExprStmt((* 7.2 (- 2 6)))");
    }

    #[test]
    fn prefix() {
        let mut parser = Parser::new(lexer::tokenize("-2 ^ 3").unwrap());

        assert_eq!(format!("{:?}", parser.parse().unwrap()[0]), "ExprStmt((- (^ 2 3)))");
    }

    #[test]
    fn postfix() {
        let mut parser = Parser::new(lexer::tokenize("-2 + arr[1]").unwrap());

        assert_eq!(format!("{:?}", parser.parse().unwrap()[0]), "ExprStmt((+ (- 2) ([ arr 1)))");
    }
}
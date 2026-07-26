pub mod types;

use winnow::{ModalResult, Parser, Result, Stateful, ascii::multispace0, combinator::{Postfix, Prefix, alt, cut_err, delimited, dispatch, empty, expression, fail, peek, preceded, repeat}, error::{ContextError, ErrMode}, stream::TokenSlice, token::{any, one_of, take_while}};

use crate::lexer::Token;
pub use types::*;

#[derive(Debug)]
pub struct State {

}

impl Default for State {
    fn default() -> Self {
        State {

        }
    }
}

impl State {

}

type Stream<'is> = TokenSlice<'is, Token<'is>>; 

pub fn new_stream<'s>(input: &'s [Token<'s>]) -> Stream<'s> {
    TokenSlice::new(input)
}

fn newline<'s>(input: &mut Stream<'s>) -> Result<(), ErrMode<ContextError>> {
    one_of(Token::NewLine).map(|_| ()).parse_next(input)
}

fn ws0<'s>(input: &mut Stream<'s>) -> Result<&'s [Token<'s>], ErrMode<ContextError>> {
    take_while(0.., |t: &Token<'_>| *t == Token::NewLine).parse_next(input)
}

fn literal<'s>(input: &mut Stream<'s>) -> Result<Expr<'s>, ErrMode<ContextError>> {
    dispatch!(any;
        &Token::Boolean(b) => empty.value(Expr::Literal(Value::Boolean(b))),
        &Token::Integer(n) => empty.value(Expr::Literal(Value::Integer(n))),
        &Token::Float(f) => empty.value(Expr::Literal(Value::Float(f))),
        _ => fail
    )
    .parse_next(input)
}

fn string_fragment<'s>(input: &mut Stream<'s>) -> Result<StringPart<'s>, ErrMode<ContextError>> {
    any
    .verify_map(|t| match t {
        &Token::StringFragment(str) => Some(StringPart::Fragment(str)),
        _ => None
    })
    .parse_next(input)
}

fn string_part<'s>(input: &mut Stream<'s>) -> Result<StringPart<'s>, ErrMode<ContextError>> {
    alt((
        string_fragment,
        delimited(
            one_of(Token::ExprStart),
            expr,
            one_of(Token::ExprEnd)
        ).map(StringPart::Expr)
    )).parse_next(input)
}

fn string<'s>(input: &mut Stream<'s>) -> Result<Expr<'s>, ErrMode<ContextError>> {
    delimited(
        one_of(Token::StringStart),
        repeat(0.., string_part),
        one_of(Token::StringEnd)
    )
    .map(Expr::String)
    .parse_next(input)
}

fn unary_operator<'s>(input: &mut Stream<'s>) -> Result<UnaryOp, ErrMode<ContextError>> {
    dispatch!(any;
        &Token::Operation(op) => empty.value(op.try_into().map_err(|_| ErrMode::Backtrack(ContextError::new()))?),
        _ => fail
    )
    .parse_next(input)
}

fn binary_operator<'s>(input: &mut Stream<'s>) -> Result<BinOp, ErrMode<ContextError>> {
    dispatch!(any;
        &Token::Operation(op) => empty.value(op.try_into().map_err(|_| ErrMode::Backtrack(ContextError::new()))?),
        _ => fail
    )
    .parse_next(input)
}

fn identifier<'s>(input: &mut Stream<'s>) -> Result<Expr<'s>, ErrMode<ContextError>> {
    dispatch!(any;
        &Token::Identifier(id) => empty.value(Expr::Identifier(id)),
        _ => fail
    )
    .parse_next(input)
}

fn tok(expected: Token) -> impl Parser<TokenSlice<'_, Token<'_>>, Token, ErrMode<ContextError>> {
    any.verify_map(move |t: &Token| {
        if *t == expected {
            Some(expected)
        } else {
            None
        }
    })
}

fn expr<'s>(input: &mut Stream<'s>) -> ModalResult<Expr<'s>, > {
    fn parser<'s>(precedence: i64) -> impl Parser<Stream<'s>, Expr<'s>, ErrMode<ContextError>> {
        move |i: &mut Stream<'s>| {
            use winnow::combinator::Infix::{Left, Neither, Right};
            expression(
                delimited(
                    ws0,
                    dispatch! { peek(any);
                        &Token::BlockDelimeter('(', false) => delimited(one_of(&Token::BlockDelimeter('(', false)), parser(0), cut_err(one_of(&Token::BlockDelimeter(')', true)))),
                        _ => alt((
                            identifier,
                            literal,
                            string
                            // add other expressions here
                        ))
                    },
                    ws0
                )
            )
            .current_precedence_level(precedence)
            .prefix(
                delimited(
                    ws0,
                    dispatch! {unary_operator;
                        UnaryOp::Plus => Prefix(15, |_: &mut _, a| Ok(Expr::Unary { op: UnaryOp::Plus, expr: Box::new(a)})),
                        UnaryOp::Minus => Prefix(15, |_: &mut _, a| Ok(Expr::Unary { op: UnaryOp::Minus, expr: Box::new(a)})),
                        UnaryOp::Write => Prefix(2, |_: &mut _, a| Ok(Expr::Unary { op: UnaryOp::Write, expr: Box::new(a)})),
                        UnaryOp::Print => Prefix(15, |_: &mut _, a| Ok(Expr::Unary { op: UnaryOp::Print, expr: Box::new(a)})),
                        UnaryOp::Not => Prefix(7, |_: &mut _, a| Ok(Expr::Unary { op: UnaryOp::Not, expr: Box::new(a)})),
                        _ => fail
                    },
                    ws0
                )
            )
            .postfix(delimited(
                ws0,
                    dispatch! {binary_operator;
                        BinOp::Index => Postfix(19, |i: &mut Stream<'s>, a| {
                            let index = delimited(ws0, parser(0), (ws0, cut_err(one_of(Token::BlockDelimeter(']', true))), ws0)).parse_next(i)?;
                            Ok(Expr::Binary { op: BinOp::Index, lhs: Box::new(a), rhs: Box::new(index)})
                        }),
                        _ => fail
                    },
                    ws0
                )
            )
            .infix(
                dispatch! { binary_operator; 
                    BinOp::DotAccess => Right(21, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::DotAccess, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::Pow => Right(17, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::Pow, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::IntDiv => Left(13, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::IntDiv, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::Mod => Left(13, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::Mod, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::Mult => Left(13, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::Mult, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::Div => Left(13, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::Div, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::Plus => Left(11, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::Plus, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::Minus => Left(11, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::Minus, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::Equal => Neither(9, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::Equal, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::NotEqual => Neither(9, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::NotEqual, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::Greater => Neither(9, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::Greater, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::GreaterOrEqual => Neither(9, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::GreaterOrEqual, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::Less => Neither(9, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::Less, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::LessOrEqual => Neither(9, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::LessOrEqual, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::And => Left(5, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::And, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::Or => Left(3, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::Or, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::VarAssignment => Left(0, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::VarAssignment, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    BinOp::ConstAssignment => Left(0, |_: &mut _, a, b| {
                        Ok(Expr::Binary { op: BinOp::ConstAssignment, lhs: Box::new(a), rhs: Box::new(b)})   
                    }),
                    _ => fail
                }
            )
            .parse_next(i)
        }
    }

    parser(0).parse_next(input)
}

fn stmt<'s>(input: &mut Stream<'s>) -> Result<Stmt<'s>, ErrMode<ContextError>> {
    alt((
        // define suitable targets later
        preceded(tok(Token::Keyword("var")), expr)
        .verify_map(|expr| match expr {
            Expr::Binary { lhs, op, rhs } if op == BinOp::VarAssignment => {
                Some(Stmt::VarAssignment(Expr::Binary { lhs, op, rhs }))
            },
            _ => None
        }),
        peek(expr).verify_map(|expr| match expr {
            Expr::Binary { lhs, op, rhs } if op == BinOp::ConstAssignment => {
                Some(Stmt::ConstAssignment(Expr::Binary { lhs, op, rhs }))
            },
            _ => None
        }),
        expr.map(|expr| Stmt::ExprStmt(expr)),
        cut_err(fail)
    )).parse_next(input)
}

pub fn parse<'s>(input: &mut Stream<'s>) -> Result<Vec<Stmt<'s>>, ErrMode<ContextError>> {
    repeat(.., delimited(ws0, stmt, ws0))
    .parse_next(input)
}

#[cfg(test)]
pub mod tests {
    use winnow::Parser;

use crate::{lexer::tokenize, parser::{Expr, StringPart, expr, new_stream, parse, string}};

    #[test]
    fn aa() {
        let mut input = include_str!("../../test.poti");

        let out1 = tokenize(&mut crate::lexer::new_stream(&mut input)).unwrap();
    
        let mut new_stream = new_stream(&out1[..]);
        let lit = parse.parse_next(&mut new_stream);

        println!("{new_stream:?}");
        assert_eq!(lit, Ok(vec![]));
    }
}

// #[derive(Debug, PartialEq)]
// pub enum ParseError {
//     UnexpectedToken(Token),
//     UnexpectedEOF,
//     EmptyStmt,
// }

// pub struct Parser {
//     tokens: Vec<Token>,
//     pos: usize,
// }

// impl Parser {
//     pub fn new(tokens: Vec<Token>) -> Self {
//         Self { tokens, pos: 0 }
//     }

//     pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
//         let mut stmts = vec![];

//         while !self.is_eot() && !self.is_terminal() {
//             stmts.push(self.parse_stmt()?);
//         }

//         Ok(stmts)
//     }

//     fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
//         match self.peek() {
//             Some(token) => {
//                 match token {
//                     Token::Unknown(tok) => panic!("Found unknown token {tok} while parsing"),
//                     Token::NewLine => {
//                         self.next();
//                         self.parse_stmt()
//                     }
//                     Token::Keyword(keyword) => {
//                         match keyword {
//                             keyword if keyword == "var" => {
//                                 self.expect(Token::Keyword("var".to_string()))?;

//                                 let token = self.peek().unwrap();
//                                 if let Token::Identifier(_) = token {
//                                     let expr = self.parse_expr(0)?;

//                                     return Ok(Stmt::VarAssignment(expr));
//                                 };
//                                 Err(ParseError::UnexpectedToken(token.clone()))
//                             }
//                             keyword if keyword == "se" => self.parse_expr_stmt(),

//                             keyword if keyword == "enquanto" => self.parse_expr_stmt(),

//                             keyword if keyword == "para" => self.parse_expr_stmt(),

//                             keyword if keyword == "imprima" => self.parse_expr_stmt(),

//                             keyword if keyword == "escreva" => self.parse_expr_stmt(),

//                             keyword => {
//                                 panic!("Unknown keyword {keyword:?} found")
//                             }
//                         }
//                     }
//                     _ => self.parse_expr_stmt(),
//                 }
//             }
//             None => Err(ParseError::UnexpectedEOF),
//         }
//     }

//     fn parse_expr_stmt(&mut self) -> Result<Stmt, ParseError> {
//         Ok(Stmt::ExprStmt(self.parse_expr(0)?))
//     }

//     fn parse_expr(&mut self, min_bp: u8) -> Result<Expr, ParseError> {
//         let mut token = match self.next() {
//             Some(tok) => tok,
//             None => return Err(ParseError::UnexpectedEOF),
//         };

//         while *token == Token::NewLine {
//             token = match self.next() {
//                 Some(tok) => tok,
//                 None => return Err(ParseError::UnexpectedEOF),
//             };
//         }

//         let mut lhs = match token {
//             Token::Identifier(ident) => Expr::Variable(ident.clone()),
//             Token::StringStart => { self.parse_fstring()? },
//             Token::Character(char) => Expr::String(vec![StringPart::Fragment(char.to_string())]),
//             Token::Integer(int) => Expr::Literal(Value::Integer(*int)),
//             Token::Float(float) => Expr::Literal(Value::Float(*float)),
//             Token::Boolean(bool) => Expr::Literal(Value::Boolean(*bool)),

//             Token::BlockDelimeter(block, false) if block == "(" => {
//                 let lhs = self.parse_expr(0)?;
//                 self.expect(Token::BlockDelimeter(")".to_string(), true))?;
//                 lhs
//             }

//             Token::BlockDelimeter(block, false) if block == "[" => {
//                 let lhs = Expr::List(self.parse_comma_separated()?);
//                 self.expect(Token::BlockDelimeter("]".to_string(), true))?;
//                 lhs
//             }

//             Token::Keyword(keyword) if keyword == "se" => {
//                 let cond = self.parse_expr(0)?;
//                 self.expect(Token::Keyword("então".to_string()))?;
//                 let then_stmts = self.parse_block_until_keyword(&["senãose", "senão", "fim"])?;

//                 let mut elifs: Vec<(Expr, Vec<Stmt>)> = Vec::new();
//                 while self.check_keyword("senãose") {
//                     self.next(); // consume 'elif'
//                     let elif_cond = self.parse_expr(0)?;
//                     self.expect(Token::Keyword("então".to_string()))?;
//                     let elif_then = self.parse_block_until_keyword(&["senãose", "senão", "fim"])?;
//                     elifs.push((elif_cond, elif_then));
//                 }

//                 let mut else_stmts: Vec<Stmt> = if self.check_keyword("senão") {
//                     self.next();
//                     self.parse_block_until_keyword(&["fim"])?
//                 } else {
//                     Vec::new()
//                 };

//                 self.expect(Token::Keyword("fim".to_string()))?;

//                 // desugar elif chain
//                 for (elif_cond, elif_then) in elifs.into_iter().rev() {
//                     let nested_if = Expr::Ternary(Box::new(elif_cond), elif_then, else_stmts);
//                     else_stmts = vec![Stmt::ExprStmt(nested_if)];
//                 }

//                 Expr::Ternary(Box::new(cond), then_stmts, else_stmts)
//             }

//             Token::Keyword(keyword) if keyword == "enquanto" => {
//                 let cond = self.parse_expr(0)?;
//                 self.expect(Token::Keyword("faça".to_string()))?;
//                 let stmts = self.parse_block_until_keyword(&["fim"])?;
//                 self.expect(Token::Keyword("fim".to_string()))?;
//                 Expr::While(Box::new(cond), stmts)
//             }

//             Token::Keyword(keyword) if keyword == "para" => {
//                 let control_var = self.next().unwrap();

//                 if let Token::Identifier(control_var) = control_var.clone() {
//                     self.expect(Token::Keyword("de".to_string()))?;
//                     let start = self.parse_expr(0)?;
//                     self.expect(Token::Keyword("até".to_string()))?;
//                     let end = self.parse_expr(0)?;
//                     let mut step = Expr::Literal(Value::Integer(1));
//                     if self.peek() == Some(&Token::Keyword("para".to_string())) {
//                         self.expect(Token::Keyword("para".to_string()))?;
//                         step = self.parse_expr(0)?;
//                     }

//                     self.expect(Token::Keyword("faça".to_string()))?;
//                     let stmts = self.parse_block_until_keyword(&["fim"])?;
//                     self.expect(Token::Keyword("fim".to_string()))?;

//                     Expr::For(control_var.clone(), Box::new(start), Box::new(end), Box::new(step), stmts)
//                 } else {
//                     return Err(ParseError::UnexpectedToken(control_var.clone()))
//                 }
//             }

//             Token::Keyword(keyword) => {
//                 let kw = keyword.clone();
//                 if matches!(kw.as_ref(), "var") {
                    
//                 };
//                 let ((), r_bp) = prefix_binding_power(&kw);
//                 let rhs = self.parse_expr(r_bp)?;

//                 Expr::Unary((&kw).into(), Box::new(rhs))
//             }
//             Token::Operation(op) => {
//                 let op = op.clone();
//                 let ((), r_bp) = prefix_binding_power(&op);
//                 let rhs = self.parse_expr(r_bp)?;

//                 Expr::Unary((&op).into(), Box::new(rhs))
//             }
//             tok => return Err(ParseError::UnexpectedToken(tok.clone())),
//         };

//         while let Some(tok) = self.peek() {
//             let op = match tok {
//                 Token::Period => ".".to_string(),
//                 Token::Keyword(keyword) => keyword.clone(),
//                 Token::Comma => break,
//                 Token::Operation(op) => op.clone(),
//                 Token::BlockDelimeter(block, _) => block.clone(),
//                 Token::NewLine => {
//                     self.next();
//                     continue;
//                 }
//                 Token::ExprEnd => {
//                     break
//                 }
//                 Token::Identifier(_) => {
//                     // expression parsing should be done
//                     break
//                 }
//                 tok => return Err(ParseError::UnexpectedToken(tok.clone())),
//             };

//             if let Some((l_bp, ())) = postfix_binding_power(&op) {
//                 if l_bp < min_bp {
//                     break;
//                 }

//                 self.next();

//                 lhs = if op == "[" {
//                     let rhs = self.parse_expr(0)?;
//                     self.expect(Token::BlockDelimeter("]".to_owned(), true))?;
//                     Expr::Binary(Box::new(lhs), (&op).into(), Box::new(rhs))
//                 }
//                 else {
//                     Expr::Unary((&op).into(), Box::new(lhs))
//                 };
//                 continue;
//             }

//             if let Some((l_bp, r_bp)) = infix_binding_power(&op) {
//                 if l_bp < min_bp {
//                     break;
//                 }

//                 self.next();

//                 let rhs = self.parse_expr(r_bp)?;

//                 lhs = Expr::Binary(Box::new(lhs), (&op).into(), Box::new(rhs));
//                 continue;
//             }

//             break;
//         }

//         Ok(lhs)
//     }

//     fn next(&mut self) -> Option<&Token> {
//         let token = self.tokens.get(self.pos);
//         if token.is_some() {
//             self.pos += 1;
//         }
//         token
//     }

//     fn peek(&self) -> Option<&Token> {
//         self.tokens.get(self.pos)
//     }

//     fn expect(&mut self, token: Token) -> Result<&Token, ParseError> {
//         if let Some(next_token) = self.next() {
//             if token != *next_token {
//                 return Err(ParseError::UnexpectedToken(next_token.clone()));
//             } else {
//                 return Ok(next_token);
//             }
//         }
//         Err(ParseError::UnexpectedEOF)
//     }

//     // fn consume_while<F: Fn(&Token) -> bool>(&mut self, f: F) {
//     //     while f(self.next().unwrap()) {}
//     // }

//     fn is_terminal(&self) -> bool {
//         self.is_eot()
//             | match self.peek().unwrap() {
//                 Token::Keyword(keyword) => matches!(keyword.as_ref(), "senão" | "senãose" | "fim"),
//                 _ => false,
//             }
//     }

//     fn is_eot(&self) -> bool {
//         self.pos >= self.tokens.len()
//     }

//     fn parse_block_until_keyword(&mut self, terminators: &[&str]) -> Result<Vec<Stmt>, ParseError> {
//         let mut stmts = Vec::new();
//         while let Some(tok) = self.peek() {
//             // stop if next token is any of the terminators
//             if let Token::Keyword(k) = tok
//                 && terminators.iter().any(|t| t == k)
//             {
//                 break;
//             }
//             stmts.push(self.parse_stmt()?);
//         }
//         Ok(stmts)
//     }

//     fn parse_comma_separated(&mut self) -> Result<Vec<Expr>, ParseError> {
//         let mut exprs = Vec::new();
//         while let Some(tok) = self.peek() {
//             if let Token::BlockDelimeter(_, is_close) = tok && *is_close {
//                 break
//             };
//             exprs.push(self.parse_expr(0)?);
//         }

//         Ok(exprs)
//     }

//     fn parse_fstring(&mut self) -> Result<Expr, ParseError> {
//         let mut parts = vec![];

//         // already past the string start
//         while let Some(tok) = self.peek() {
//             match tok {
//                 Token::StringEnd => break,
//                 Token::StringFragment(str) => {
//                     let str = str.clone();
//                     self.next();
//                     parts.push(StringPart::Fragment(str));
//                 },
//                 Token::ExprStart => {
//                     self.next();
//                     let expr = self.parse_expr(0)?;
//                     self.expect(Token::ExprEnd)?;
//                     parts.push(StringPart::Expr(expr));
//                 },
//                 _ => return Err(ParseError::UnexpectedToken(tok.clone()))
//             }
//         }

//         self.expect(Token::StringEnd)?;

//         Ok(Expr::String(parts))
//     }

//     // --- new helper: check next token is a specific keyword (by string) ---
//     fn check_keyword(&self, kw: &str) -> bool {
//         matches!(self.peek(), Some(Token::Keyword(k)) if k == kw)
//     }
// }

// fn infix_binding_power(op: &str) -> Option<(u8, u8)> {
//     Some(match op {
//         ":=" | "=" => (0, 1),
//         "ou" => (3, 4),
//         "e" => (5, 6),
//         // "não"  => ((), 7)
//         "==" | "<>" | ">" | ">=" | "<" | "<=" => (9, 10),
//         "+" | "-" => (11, 12),
//         "div" | "mod" | "*" | "/" => (13, 14),
//         // unary "+" "-" => ((), 15)
//         "^" => (18, 17),
//         // index "[" => (19, ())
//         "." => (22, 21),
//         _ => return None,
//     })
// }

// fn prefix_binding_power(op: &str) -> ((), u8) {
//     match op {
//         "imprima" | "escreva" => ((), 2),
//         "não" => ((), 7),
//         "+" | "-" => ((), 15),

//         op => panic!("Invalid op {op:?}"),
//     }
// }

// fn postfix_binding_power(op: &str) -> Option<(u8, ())> {
//     Some(match op {
//         "[" => (19, ()),
//         _ => return None,
//     })
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::lexer;

//     #[test]
//     fn incomplete_var_assignment() {
//         let mut parser = Parser::new(lexer::tokenize("var abc := ").unwrap());
//         let res = parser.parse();
//         assert!(res.is_err());
//         assert_eq!(res.err().unwrap(), ParseError::UnexpectedEOF);
//     }

//     #[test]
//     fn two_plus_two() {
//         let mut parser = Parser::new(lexer::tokenize("2 + 2").unwrap());

//         assert_eq!(
//             format!("{:?}", parser.parse().unwrap()[0]),
//             "ExprStmt((+ 2 2))"
//         );
//     }

//     #[test]
//     fn precedence() {
//         let mut parser = Parser::new(lexer::tokenize("2 + 2 * 4 ^ 1.2").unwrap());

//         assert_eq!(
//             format!("{:?}", parser.parse().unwrap()[0]),
//             "ExprStmt((+ 2 (* 2 (^ 4 1.2))))"
//         );
//     }

//     #[test]
//     fn handed_precedence() {
//         let mut parser = Parser::new(lexer::tokenize("2 * 2 * 2 + 3 ^ 2 ^ 1").unwrap());

//         assert_eq!(
//             format!("{:?}", parser.parse().unwrap()[0]),
//             "ExprStmt((+ (* (* 2 2) 2) (^ 3 (^ 2 1))))"
//         );
//     }

//     #[test]
//     fn parenthesis() {
//         let mut parser = Parser::new(lexer::tokenize("7.2 * (2 - 6)").unwrap());

//         assert_eq!(
//             format!("{:?}", parser.parse().unwrap()[0]),
//             "ExprStmt((* 7.2 (- 2 6)))"
//         );
//     }

//     #[test]
//     fn prefix() {
//         let mut parser = Parser::new(lexer::tokenize("-2 ^ 3").unwrap());

//         assert_eq!(
//             format!("{:?}", parser.parse().unwrap()[0]),
//             "ExprStmt((- (^ 2 3)))"
//         );
//     }

//     #[test]
//     fn postfix() {
//         let mut parser = Parser::new(lexer::tokenize("-2 + arr[1]").unwrap());

//         assert_eq!(
//             format!("{:?}", parser.parse().unwrap()[0]),
//             "ExprStmt((+ (- 2) ([ arr 1)))"
//         );
//     }
// }

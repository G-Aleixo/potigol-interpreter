pub mod types;

use winnow::{ModalResult, Parser, Result, combinator::{Postfix, Prefix, alt, cut_err, delimited, dispatch, empty, expression, fail, opt, peek, preceded, repeat, separated, separated_pair, seq}, error::{ContextError, ErrMode}, stream::TokenSlice, token::{any, one_of, take_while}};

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

// fn newline<'s>(input: &mut Stream<'s>) -> Result<(), ErrMode<ContextError>> {
//     one_of(Token::NewLine).map(|_| ()).parse_next(input)
// }

fn ws0<'s>(input: &mut Stream<'s>) -> Result<&'s [Token<'s>], ErrMode<ContextError>> {
    take_while(0.., |t: &Token<'_>| *t == Token::NewLine).parse_next(input)
}

fn literal<'s>(input: &mut Stream<'s>) -> Result<Expr<'s>, ErrMode<ContextError>> {
    dispatch!(any;
        &Token::Boolean(b) => empty.value(Expr::Literal(Value::Boolean(b))),
        &Token::Integer(n) => empty.value(Expr::Literal(Value::Integer(n))),
        &Token::Float(f) => empty.value(Expr::Literal(Value::Float(f))),
        &Token::Character(c) => empty.value(Expr::Literal(Value::Character(c))),
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

fn ptype<'s>(input: &mut Stream<'s>) -> Result<Expr<'s>, ErrMode<ContextError>> {
    //TODO: also parse identifiers as well, as they may be types
    // decide to use stateful or not 
    dispatch!(any;
        &Token::Type(r#type) => empty.value(Expr::Identifier(r#type)),
        _ => fail
    )
    .parse_next(input)
}

fn tok<'s>(expected: Token<'s>) -> impl Parser<TokenSlice<'s, Token<'s>>, Token<'s>, ErrMode<ContextError>> {
    any.verify_map(move |t: &Token| {
        if *t == expected {
            Some(expected)
        } else {
            None
        }
    })
}


fn if_expr<'s>(input: &mut Stream<'s>) -> Result<Expr<'s>, ErrMode<ContextError>> {
    seq!(
        _: tok(Token::Keyword("se")),
        expr,
        _: tok(Token::Keyword("entao")),
        parse,
        opt(repeat(0.., seq!(
            _: tok(Token::Keyword("senaose")),
            expr,
            _: tok(Token::Keyword("entao")),
            parse
        ))),
        opt(preceded(
            tok(Token::Keyword("senao")),
            parse
        )),
        _: tok(Token::Keyword("fim"))
    )
    .map(|(cond, true_branch, elseif_branches, else_branch): (Expr<'_>, Vec<Stmt<'_>>, Option<Vec<(Expr<'_>, Vec<Stmt<'_>>)>>, Option<Vec<Stmt<'_>>>)| {
        let mut else_block = else_branch.unwrap_or(vec![]);

        // desugar elseif chain
        if let Some(elseif_branches) = elseif_branches {
            for (cond, then) in elseif_branches.into_iter().rev() {
                else_block = vec![
                    Stmt::ExprStmt(
                        Expr::Ternary {
                            cond: Box::new(cond),
                            if_true: then,
                            if_false: else_block
                        }
                    )
                ]
            }
        }

        Expr::Ternary {
            cond: Box::new(cond),
            if_true: true_branch,
            if_false: else_block
        }
    })
    .parse_next(input)
}

fn while_expr<'s>(input: &mut Stream<'s>) -> Result<Expr<'s>, ErrMode<ContextError>> {
    seq!(
        _: tok(Token::Keyword("enquanto")),
        expr,
        _: tok(Token::Keyword("faça")),
        parse,
        _: tok(Token::Keyword("fim"))
    )
    .map(|(cond, stmts)| {
        Expr::While {
            cond: Box::new(cond),
            stmts
        }
    })
    .parse_next(input)
}

fn range_for<'s>(input: &mut Stream<'s>) -> Result<Expr<'s>, ErrMode<ContextError>> {
    seq!(
        _: tok(Token::Keyword("para")),
        separated(1.., seq!(
            identifier,
            _: tok(Token::Keyword("de")),
            expr,
            _: tok(Token::Keyword("até")),
            expr,
            opt(preceded(
                tok(Token::Keyword("passo")),
                expr
            )),
        ), tok(Token::Comma)),
        _: tok(Token::Keyword("faça")),
        parse,
        _:tok(Token::Keyword("fim")),
    )
    .map(|mut forl: (Vec<_>, Vec<Stmt<'_>>)| {
        let innermost = forl.0.pop().unwrap();
        let mut for_loop = Expr::RangeFor {
            control: Box::new(innermost.0),
            start: Box::new(innermost.1),
            end: Box::new(innermost.2),
            step: innermost.3.map(|v| Box::new(v)),
            stmts: forl.1
        };

        while let Some(inner) = forl.0.pop() {
            for_loop = Expr::RangeFor {
                control: Box::new(inner.0),
                start: Box::new(inner.1),
                end: Box::new(inner.2),
                step: inner.3.map(|v| Box::new(v)),
                stmts: match for_loop {
                    Expr::RangeFor { control: _, start: _, end: _, step: _, stmts } => stmts,
                    _ => panic!("stupid type system")
                }
            };
        };

        for_loop
    })
    .parse_next(input)
}

fn inline_function_body<'s>(input: &mut Stream<'s>) -> Result<Expr<'s>, ErrMode<ContextError>> {
    preceded(
        tok(Token::Operation("=")),
        expr
    ).parse_next(input)
}

fn function_body<'s>(input: &mut Stream<'s>) -> Result<Vec<Stmt<'s>>, ErrMode<ContextError>> {
    delimited(
        ws0,
        parse,
        preceded(
            ws0,
            cut_err(tok(Token::Keyword("fim")))
        )
    ).parse_next(input)
}

fn function<'s>(input: &mut Stream<'s>) -> Result<Stmt<'s>, ErrMode<ContextError>> {
    seq!(
        _: identifier,
        delimited(
            tok(Token::BlockDelimeter('(', false)),
            repeat(..,
                separated_pair(
                    identifier,
                    tok(Token::Colon),
                    alt((
                        identifier,
                        ptype
                    ))
                )
            ),
            cut_err(tok(Token::BlockDelimeter(')', true)))
        ),
        opt(
            preceded(
                tok(Token::Colon),
                alt((
                    identifier,
                    ptype
                ))
            )
        ),
        alt((
            inline_function_body.map(|expr| vec![Stmt::ExprStmt(expr)]),
            function_body
        ))

    )
    .map(|func: (Vec<_>, _, _)| {
        Stmt::FunctionDeclaration {
            variables: func.0,
            return_type: func.1,
            body: func.2
        }
    })
    .parse_next(input)
}

fn iter_for<'s>(input: &mut Stream<'s>) -> Result<Expr<'s>, ErrMode<ContextError>> {
    seq!(
        _: tok(Token::Keyword("para")),
        identifier,
        _: tok(Token::Keyword("em")),
        expr,
        _: tok(Token::Keyword("faça")),
        parse,
        _:tok(Token::Keyword("fim")),
    )
    .map(|forl| {
        Expr::IterFor {
            control: Box::new(forl.0),
            iterator: Box::new(forl.1),
            stmts: forl.2
        }
    })
    .parse_next(input)
}

fn for_expr<'s>(input: &mut Stream<'s>) -> Result<Expr<'s>, ErrMode<ContextError>> {
    alt((
        range_for,
        iter_for
    ))
    .parse_next(input)
}

fn tuple<'s>(input: &mut Stream<'s>) -> Result<Expr<'s>, ErrMode<ContextError>> {
    delimited(
        tok(Token::BlockDelimeter('(', false)),
        seq!(
            expr,
            _: delimited(
                ws0,
                tok(Token::Comma),
                ws0
            ),
            opt(
                separated(1..,
                    expr,
                    delimited(
                        ws0,
                        tok(Token::Comma),
                        ws0
                    )
                )
            ),
        ),
        preceded(
            ws0,
            cut_err(tok(Token::BlockDelimeter(')', true)))
        )
    )
    .map(|(first, nexts): (_, Option<Vec<_>>)| {
            let mut values = vec![first];
            nexts.map(|mut v| values.append(&mut v));

            Expr::Tuple(values)
    })
    .parse_next(input)
}

fn list<'s>(input: &mut Stream<'s>) -> Result<Expr<'s>, ErrMode<ContextError>> {
    delimited(
    tok(Token::BlockDelimeter('[', false)),
    separated(0..,
        expr,
        delimited(
            ws0,
            tok(Token::Comma),
            ws0
        )
    ),
    preceded(
        ws0,
        cut_err(tok(Token::BlockDelimeter(']', true)))
    )
    )
    .map(Expr::List)
    .parse_next(input)
}

fn expr<'s>(input: &mut Stream<'s>) -> ModalResult<Expr<'s>> {
    fn parser<'s>(precedence: i64) -> impl Parser<Stream<'s>, Expr<'s>, ErrMode<ContextError>> {
        move |i: &mut Stream<'s>| {
            use winnow::combinator::Infix::{Left, Neither, Right};
            expression(
                preceded(
                    ws0,
                    dispatch! { peek(any);
                        &Token::BlockDelimeter('(', false) => alt((
                            tuple,
                            delimited(one_of(&Token::BlockDelimeter('(', false)), parser(0), cut_err(one_of(&Token::BlockDelimeter(')', true)))),
                        )),
                        _ => alt((
                            identifier,
                            literal,
                            string,
                            if_expr,
                            while_expr,
                            for_expr,
                            list
                            // add other expressions here
                        ))
                    }
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
                        UnaryOp::Print => Prefix(2, |_: &mut _, a| Ok(Expr::Unary { op: UnaryOp::Print, expr: Box::new(a)})),
                        UnaryOp::Not => Prefix(7, |_: &mut _, a| Ok(Expr::Unary { op: UnaryOp::Not, expr: Box::new(a)})),
                        UnaryOp::Return => Prefix(0, |_: &mut _, a| Ok(Expr::Unary { op: UnaryOp::Return, expr: Box::new(a)})),
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
        expr.verify_map(|expr| match expr {
            Expr::Binary { lhs, op, rhs } if op == BinOp::ConstAssignment => {
                Some(Stmt::ConstAssignment(Expr::Binary { lhs, op, rhs }))
            },
            _ => None
        }),
        function,
        expr.map(|expr| Stmt::ExprStmt(expr)),
        fail
    )).parse_next(input)
}

pub fn parse<'s>(input: &mut Stream<'s>) -> Result<Vec<Stmt<'s>>, ErrMode<ContextError>> {
    repeat(.., delimited(ws0, stmt, ws0))
    .parse_next(input)
}

#[cfg(test)]
pub mod tests {
    use winnow::{Parser};

use crate::{lexer::tokenize, parser::{new_stream, parse}};

    #[test]
    fn aa() {
        let mut input = include_str!("../../test4.poti");

        let out1 = tokenize(&mut crate::lexer::new_stream(&mut input)).unwrap();
    
        let mut new_stream = new_stream(&out1[..]);
        let lit = parse.parse_next(&mut new_stream);

        println!("{new_stream:#?}");
        println!("{lit:#?}");

        assert!(new_stream.is_empty());
        assert_eq!(lit, Ok(vec![]));
    }
}
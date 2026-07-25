pub mod types;

use winnow::{Parser, Result, ascii::{alphanumeric1, dec_int, }, combinator::{alt, delimited, not, peek, repeat}, error::ContextError, token::{any, one_of, take_till, take_while}};

pub use crate::lexer::types::*;

fn block_delimeter<'s>(input: &mut &'s str) -> Result<Token<'s>> {
    one_of(['(', ')', '[', ']', '{', '}'])
        .map(|c| Token::BlockDelimeter(c, matches!(c, ')' | ']' | '}')))
        .parse_next(input)
}

fn operator<'s>(input: &mut &'s str) -> Result<Token<'s>> {
    alt((
        alt(("+", "-", "*", "/", "^")),
        alt((">=", ">", "<=", "<>", "<")),
        alt(("==", "=>", "=", ":=", "::")),
        alt(("div", "mod"))
    ))
        .map(|op| Token::Operation(op))
        .parse_next(input)
}

fn integer<'s>(input: &mut &'s str) -> Result<Token<'s>> {
    (
        dec_int,
        not(one_of(('.', 'e', 'E')))
    )
    .map(|(n, _)| Token::Integer(n))
    .parse_next(input)
}

fn float<'s>(input: &mut &'s str) -> Result<Token<'s>> {
    winnow::ascii::float.map(|f| Token::Float(f)).parse_next(input)
}

fn bool<'s>(input: &mut &'s str) -> Result<Token<'s>> {
    alt(("verdadeiro", "falso"))
    .map(|c| c == "verdadeiro")
    .map(|b| Token::Boolean(b))
    .parse_next(input)
}

fn literal<'s>(input: &mut &'s str) -> Result<Token<'s>> {
    alt((
        '\n'.map(|_| Token::NewLine),
        ':'.map(|_| Token::Colon),
        ','.map(|_| Token::Comma),
        '.'.map(|_| Token::Period)
    )).parse_next(input)
}

fn unknown<'s>(input: &mut &'s str) -> Result<Token<'s>> {
    any.map(|c| Token::Unknown(c)).parse_next(input)
}

fn keyword<'s>(input: &mut &'s str) -> Result<Token<'s>> {
    dbg!(&input);
    (alt((
        alt(("escreva",
        "imprima",
        "var",
        "em",
        "ou",
        "não", // ew, a tilde
        "se")),
        alt(("então",
        "senãose",
        "senao",
        "fim",
        "escolha",
        "caso",
        "para",
        "de",
        "até")),
        "faça",
        "passo",
        "enquanto",
        "e",
        "retorne",
        "tipo",
        "gere",
    )),
    peek(not(alphanumeric1)))
    .map(|(kw, _)| Token::Keyword(kw))
    .parse_next(input)
}

// short for "parse type"
// stupid type keyword
fn ptype<'s>(input: &mut &'s str) -> Result<Token<'s>> {
    alt(("Inteiro",
        "Real",
        "Texto",
        "Lógico",
        "Caractere",
        //"Tupla" is deduced in the parser
        "Lista"
    )).map(|t| Token::Type(t))
    .parse_next(input)
}

// ran after all the other functions that may get a string
fn identifier<'s>(input: &mut &'s str) -> Result<Token<'s>> {
    (
        one_of(|c: char| c.is_alphabetic() || c == '_'),
        take_while(0.., |c: char| c.is_alphanumeric() || c == '_')
    )
    .take()
    .map(|s| Token::Identifier(s))
    .parse_next(input)
}

fn text<'s>(input: &mut &'s str) -> Result<Token<'s>> {
    println!("Retrieving text from \"{input}\"");
    take_till(1.., ['{', '"'])
    .map(|str| Token::StringFragment(str))
    .parse_next(input)
}

fn interpolation<'s>(input: &mut &'s str) -> Result<Vec<Token<'s>>> {
    println!("Retrieving interp from \"{input}\"");
    let tokens: Vec<Vec<Token<'_>>> = delimited(
        "{",
        repeat(0.., token),
        "}"
    ).parse_next(input)?;
    
    let mut flattened: Vec<_> = tokens.iter().flatten().copied().collect();

    let mut ret = vec![Token::ExprStart];
    
    ret.append(&mut flattened);
    ret.push(Token::ExprEnd);

    Ok(ret)
}

fn string<'s>(input: &mut &'s str) -> Result<Vec<Token<'s>>> {
    let mut string = vec![Token::StringStart];
    
    let parts: Vec<Vec<Token<'_>>> = delimited('"',
    repeat(
        0..,
        alt((
            text.map(|t| vec![t]),
            interpolation
        ))
    ),
    '"'
    )
    .parse_next(input)?;

    let mut flattened: Vec<_> = parts.iter().flatten().copied().collect();

    string.append(&mut flattened);
    string.push(Token::StringEnd);

    Ok(string)
}

fn token<'s>(input: &mut &'s str) -> Result<Vec<Token<'s>>> {
    let tokens = alt((
        alt((
            operator,
            integer,
            float,
            bool,
            literal
        )).map(|t| vec![t]),
        alt((
            keyword,
            ptype,
            operator,
            identifier
        )).map(|t| vec![t]),
        string,
        

        block_delimeter.map(|t| vec![t]),
        
        winnow::ascii::multispace1.map(|_| vec![]),
        unknown.map(|t| vec![t])
    ))
    .parse_next(input)?;

    // } block delimiter should be handled by the string parser
    if tokens.len() == 1 && matches!(tokens[0], Token::BlockDelimeter('}', true)) {
        return Err(ContextError::new());
    }

    Ok(tokens)
}

pub fn tokenize<'s>(input: &mut &'s str) -> Result<Vec<Token<'s>>> {
    println!("call \"{input}\"");
    let t: Vec<Vec<Token<'s>>> = repeat(.., token).parse_next(input)?;

    let flattened: Vec<_> = t.iter().flatten().copied().collect();

    Ok(flattened)
}
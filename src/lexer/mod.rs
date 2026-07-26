pub mod types;

use winnow::{Parser, Result, Stateful, ascii::{alphanumeric1, dec_int, }, combinator::{alt, delimited, not, peek, repeat}, token::{any, one_of, take_till, take_while}};

pub use crate::lexer::types::*;

#[derive(Debug)]
pub struct State {
    interp_level: u32
}

impl Default for State {
    fn default() -> Self {
        State {
            interp_level: 0
        }
    }
}

impl State {
    pub fn is_interpolating(&self) -> bool {
        self.interp_level > 0
    }

    pub fn increase_interp(&mut self) {
        self.interp_level += 1;
    }

    pub fn decrease_interp(&mut self) {
        if self.interp_level == 0 {
            panic!("Tried to decrease interpolation level past 0!");
        }
        self.interp_level -= 1;
    }
}

pub type Stream<'is> = Stateful<&'is str, State>;

pub fn new_stream<'s>(input: &mut &'s str) -> Stream<'s> {
    Stream {
        input,
        state: State::default()
    }
}

fn block_delimeter<'s>(input: &mut Stream<'s>) -> Result<Token<'s>> {
    one_of(['(', ')', '[', ']', '{', '}'])
        .map(|c| Token::BlockDelimeter(c, matches!(c, ')' | ']' | '}')))
        .parse_next(input)
}

fn operator<'s>(input: &mut Stream<'s>) -> Result<Token<'s>> {
    alt((
        alt(("+", "-", "*", "/", "^")),
        alt((">=", ">", "<=", "<>", "<")),
        alt(("==", "=>", "=", ":=", "::")),
        alt(("div", "mod")),
        alt(("imprima", "escreva"))
    ))
        .map(|op| Token::Operation(op))
        .parse_next(input)
}

fn integer<'s>(input: &mut Stream<'s>) -> Result<Token<'s>> {
    (
        dec_int,
        not(one_of(('.', 'e', 'E')))
    )
    .map(|(n, _)| Token::Integer(n))
    .parse_next(input)
}

fn float<'s>(input: &mut Stream<'s>) -> Result<Token<'s>> {
    winnow::ascii::float.map(|f| Token::Float(f)).parse_next(input)
}

fn bool<'s>(input: &mut Stream<'s>) -> Result<Token<'s>> {
    alt(("verdadeiro", "falso"))
    .map(|c| c == "verdadeiro")
    .map(|b| Token::Boolean(b))
    .parse_next(input)
}

fn literal<'s>(input: &mut Stream<'s>) -> Result<Token<'s>> {
    alt((
        '\n'.map(|_| Token::NewLine),
        ':'.map(|_| Token::Colon),
        ','.map(|_| Token::Comma),
        '.'.map(|_| Token::Period)
    )).parse_next(input)
}

fn unknown<'s>(input: &mut Stream<'s>) -> Result<Token<'s>> {
    any.map(|c| Token::Unknown(c)).parse_next(input)
}

fn keyword<'s>(input: &mut Stream<'s>) -> Result<Token<'s>> {
    (alt((
        alt((
        "var",
        "em",
        "ou",
        "não", // ew, a tilde
        )),
        alt((
        "entao",
        "senaose",
        "senao",
        "se",
        "fim",
        "escolha",
        "caso",
        "para",
        "de",)),
        alt(("até",
        "faça",
        "passo",
        "enquanto",
        "e",
        "retorne",
        "tipo",
        "gere",))
    )),
    peek(not(alphanumeric1)))
    .map(|(kw, _)| Token::Keyword(kw))
    .parse_next(input)
}

// short for "parse type"
// stupid type keyword
fn ptype<'s>(input: &mut Stream<'s>) -> Result<Token<'s>> {
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
fn identifier<'s>(input: &mut Stream<'s>) -> Result<Token<'s>> {
    (
        one_of(|c: char| c.is_alphabetic() || c == '_'),
        take_while(0.., |c: char| c.is_alphanumeric() || c == '_')
    )
    .take()
    .map(|s| Token::Identifier(s))
    .parse_next(input)
}

fn text<'s>(input: &mut Stream<'s>) -> Result<Token<'s>> {
    take_till(1.., ['{', '"'])
    .map(|str| Token::StringFragment(str))
    .parse_next(input)
}

fn interpolation<'s>(input: &mut Stream<'s>) -> Result<Vec<Token<'s>>> {
    input.state.increase_interp();

    let tokens: Result<Vec<Vec<Token<'_>>>> = delimited(
        "{",
        repeat(0.., token),
        "}"
    ).parse_next(input);

    input.state.decrease_interp();

    let tokens = tokens?;

    let mut flattened: Vec<_> = tokens.iter().flatten().copied().collect();

    let mut ret = vec![Token::ExprStart];
    
    ret.append(&mut flattened);
    ret.push(Token::ExprEnd);

    Ok(ret)
}

fn string<'s>(input: &mut Stream<'s>) -> Result<Vec<Token<'s>>> {
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

fn token<'s>(input: &mut Stream<'s>) -> Result<Vec<Token<'s>>> {
    if input.state.is_interpolating() {
        not(peek('}'))
        .verify(|_| true)
        .parse_next(input)?;
    }
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

    Ok(tokens)
}

pub fn tokenize<'s>(input: &mut Stream<'s>) -> Result<Vec<Token<'s>>> {
    let t: Vec<Vec<Token<'s>>> = repeat(.., token).parse_next(input)?;

    let flattened: Vec<_> = t.iter().flatten().copied().collect();

    Ok(flattened)
}
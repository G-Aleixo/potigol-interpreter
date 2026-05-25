pub mod types;

use std::collections::HashMap;

pub use crate::lexer::types::*;

pub fn tokenize(code: &str) -> Result<Vec<Token>, &'static str> {
    let mut tokens = vec![];
    let keywords = Trie::keywords();
    let types = Trie::types();
    let operators = Trie::operators();

    let chars: Vec<_> = code.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            // the unary - and + will be handled later down
            '+' => tokens.push(Token::Operation(String::from("+"))),
            '-' => tokens.push(Token::Operation(String::from("-"))),
            '*' => tokens.push(Token::Operation(String::from("*"))),
            '^' => tokens.push(Token::Operation(String::from("^"))),
            '/' => tokens.push(Token::Operation(String::from("/"))),
            '(' => tokens.push(Token::BlockDelimeter(String::from("("), false)),
            ')' => tokens.push(Token::BlockDelimeter(String::from(")"), true)),
            '[' => tokens.push(Token::BlockDelimeter(String::from("["), false)),
            ']' => tokens.push(Token::BlockDelimeter(String::from("]"), true)),
            '{' => tokens.push(Token::BlockDelimeter(String::from("{"), false)),
            '}' => tokens.push(Token::BlockDelimeter(String::from("}"), true)),
            c if c.is_ascii_digit() => {
                let mut num = String::new();
                let mut has_dot = false;

                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    if chars[i] == '.' && has_dot {
                        break;
                    }
                    if chars[i] == '.' {
                        has_dot = true;
                    }

                    num.push(chars[i]);
                    i += 1;
                }
                if let Ok(int) = num.parse() {
                    tokens.push(Token::Integer(int));
                } else if let Ok(float) = num.parse() {
                    tokens.push(Token::Float(float));
                } else {
                    return Err("Could not parse num as any number type");
                }

                continue;
            }
            c if c.is_alphabetic() => {
                let mut ident = String::new();
                while i < chars.len()
                    && (chars[i].is_alphanumeric() || matches!(chars[i], '_' | '-'))
                {
                    ident.push(chars[i]);
                    i += 1;
                }

                if keywords.contains(&ident) {
                    tokens.push(Token::Keyword(ident))
                } else if types.contains(&ident) {
                    tokens.push(Token::Type(ident))
                } else if operators.contains(&ident) {
                    tokens.push(Token::Operation(ident))
                } else {
                    tokens.push(Token::Identifier(ident));
                }
                continue;
            }
            '"' => {
                i += 1;
                let mut string = String::new();
                while i < chars.len() && chars[i] != '"' {
                    string.push(chars[i]);
                    i += 1;
                }
                if i == chars.len() && !string.ends_with('"') {
                    return Err("Unexpected EOF");
                }

                tokens.push(Token::String(string));
            }
            '\'' => {
                i += 1;
                let mut string = String::new();
                while i < chars.len() && chars[i] != '\'' {
                    string.push(chars[i]);
                    i += 1;
                }
                if i == chars.len() && !string.ends_with('\'') {
                    return Err("Unexpected EOF");
                }

                tokens.push(Token::String(string));
            }
            ':' => {
                let tmp = i + 1;
                if tmp < chars.len() {
                    match chars[tmp] {
                        '=' => {
                            tokens.push(Token::Operation(String::from(":=")));
                            i += 1
                        }
                        ':' => {
                            tokens.push(Token::Operation(String::from("::")));
                            i += 1
                        }
                        _ => tokens.push(Token::Colon),
                    }
                } else {
                    tokens.push(Token::Colon);
                }
            }
            '=' => {
                let tmp = i + 1;
                if tmp < chars.len() {
                    match chars[tmp] {
                        '=' => {
                            tokens.push(Token::Operation(String::from("==")));
                            i += 1
                        }
                        '>' => {
                            tokens.push(Token::Operation(String::from("=>")));
                            i += 1
                        }
                        _ => tokens.push(Token::Operation(String::from("="))),
                    }
                } else {
                    tokens.push(Token::Operation(String::from("=")))
                }
            }
            '<' => {
                let tmp = i + 1;
                if tmp < chars.len() {
                    match chars[tmp] {
                        '=' => {
                            tokens.push(Token::Operation(String::from("<=")));
                            i += 1
                        }
                        '>' => {
                            tokens.push(Token::Operation(String::from("<>")));
                            i += 1
                        }
                        _ => tokens.push(Token::Operation(String::from("<"))),
                    }
                } else {
                    tokens.push(Token::Operation(String::from("<")))
                }
            }
            '>' => {
                let tmp = i + 1;
                if tmp < chars.len() {
                    match chars[tmp] {
                        '=' => {
                            tokens.push(Token::Operation(String::from(">=")));
                            i += 1
                        }
                        _ => tokens.push(Token::Operation(String::from(">"))),
                    }
                } else {
                    tokens.push(Token::Operation(String::from(">")))
                }
            }
            '\n' => {
                tokens.push(Token::NewLine);
            }
            ',' => {
                tokens.push(Token::Comma);
            }
            '.' => {
                tokens.push(Token::Period);
            }
            ' ' => {}
            '\r' => {}
            c => {
                tokens.push(Token::Unknown(c));
            }
        }

        i += 1;
    }

    Ok(tokens)
}

#[derive(Debug)]
pub struct Trie {
    children: HashMap<u8, Trie>,
    is_leaf: bool,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            children: HashMap::new(),
            is_leaf: true,
        }
    }

    pub fn insert(&mut self, text: &str) {
        self.insert_bytes(text.as_bytes());
    }

    fn insert_bytes(&mut self, text: &[u8]) {
        if !text.is_empty() {
            self.children.entry(text[0]).or_default();
            self.is_leaf = false;

            let child = self.children.get_mut(&text[0]).unwrap();
            child.insert_bytes(&text[1..text.len()]);
        }
    }

    pub fn contains(&self, text: &str) -> bool {
        self.contains_bytes(text.as_bytes())
    }

    fn contains_bytes(&self, text: &[u8]) -> bool {
        if text.is_empty() {
            return true;
        } else if self.is_leaf && !text.is_empty() {
            return false;
        } else if let Some(child) = self.children.get(&text[0]) {
            return child.contains_bytes(&text[1..text.len()]);
        };

        false
    }

    fn keywords() -> Trie {
        Trie::from(vec![
            // these are loaded in as a "std lib"
            // "leia_texto",
            // "leia_inteiro",
            // "leia_numero",
            "escreva",
            "imprima",
            "var",
            "falso",
            "verdadeiro",
            "e",
            "ou",
            "não", // ew, a tilde
            "se",
            "então",
            "senão",
            "senãose",
            "fim",
            "escolha",
            "caso",
            "para",
            "de",
            "até",
            "faça",
            "passo",
            "em",
            "enquanto",
            "retorne",
            "tipo",
            "gere",
        ])
    }
    fn types() -> Trie {
        Trie::from(vec![
            "Inteiro",
            "Real",
            "Texto",
            "Lógico",
            "Caractere",
            //"Tupla" é deduzida no proximo passo da compilação
            "Lista",
        ])
    }
    fn operators() -> Trie {
        Trie::from(vec!["div", "mod"])
    }
}

impl Default for Trie {
    fn default() -> Trie {
        Trie::new()
    }
}

impl From<Vec<&str>> for Trie {
    fn from(value: Vec<&str>) -> Trie {
        let mut trie = Trie::new();

        for str in value {
            trie.insert(str);
        }

        trie
    }
}

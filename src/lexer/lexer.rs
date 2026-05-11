use std::collections::HashMap;

pub use crate::lexer::types::*;


pub fn tokenize<'a>(code: &'a str) -> Result<Vec<Token<'a>>, &'static str> {
    let mut tokens = vec![];
    let keywords = Trie::keywords();

    let chars: Vec<_> = code.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '+' => tokens.push(Token::Operation(Operation { operation: "+" })),
            '-' => tokens.push(Token::Operation(Operation { operation: "-" })),
            '*' => tokens.push(Token::Operation(Operation { operation: "*" })),
            '/' => tokens.push(Token::Operation(Operation { operation: "/" })),
            '=' => tokens.push(Token::Operation(Operation { operation: "=" })),
            '(' => tokens.push(Token::BlockDelimeter(BlockDelimeter { delimeter: "(", is_close: false })),
            ')' => tokens.push(Token::BlockDelimeter(BlockDelimeter { delimeter: ")", is_close: true })),
            '[' => tokens.push(Token::BlockDelimeter(BlockDelimeter { delimeter: "[", is_close: false })),
            ']' => tokens.push(Token::BlockDelimeter(BlockDelimeter { delimeter: "]", is_close: true })),
            '{' => tokens.push(Token::BlockDelimeter(BlockDelimeter { delimeter: "{", is_close: false })),
            '}' => tokens.push(Token::BlockDelimeter(BlockDelimeter { delimeter: "}", is_close: true })),
            c if c.is_digit(10) => {
                let mut num = String::new();
                let mut has_dot = false;

                while i < chars.len() && (chars[i].is_digit(10) || chars[i] == '.') {
                    if chars[i] == '.' && has_dot {
                        return Err("Float number has 2 dots");
                    }
                    has_dot = true;
                    num.push(chars[i]);
                    i += 1;
                }
                tokens.push(Token::Integer(num.parse().expect(format!("Failed to parse {num}").as_str())));
                continue
            }
            c if c.is_alphabetic() => {
                let mut ident = String::new();
                while i < chars.len() && chars[i].is_alphanumeric() {
                    ident.push(chars[i]);
                    i += 1;
                }

                if keywords.contains(&ident) {
                    tokens.push(Token::Keyword(Keyword { keyword: ident }))
                } else {
                    tokens.push(Token::Identifier(Identifier { symbol: ident }));
                }
                continue;
            }
            c if c == '"' => {
                i += 1;
                let mut string = String::new();
                while i < chars.len() && chars[i] != '"' {
                    string.push(chars[i]);
                    i += 1;
                }

                tokens.push(Token::String(string));
            }
            c if c == ':' => {
                let tmp = i + 1;
                if tmp < chars.len() && chars[tmp] == '=' {
                    tokens.push(Token::String(String::from(":=")));
                }
            }
            '\n' => {
                tokens.push(Token::NewLine);
            }
            ',' => {
                tokens.push(Token::Comma);
            }
            ' ' => {}
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
    children: HashMap<char, Trie>,
    is_leaf: bool,
}

impl Trie {
    pub fn new() -> Trie {
        Trie { children: HashMap::new(), is_leaf: true }
    }

    pub fn insert(&mut self, text: &str) {
        if text.len() > 0 {
            if let None = self.children.get(&text.chars().nth(0).unwrap()) {
                self.children.insert(text.chars().nth(0).unwrap(), Trie::new());
                self.is_leaf = false;
            }
            let child = self.children.get_mut(&text.chars().nth(0).unwrap()).unwrap();
            child.insert(&text[1..text.len()]);
        }
    }

    fn contains(&self, text: &str) -> bool {
        if self.is_leaf && text.len() == 0 {
            return true
        } else if self.is_leaf {
            return false
        } else if let Some(child) = self.children.get(&text.chars().nth(0).unwrap()) {
            return child.contains(&text[1..text.len()]);
        };
        
        false
    }

    fn keywords() -> Trie {
        Trie::from(vec![
            "escreva",
            "imprima",
            "leia_texto",
            "leia_inteiro",
            "leia_numero",
            "var",
        ])
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
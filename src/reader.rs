use anyhow::{bail, Result};

#[derive(Debug)]
pub enum TokenData<'a> {
    LeftParen,
    RightParen,
    Int(i32),
    Symbol(&'a str),
}

#[derive(Debug)]
pub struct Token<'a> {
    data: TokenData<'a>,
    offset: usize,
    len: usize,
}

fn is_symbolic(c: char) -> bool {
    c.is_alphanumeric()
        || c == '*'
        || c == '+'
        || c == '!'
        || c == '-'
        || c == '_'
        || c == '\''
        || c == '?'
        || c == '<'
        || c == '>'
        || c == '='
}

pub struct Tokens<'a> {
    source: &'a str,
    offset: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(source: &'a str) -> Tokens<'a> {
        Tokens { source, offset: 0 }
    }

    fn read_int(&self) -> Result<Token<'a>> {
        let mut len = 0;
        for c in self.source[self.offset..].chars() {
            if !(c.is_ascii_digit() || c == '-' || c == '+') {
                break;
            }
            len += c.len_utf8();
        }
        Ok(Token {
            data: TokenData::Int(self.source[self.offset..self.offset + len].parse()?),
            offset: self.offset,
            len,
        })
    }

    fn read_symbol(&self) -> Result<Token<'a>> {
        let mut len = 0;
        for c in self.source[self.offset..].chars() {
            if !is_symbolic(c) {
                break;
            }
            len += c.len_utf8();
        }
        Ok(Token {
            data: TokenData::Symbol(&self.source[self.offset..self.offset + len]),
            offset: self.offset,
            len,
        })
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Result<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        for c in self.source[self.offset..].chars() {
            if c.is_whitespace() {
                self.offset += c.len_utf8();
            } else {
                break;
            }
        }
        self.source[self.offset..].chars().next().map(|c| {
            let token = match c {
                '(' => Token {
                    data: TokenData::LeftParen,
                    offset: self.offset,
                    len: 1,
                },
                ')' => Token {
                    data: TokenData::RightParen,
                    offset: self.offset,
                    len: 1,
                },
                c if c.is_ascii_digit() || c == '-' || c == '+' => self.read_int()?,
                c if is_symbolic(c) => self.read_symbol()?,
                c => bail!("unexpected character {} at offset {}", c, self.offset),
            };
            self.offset = token.offset + token.len;
            Ok(token)
        })
    }
}

struct Reader<'a> {
    tokens: Tokens<'a>,
}

impl<'a> Reader<'a> {
    fn new(source: &'a str) -> Reader<'a> {
        Reader {
            tokens: Tokens::new(source),
        }
    }
}

impl<'a> Iterator for Reader<'a> {
    type Item = Result<Form<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next().map(|token| {
            let token = token?;
            let form = match token.data {
                TokenData::LeftParen => {
                    todo!()
                }
                TokenData::RightParen => {
                    bail!("unexpected closing parenthesis at offset {}", token.offset)
                }
                TokenData::Int(int) => Form {
                    data: FormData::Int(int),
                },
                TokenData::Symbol(symbol) => Form {
                    data: FormData::Symbol(symbol),
                },
            };
            Ok(token)
        })
    }
}

use {
    crate::{
        error::{Error, ErrorData, Result},
        value::{Meta, Value},
    },
    std::{iter, rc::Rc, str},
};

#[derive(Debug, PartialEq)]
enum TokenData<'a> {
    LeftParen,
    RightParen,
    LeftSquare,
    RightSquare,
    Int(i32),
    Sym(&'a str),
    Quote,
    Quasiquote,
    Unquote,
    SpliceUnquote,
}

#[derive(Debug)]
struct Token<'a> {
    data: TokenData<'a>,
    line_no: usize,
}

struct Tokens<'a> {
    source: &'a str,
    chars: iter::Peekable<str::CharIndices<'a>>,
    line_no: usize,
}

impl<'a> Tokens<'a> {
    fn new(source: &'a str) -> Tokens<'a> {
        Tokens {
            source,
            chars: source.char_indices().peekable(),
            line_no: 1,
        }
    }

    // byte index of end of current char (exclusive)
    fn end(&mut self) -> usize {
        match self.chars.peek() {
            Some((offset, _)) => *offset,
            None => self.source.len(),
        }
    }

    fn next_char(&mut self) -> Option<(usize, char)> {
        let res = self.chars.next();
        if let Some((_, '\n')) = res {
            self.line_no += 1;
        }
        res
    }

    fn peek_char_is<F>(&mut self, mut f: F) -> bool
    where
        F: FnMut(char) -> bool,
    {
        self.chars.peek().map_or(false, |(_, c)| f(*c))
    }

    fn read_int(&mut self, offset: usize) -> Result<Token<'a>> {
        while self.peek_char_is(is_symbolic) {
            self.next_char();
        }
        self.source[offset..self.end()]
            .parse()
            .map(|int| Token {
                data: TokenData::Int(int),
                line_no: self.line_no,
            })
            .map_err(|err| {
                Error::new(ErrorData::ParseInt(
                    self.source[offset..self.end()].to_string(),
                    err,
                ))
                .with(Meta {
                    line_no: Some(self.line_no),
                })
            })
    }

    fn read_symbol(&mut self, offset: usize) -> Token<'a> {
        while self.peek_char_is(is_symbolic) {
            self.next_char();
        }
        Token {
            data: TokenData::Sym(&self.source[offset..self.end()]),
            line_no: self.line_no,
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Result<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.peek_char_is(|c| c.is_whitespace()) {
            self.next_char();
        }
        if self.peek_char_is(|c| c == ';') {
            while !self.peek_char_is(|c| c == '\n') {
                self.next_char();
            }
            return self.next();
        }
        self.next_char().map(|(offset, c)| {
            Ok(match c {
                '(' => Token {
                    data: TokenData::LeftParen,
                    line_no: self.line_no,
                },
                ')' => Token {
                    data: TokenData::RightParen,
                    line_no: self.line_no,
                },
                '[' => Token {
                    data: TokenData::LeftSquare,
                    line_no: self.line_no,
                },
                ']' => Token {
                    data: TokenData::RightSquare,
                    line_no: self.line_no,
                },
                '\'' => Token {
                    data: TokenData::Quote,
                    line_no: self.line_no,
                },
                '`' => Token {
                    data: TokenData::Quasiquote,
                    line_no: self.line_no,
                },
                '~' => {
                    if self.peek_char_is(|c| c == '@') {
                        self.next();
                        Token {
                            data: TokenData::SpliceUnquote,
                            line_no: self.line_no,
                        }
                    } else {
                        Token {
                            data: TokenData::Unquote,
                            line_no: self.line_no,
                        }
                    }
                }
                c if c.is_ascii_digit()
                    || (c == '-' || c == '+') && self.peek_char_is(|c| c.is_ascii_digit()) =>
                {
                    self.read_int(offset)?
                }
                c if is_symbolic(c) => self.read_symbol(offset),
                c => {
                    return Err(Error::new(ErrorData::UnexpectedChar(c)).with(Meta {
                        line_no: Some(self.line_no),
                    }))
                }
            })
        })
    }
}

struct Reader<'a> {
    tokens: iter::Peekable<Tokens<'a>>,
}

impl<'a> Reader<'a> {
    fn peek_token_is(&mut self, data: &TokenData) -> bool {
        self.tokens
            .peek()
            .map_or(false, |t| t.as_ref().map_or(false, |t| &t.data == data))
    }

    fn read_coll(&mut self, close: TokenData) -> Result<Vec<Value>> {
        let mut coll = Vec::new();
        while !self.peek_token_is(&close) {
            if let Some(value) = self.next() {
                coll.push(value?);
            } else {
                return Err(Error::new(ErrorData::UnexpectedEof));
            }
        }
        self.tokens.next();
        Ok(coll)
    }

    fn application(&mut self, sym: &str, line_no: usize) -> Result<Value> {
        Ok(Value::List(
            Rc::new(vec![
                Value::Sym(
                    sym.to_string(),
                    Meta {
                        line_no: Some(line_no),
                    },
                ),
                self.next()
                    .unwrap_or(Err(Error::new(ErrorData::UnexpectedEof)))?,
            ]),
            Meta {
                line_no: Some(line_no),
            },
        ))
    }
}

impl<'a> Iterator for Reader<'a> {
    type Item = Result<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next().map(|token| {
            let token = token?;
            Ok(match token.data {
                TokenData::Int(int) => Value::Int(int),
                TokenData::Sym(sym) => Value::Sym(
                    sym.to_string(),
                    Meta {
                        line_no: Some(token.line_no),
                    },
                ),
                TokenData::LeftParen => Value::List(
                    Rc::new(self.read_coll(TokenData::RightParen)?),
                    Meta {
                        line_no: Some(token.line_no),
                    },
                ),
                TokenData::RightParen => {
                    return Err(Error::new(ErrorData::UnexpectedChar(')')).with(Meta {
                        line_no: Some(token.line_no),
                    }))
                }
                TokenData::LeftSquare => {
                    Value::Vec(Rc::new(self.read_coll(TokenData::RightSquare)?))
                }
                TokenData::RightSquare => {
                    return Err(Error::new(ErrorData::UnexpectedChar(']')).with(Meta {
                        line_no: Some(token.line_no),
                    }))
                }
                TokenData::Quote => self.application("quote", token.line_no)?,
                TokenData::Quasiquote => self.application("quasiquote", token.line_no)?,
                TokenData::Unquote => self.application("unquote", token.line_no)?,
                TokenData::SpliceUnquote => self.application("splice-unquote", token.line_no)?,
            })
        })
    }
}

fn is_symbolic(c: char) -> bool {
    c.is_alphanumeric() || "*+!-_'?<>=/".contains(c)
}

pub fn read(source: &str) -> impl Iterator<Item = Result<Value>> + '_ {
    Reader {
        tokens: Tokens::new(source).peekable(),
    }
}

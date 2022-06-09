use {
    crate::{
        error::{Error, Result},
        value::Value,
    },
    std::{iter, str},
};

#[derive(Debug, PartialEq, Clone)]
enum TokenData<'a> {
    LeftParen,
    RightParen,
    Dot,
    Int(i32),
    Sym(&'a str),
    Quote,
    Quasiquote,
    Unquote,
    SpliceUnquote,
}

#[derive(Debug, Clone)]
struct Token<'a> {
    data: TokenData<'a>,
    _line_no: usize,
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
                _line_no: self.line_no,
            })
            .map_err(|err| Error::parse_int(&self.source[offset..self.end()], err))
    }

    fn read_symbol(&mut self, offset: usize) -> Token<'a> {
        while self.peek_char_is(is_symbolic) {
            self.next_char();
        }
        Token {
            data: TokenData::Sym(&self.source[offset..self.end()]),
            _line_no: self.line_no,
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
                    _line_no: self.line_no,
                },
                ')' => Token {
                    data: TokenData::RightParen,
                    _line_no: self.line_no,
                },
                '.' => Token {
                    data: TokenData::Dot,
                    _line_no: self.line_no,
                },
                '\'' => Token {
                    data: TokenData::Quote,
                    _line_no: self.line_no,
                },
                '`' => Token {
                    data: TokenData::Quasiquote,
                    _line_no: self.line_no,
                },
                '~' => {
                    if self.peek_char_is(|c| c == '@') {
                        self.next_char();
                        Token {
                            data: TokenData::SpliceUnquote,
                            _line_no: self.line_no,
                        }
                    } else {
                        Token {
                            data: TokenData::Unquote,
                            _line_no: self.line_no,
                        }
                    }
                }
                c if c.is_ascii_digit()
                    || (c == '-' || c == '+') && self.peek_char_is(|c| c.is_ascii_digit()) =>
                {
                    self.read_int(offset)?
                }
                c if is_symbolic(c) => self.read_symbol(offset),
                c => return Err(Error::unexpected_char(c)),
            })
        })
    }
}

struct Reader<'a> {
    tokens: iter::Peekable<Tokens<'a>>,
}

impl<'a> Reader<'a> {
    fn peek_token_is(&mut self, data: TokenData) -> bool {
        self.tokens
            .peek()
            .map_or(false, |t| t.as_ref().map_or(false, |t| t.data == data))
    }

    fn application(&mut self, sym: &str) -> Result<Value> {
        match self.next() {
            Some(v) => Ok(Value::cons(
                Value::sym(sym.to_string()),
                Value::cons(v?, Value::Nil),
            )),
            None => Err(Error::unexpected_eof()),
        }
    }

    fn consume(&mut self, data: TokenData) -> Result<()> {
        match self.tokens.next() {
            Some(Ok(token)) => {
                if token.data == data {
                    Ok(())
                } else {
                    Err(Error::todo("unexpected token"))
                }
            }
            Some(Err(err)) => Err(err),
            None => Err(Error::unexpected_eof()),
        }
    }

    fn read_list(&mut self) -> Result<Value> {
        if self.peek_token_is(TokenData::RightParen) {
            self.consume(TokenData::RightParen)?;
            Ok(Value::Nil)
        } else if self.peek_token_is(TokenData::Dot) {
            self.consume(TokenData::Dot)?;
            let res = self.next().unwrap_or(Err(Error::unexpected_eof()));
            self.consume(TokenData::RightParen)?;
            res
        } else {
            Ok(Value::cons(
                self.next().unwrap_or(Err(Error::unexpected_eof()))?,
                self.read_list()?,
            ))
        }
    }
}

impl<'a> Iterator for Reader<'a> {
    type Item = Result<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next().map(|token| {
            Ok(match token?.data {
                TokenData::Int(int) => Value::Int(int),
                TokenData::Sym(sym) => Value::sym(sym.to_string()),
                TokenData::LeftParen => self.read_list()?,
                TokenData::RightParen => return Err(Error::unexpected_char(')')),
                TokenData::Dot => return Err(Error::unexpected_char('.')),
                TokenData::Quote => self.application("quote")?,
                TokenData::Quasiquote => self.application("quasiquote")?,
                TokenData::Unquote => self.application("unquote")?,
                TokenData::SpliceUnquote => self.application("splice-unquote")?,
            })
        })
    }
}

fn is_symbolic(c: char) -> bool {
    c.is_alphanumeric() || "!$%&*+-./:<=>?@^_~".contains(c)
}

pub fn read(source: &str) -> impl Iterator<Item = Result<Value>> + '_ {
    Reader {
        tokens: Tokens::new(source).peekable(),
    }
}

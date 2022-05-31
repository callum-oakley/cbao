use {
    crate::value::Value,
    anyhow::{bail, Result},
    std::{iter, str},
};

#[derive(Debug, PartialEq)]
enum TokenData<'a> {
    LeftParen,
    RightParen,
    Int(i32),
    Symbol(&'a str),
}

#[derive(Debug)]
struct Token<'a> {
    data: TokenData<'a>,
    offset: usize,
}

struct Tokens<'a> {
    source: &'a str,
    chars: iter::Peekable<str::CharIndices<'a>>,
}

impl<'a> Tokens<'a> {
    fn new(source: &'a str) -> Tokens<'a> {
        Tokens {
            source,
            chars: source.char_indices().peekable(),
        }
    }

    // byte index of end of current char (exclusive)
    fn end(&mut self) -> usize {
        match self.chars.peek() {
            Some((offset, _)) => *offset,
            None => self.source.len(),
        }
    }

    fn next_char_is<F>(&mut self, mut f: F) -> bool
    where
        F: FnMut(char) -> bool,
    {
        self.chars.peek().map_or(false, |(_, c)| f(*c))
    }

    fn read_int(&mut self, offset: usize) -> Result<Token<'a>> {
        while self.next_char_is(|c| c.is_ascii_digit()) {
            self.chars.next();
        }
        Ok(Token {
            data: TokenData::Int(self.source[offset..self.end()].parse()?),
            offset,
        })
    }

    fn read_symbol(&mut self, offset: usize) -> Token<'a> {
        while self.next_char_is(is_symbolic) {
            self.chars.next();
        }
        Token {
            data: TokenData::Symbol(&self.source[offset..self.end()]),
            offset,
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Result<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next_char_is(|c| c.is_whitespace()) {
            self.chars.next();
        }
        self.chars.next().map(|(offset, c)| {
            Ok(match c {
                '(' => Token {
                    data: TokenData::LeftParen,
                    offset,
                },
                ')' => Token {
                    data: TokenData::RightParen,
                    offset,
                },
                c if c.is_ascii_digit()
                    || (c == '-' || c == '+') && self.next_char_is(|c| c.is_ascii_digit()) =>
                {
                    self.read_int(offset)?
                }
                c if is_symbolic(c) => self.read_symbol(offset),
                c => bail!(
                    "unexpected character '{}' on line {}",
                    c,
                    line_no(self.source, offset),
                ),
            })
        })
    }
}

pub struct Reader<'a> {
    source: &'a str,
    tokens: iter::Peekable<Tokens<'a>>,
}

impl<'a> Reader<'a> {
    pub fn new(source: &'a str) -> Reader<'a> {
        Reader {
            source,
            tokens: Tokens::new(source).peekable(),
        }
    }

    fn next_token_is(&mut self, data: TokenData) -> bool {
        self.tokens
            .peek()
            .map_or(false, |t| t.as_ref().map_or(false, |t| t.data == data))
    }
}

impl<'a> Iterator for Reader<'a> {
    type Item = Result<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.next().map(|token| {
            let token = token?;
            Ok(match token.data {
                TokenData::Int(int) => Value::Int(int),
                TokenData::Symbol(symbol) => Value::Symbol(symbol.to_string()),
                TokenData::LeftParen => {
                    let mut list = Vec::new();
                    while !self.next_token_is(TokenData::RightParen) {
                        if let Some(value) = self.next() {
                            list.push(value?);
                        } else {
                            bail!("unexpected end of input")
                        }
                    }
                    self.tokens.next();
                    Value::List(list)
                }
                TokenData::RightParen => {
                    bail!(
                        "unmatched closing parenthesis on line {}",
                        line_no(self.source, token.offset),
                    )
                }
            })
        })
    }
}

fn is_symbolic(c: char) -> bool {
    c.is_alphanumeric() || "*+!-_'?<>=".contains(c)
}

// The line number of offset in the given source; 1 indexed
fn line_no(source: &str, offset: usize) -> usize {
    source[..offset].chars().filter(|c| *c == '\n').count() + 1
}

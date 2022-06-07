use {crate::value::Value, std::fmt};

#[derive(Debug)]
pub enum ErrorData {
    IO(std::io::Error),
    ParseInt(String, std::num::ParseIntError),
    UnexpectedChar(char),
    UnexpectedEof,
    UnknownSym(String),
    Function(Value),
    Cast(Value, String),
    Todo(String),
}

#[derive(Debug)]
pub struct Error {
    pub data: ErrorData,
    source: Option<Box<Error>>,
}

impl Error {
    pub fn parse_int(s: String, err: std::num::ParseIntError) -> Error {
        Error {
            data: ErrorData::ParseInt(s, err),
            source: None,
        }
    }

    pub fn unexpected_char(c: char) -> Error {
        Error {
            data: ErrorData::UnexpectedChar(c),
            source: None,
        }
    }

    pub fn unexpected_eof() -> Error {
        Error {
            data: ErrorData::UnexpectedEof,
            source: None,
        }
    }

    pub fn unknown_sym(sym: String) -> Error {
        Error {
            data: ErrorData::UnknownSym(sym),
            source: None,
        }
    }

    pub fn function(value: Value) -> Error {
        Error {
            data: ErrorData::Function(value),
            source: None,
        }
    }

    pub fn cast(v: Value, t: String) -> Error {
        Error {
            data: ErrorData::Cast(v, t),
            source: None,
        }
    }

    pub fn todo(msg: String) -> Error {
        Error {
            data: ErrorData::Todo(msg),
            source: None,
        }
    }

    pub fn source(self, err: Error) -> Error {
        Error {
            data: self.data,
            source: Some(Box::new(err)),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error {
            data: ErrorData::IO(err),
            source: None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            ErrorData::IO(err) => err.fmt(f),
            ErrorData::ParseInt(s, _) => write!(f, "failed to parse '{s}' as an int"),
            ErrorData::UnexpectedChar(c) => write!(f, "unexpected char '{c}'"),
            ErrorData::UnexpectedEof => write!(f, "unexpected end of input"),
            ErrorData::UnknownSym(sym) => write!(f, "unknown symbol '{sym}'"),
            ErrorData::Function(value) => write!(f, "in function '{value}'"),
            ErrorData::Cast(v, t) => write!(f, "type error: '{v}' is not {t}"),
            ErrorData::Todo(msg) => write!(f, "TODO {msg}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.source {
            Some(err) => Some(err.as_ref()),
            None => match &self.data {
                ErrorData::IO(err) => Some(err),
                ErrorData::ParseInt(_, err) => Some(err),
                _ => None,
            },
        }
    }
}

pub fn report(mut err: &(dyn std::error::Error)) {
    eprintln!("{}", err);
    while let Some(e) = err.source() {
        err = e;
        eprintln!("{}", err);
    }
}

pub type Result<T> = std::result::Result<T, Error>;

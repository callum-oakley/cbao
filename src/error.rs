use {
    crate::value::{Meta, Value},
    std::fmt,
};

#[derive(Debug)]
pub enum ErrorData {
    IO(std::io::Error),
    ParseInt(String, std::num::ParseIntError),
    UnexpectedChar(char),
    UnexpectedEof,
    UnknownSym(String),
    Apply(Value),
    Type(Value, String),
    Todo,
}

#[derive(Debug)]
pub struct Error {
    pub data: ErrorData,
    meta: Meta,
    source: Option<Box<Error>>,
}

impl Error {
    pub fn new(data: ErrorData) -> Error {
        Error {
            data,
            meta: Meta { line_no: None },
            source: None,
        }
    }

    pub fn with(self, meta: Meta) -> Error {
        Error {
            data: self.data,
            meta,
            source: self.source,
        }
    }

    pub fn wrap(self, data: ErrorData) -> Error {
        Error {
            data,
            meta: Meta { line_no: None },
            source: Some(Box::new(self)),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::new(ErrorData::IO(err))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            ErrorData::IO(err) => err.fmt(f),
            ErrorData::ParseInt(s, _) => write!(f, "failed to parse '{s}' as an Int"),
            ErrorData::UnexpectedChar(c) => write!(f, "unexpected char '{c}'"),
            ErrorData::UnexpectedEof => write!(f, "unexpected end of input"),
            ErrorData::UnknownSym(sym) => write!(f, "unknown symbol '{sym}'"),
            ErrorData::Apply(value) => write!(f, "in '{value}'"),
            ErrorData::Type(value, t) => write!(f, "type error: '{value}' is not {t}"),
            ErrorData::Todo => write!(f, "TODO"),
        }?;
        if let Some(line_no) = self.meta.line_no {
            write!(f, " (line {line_no})")?;
        }
        Ok(())
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

use {crate::value::Value, std::fmt};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    ParseInt {
        target: String,
        line_no: usize,
        err: std::num::ParseIntError,
    },
    UnexpectedChar {
        target: char,
        line_no: usize,
    },
    UnexpectedEof,
    // TODO line numbers and extra context for the below
    UnknownSymbol {
        target: String,
    },
    NotFn {
        target: Value,
    },
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IO(err) => err.fmt(f),
            Error::ParseInt {
                target, line_no, ..
            } => write!(
                f,
                "failed to parse \"{target}\" as an int on line {line_no}"
            ),
            Error::UnexpectedChar { target, line_no } => {
                write!(f, "unexpected char '{target}' on line {line_no}")
            }
            Error::UnexpectedEof => write!(f, "unexpected end of input"),
            Error::UnknownSymbol { target } => write!(f, "unknown symbol {target}"),
            Error::NotFn { target } => write!(f, "{target} is not a function"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IO(err) => Some(err),
            Error::ParseInt { err, .. } => Some(err),
            _ => None,
        }
    }
}

pub fn report(mut err: &(dyn std::error::Error)) {
    eprintln!("error: {}", err);
    while let Some(e) = err.source() {
        err = e;
        eprintln!("caused by: {}", err);
    }
}

pub type Result<T> = std::result::Result<T, Error>;

// The line number of offset in the given source; 1 indexed
pub fn line_no(source: &str, offset: usize) -> usize {
    source[..offset].chars().filter(|c| *c == '\n').count() + 1
}

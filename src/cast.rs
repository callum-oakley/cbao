use crate::{
    error::{Error, Result},
    value::{Pair, Value},
};

pub fn int(v: &Value) -> Result<&i32> {
    match v {
        Value::Int(int) => Ok(int),
        _ => Err(Error::cast(v, "an int")),
    }
}

pub fn sym(v: &Value) -> Result<&str> {
    match v {
        Value::Sym(sym) => Ok(sym),
        _ => Err(Error::cast(v, "a sym")),
    }
}

pub fn pair(v: &Value) -> Result<&Pair> {
    match v {
        Value::Pair(pair) => Ok(pair),
        _ => Err(Error::cast(v, "a pair")),
    }
}

pub fn car(v: &Value) -> Result<&Value> {
    Ok(pair(v)?.car())
}

pub fn cdr(v: &Value) -> Result<&Value> {
    Ok(pair(v)?.cdr())
}

pub fn cadr(v: &Value) -> Result<&Value> {
    car(cdr(v)?)
}

pub fn cddr(v: &Value) -> Result<&Value> {
    cdr(cdr(v)?)
}

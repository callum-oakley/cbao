use crate::{
    error::{Error, Result},
    value::{Env, Pair, Primitive, Value},
};

pub fn get_1(args: &Value) -> Result<&Value> {
    match args {
        Value::Pair(pair) => match pair.cdr() {
            Value::Nil => Ok(pair.car()),
            _ => Err(Error::too_many_args(1)),
        },
        _ => Ok(&Value::Nil),
    }
}

pub fn get_2(args: &Value) -> Result<(&Value, &Value)> {
    match args {
        Value::Pair(pair) => match get_1(pair.cdr()) {
            Ok(arg) => Ok((pair.car(), arg)),
            _ => Err(Error::too_many_args(2)),
        },
        _ => Ok((&Value::Nil, &Value::Nil)),
    }
}

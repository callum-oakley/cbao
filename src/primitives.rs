use crate::{args, cast, error::Result, value::Value};

pub fn cons(args: &Value) -> Result<Value> {
    args::arity(args, 2)?;
    Ok(Value::cons(
        cast::car(args)?.clone(),
        cast::cadr(args)?.clone(),
    ))
}

pub fn car(args: &Value) -> Result<Value> {
    args::arity(args, 1)?;
    Ok(cast::car(cast::car(args)?)?.clone())
}

pub fn cdr(args: &Value) -> Result<Value> {
    args::arity(args, 1)?;
    Ok(cast::cdr(cast::car(args)?)?.clone())
}

pub fn add(args: &Value) -> Result<Value> {
    args::arity(args, 2)?;
    Ok(Value::Int(
        cast::int(cast::car(args)?)? + cast::int(cast::cadr(args)?)?,
    ))
}

pub fn mul(args: &Value) -> Result<Value> {
    args::arity(args, 2)?;
    Ok(Value::Int(
        cast::int(cast::car(args)?)? * cast::int(cast::cadr(args)?)?,
    ))
}

pub fn sub(args: &Value) -> Result<Value> {
    args::arity(args, 2)?;
    Ok(Value::Int(
        cast::int(cast::car(args)?)? - cast::int(cast::cadr(args)?)?,
    ))
}

pub fn div(args: &Value) -> Result<Value> {
    args::arity(args, 2)?;
    Ok(Value::Int(
        cast::int(cast::car(args)?)? / cast::int(cast::cadr(args)?)?,
    ))
}

pub fn eq(args: &Value) -> Result<Value> {
    args::arity(args, 2)?;
    todo!()
}

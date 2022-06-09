use crate::{args, cast, error::Result, value::Value};

pub fn cons(args: &Value) -> Result<Value> {
    Ok(Value::cons(
        args::arg_0(args)?.clone(),
        args::arg_1(args)?.clone(),
    ))
}

pub fn car(args: &Value) -> Result<Value> {
    Ok(cast::car(args::arg_0(args)?)?.clone())
}

pub fn cdr(args: &Value) -> Result<Value> {
    Ok(cast::cdr(args::arg_0(args)?)?.clone())
}

pub fn add(args: &Value) -> Result<Value> {
    Ok(Value::Int(
        cast::int(args::arg_0(args)?)? + cast::int(args::arg_1(args)?)?,
    ))
}

pub fn mul(args: &Value) -> Result<Value> {
    Ok(Value::Int(
        cast::int(args::arg_0(args)?)? * cast::int(args::arg_1(args)?)?,
    ))
}

pub fn sub(args: &Value) -> Result<Value> {
    Ok(Value::Int(
        cast::int(args::arg_0(args)?)? - cast::int(args::arg_1(args)?)?,
    ))
}

pub fn div(args: &Value) -> Result<Value> {
    Ok(Value::Int(
        cast::int(args::arg_0(args)?)? / cast::int(args::arg_1(args)?)?,
    ))
}

pub fn eq(_args: &Value) -> Result<Value> {
    todo!()
    // let (x, y) = args::get_2(args)?;
    // Ok(if x == y { x } else { Value::Nil })
}

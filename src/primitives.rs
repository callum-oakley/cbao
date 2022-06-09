use {
    crate::{
        cast,
        error::{Error, Result},
        value::{Env, Pair, Primitive, Proc, Value},
    },
    std::rc::Rc,
};

fn args_1(args: &Value) -> Result<&Value> {
    match args {
        Value::Pair(pair) => match pair.cdr() {
            Value::Nil => Ok(pair.car()),
            _ => Err(Error::too_many_args(1)),
        },
        _ => Ok(&Value::Nil),
    }
}

fn args_2(args: &Value) -> Result<(&Value, &Value)> {
    match args {
        Value::Pair(pair) => match args_1(pair.cdr()) {
            Ok(arg) => Ok((pair.car(), arg)),
            _ => Err(Error::too_many_args(2)),
        },
        _ => Ok((&Value::Nil, &Value::Nil)),
    }
}

pub fn cons(args: &Value) -> Result<Value> {
    let (x, y) = args_2(args)?;
    Ok(Value::cons(x.clone(), y.clone()))
}

pub fn car(args: &Value) -> Result<Value> {
    Ok(cast::car(args_1(args)?)?.clone())
}

pub fn cdr(args: &Value) -> Result<Value> {
    Ok(cast::cdr(args_1(args)?)?.clone())
}

pub fn add(args: &Value) -> Result<Value> {
    let (x, y) = args_2(args)?;
    Ok(Value::Int(cast::int(x)? + cast::int(y)?))
}

pub fn mul(args: &Value) -> Result<Value> {
    let (x, y) = args_2(args)?;
    Ok(Value::Int(cast::int(x)? * cast::int(y)?))
}

pub fn sub(args: &Value) -> Result<Value> {
    let (x, y) = args_2(args)?;
    Ok(Value::Int(cast::int(x)? - cast::int(y)?))
}

pub fn div(args: &Value) -> Result<Value> {
    let (x, y) = args_2(args)?;
    Ok(Value::Int(cast::int(x)? / cast::int(y)?))
}

pub fn eq(args: &Value) -> Result<Value> {
    let (x, y) = args_2(args)?;
    todo!()
    // Ok(if x == y { x } else { Value::Nil })
}

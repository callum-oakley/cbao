use {
    crate::{
        args, cast,
        error::{Error, Result},
        value::{Env, Pair, Primitive, Value},
    },
    std::rc::Rc,
};

pub fn cons(args: &Value) -> Result<Value> {
    let (x, y) = args::get_2(args)?;
    Ok(Value::cons(x.clone(), y.clone()))
}

pub fn car(args: &Value) -> Result<Value> {
    Ok(cast::car(args::get_1(args)?)?.clone())
}

pub fn cdr(args: &Value) -> Result<Value> {
    Ok(cast::cdr(args::get_1(args)?)?.clone())
}

pub fn add(args: &Value) -> Result<Value> {
    let (x, y) = args::get_2(args)?;
    Ok(Value::Int(cast::int(x)? + cast::int(y)?))
}

pub fn mul(args: &Value) -> Result<Value> {
    let (x, y) = args::get_2(args)?;
    Ok(Value::Int(cast::int(x)? * cast::int(y)?))
}

pub fn sub(args: &Value) -> Result<Value> {
    let (x, y) = args::get_2(args)?;
    Ok(Value::Int(cast::int(x)? - cast::int(y)?))
}

pub fn div(args: &Value) -> Result<Value> {
    let (x, y) = args::get_2(args)?;
    Ok(Value::Int(cast::int(x)? / cast::int(y)?))
}

pub fn eq(args: &Value) -> Result<Value> {
    let (x, y) = args::get_2(args)?;
    todo!()
    // Ok(if x == y { x } else { Value::Nil })
}

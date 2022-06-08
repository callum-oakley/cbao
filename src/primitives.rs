use {
    crate::{
        error::{Error, Result},
        value::{Env, Pair, Primitive, Proc, Value},
    },
    std::rc::Rc,
};

fn as_int(v: &Value) -> Result<&i32> {
    match v {
        Value::Int(int) => Ok(int),
        _ => Err(Error::cast(v, "int")),
    }
}

fn as_pair(v: &Value) -> Result<&Pair> {
    match v {
        Value::Pair(pair) => Ok(pair),
        _ => Err(Error::cast(v, "pair")),
    }
}

fn args_1(args: &Value) -> &Value {
    match args {
        Value::Pair(pair) => pair.car(),
        _ => &Value::Nil,
    }
}

fn args_2(args: &Value) -> (&Value, &Value) {
    match args {
        Value::Pair(pair) => (pair.car(), args_1(pair.cdr())),
        _ => (&Value::Nil, &Value::Nil),
    }
}

pub fn cons(args: &Value) -> Result<Value> {
    let (x, y) = args_2(args);
    Ok(Value::cons(x.clone(), y.clone()))
}

pub fn car(args: &Value) -> Result<Value> {
    as_pair(args_1(args)).map(|p| p.car().clone())
}

pub fn cdr(args: &Value) -> Result<Value> {
    as_pair(args_1(args)).map(|p| p.cdr().clone())
}

pub fn add(args: &Value) -> Result<Value> {
    let (x, y) = args_2(args);
    Ok(Value::Int(as_int(x)? + as_int(y)?))
}

pub fn mul(args: &Value) -> Result<Value> {
    let (x, y) = args_2(args);
    Ok(Value::Int(as_int(x)? * as_int(y)?))
}

pub fn sub(args: &Value) -> Result<Value> {
    let (x, y) = args_2(args);
    Ok(Value::Int(as_int(x)? - as_int(y)?))
}

pub fn div(args: &Value) -> Result<Value> {
    let (x, y) = args_2(args);
    Ok(Value::Int(as_int(x)? / as_int(y)?))
}

pub fn eq(args: &Value) -> Result<Value> {
    let (x, y) = args_2(args);
    todo!()
    // Ok(if x == y { x } else { Value::Nil })
}

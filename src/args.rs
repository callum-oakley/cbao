use {
    crate::{
        cast,
        error::{Error, Result},
        value::Value,
    },
    std::collections::HashMap,
};

pub fn arg_tail(v: &Value) -> Result<&Value> {
    match v {
        Value::Nil => Ok(v),
        Value::Pair(pair) => Ok(pair.cdr()),
        _ => Err(Error::cast(v, "a pair or nil")),
    }
}

pub fn arg_0(v: &Value) -> Result<&Value> {
    match v {
        Value::Nil => Ok(v),
        Value::Pair(pair) => Ok(pair.car()),
        _ => Err(Error::cast(v, "a pair or nil")),
    }
}

pub fn arg_1(v: &Value) -> Result<&Value> {
    arg_0(arg_tail(v)?)
}

fn bind_list(
    mut params: &Value,
    mut args: &Value,
    frame: &mut HashMap<String, Value>,
) -> Result<()> {
    loop {
        bind(cast::car(params)?, arg_0(args)?, frame)?;
        params = cast::cdr(params)?;
        args = arg_tail(args)?;
        match params {
            Value::Nil => return Ok(()),
            Value::Sym(_) => return bind(params, args, frame),
            _ => (),
        }
    }
}

pub fn bind(params: &Value, args: &Value, frame: &mut HashMap<String, Value>) -> Result<()> {
    match params {
        Value::Sym(sym) => {
            frame.insert(sym.to_string(), args.clone());
            Ok(())
        }
        _ => bind_list(params, args, frame),
    }
}

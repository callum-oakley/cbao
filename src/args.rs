use {
    crate::{
        cast,
        error::{Error, Result},
        value::Value,
    },
    std::collections::HashMap,
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

fn bind_list(
    mut params: &Value,
    mut args: &Value,
    frame: &mut HashMap<String, Value>,
) -> Result<()> {
    loop {
        bind(cast::car(params)?, cast::car_or_nil(args)?, frame)?;
        params = cast::cdr(params)?;
        args = cast::cdr_or_nil(args)?;
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

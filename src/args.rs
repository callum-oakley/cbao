use {
    crate::{
        cast,
        error::{Error, ErrorData, Result},
        value::Value,
    },
    std::collections::HashMap,
};

pub fn arity(args: &Value, n: usize) -> Result<()> {
    if n == 0 && args.is_nil() {
        Ok(())
    } else if n == 0 {
        Err(Error::too_many_args(n))
    } else if args.is_nil() {
        Err(Error::too_few_args(n))
    } else {
        arity(cast::cdr(args)?, n - 1).map_err(|err| match err.data {
            ErrorData::TooManyArgs(_) => Error::too_many_args(n),
            ErrorData::TooFewArgs(_) => Error::too_few_args(n),
            _ => err,
        })
    }
}

fn bind_list(
    mut params: &Value,
    mut args: &Value,
    frame: &mut HashMap<String, Value>,
) -> Result<()> {
    loop {
        bind(cast::car(params)?, cast::car(args)?, frame)?;
        params = cast::cdr(params)?;
        args = cast::cdr(args)?;
        match params {
            Value::Nil => return Ok(()),
            Value::Sym(_) => return bind(params, args, frame),
            _ => (),
        }
    }
}

pub fn bind(params: &Value, args: &Value, frame: &mut HashMap<String, Value>) -> Result<()> {
    match params {
        Value::Nil => Ok(()),
        Value::Sym(sym) => {
            frame.insert(sym.to_string(), args.clone());
            Ok(())
        }
        _ => bind_list(params, args, frame).map_err(|err| Error::bind(params.clone()).source(err)),
    }
}

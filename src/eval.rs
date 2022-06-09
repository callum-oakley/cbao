use {
    crate::{
        args, cast,
        error::{Error, Result},
        primitives,
        value::{Closure, Env, Fn, Primitive, Value},
    },
    std::collections::HashMap,
};

fn apply_closure(closure: &Closure, args: &Value) -> Result<Value> {
    let mut frame = HashMap::new();
    args::bind(&closure.params, args, &mut frame)?;
    eval(cast::car(&closure.body)?, &closure.env.extend(frame))
}

fn apply(function: &Value, args: &Value) -> Result<Value> {
    match function {
        Value::Fn(Fn::Closure(closure)) => apply_closure(closure, args),
        Value::Fn(Fn::Primitive(primitive)) => match primitive {
            Primitive::Cons => primitives::cons(args),
            Primitive::Car => primitives::car(args),
            Primitive::Cdr => primitives::cdr(args),
            Primitive::Add => primitives::add(args),
            Primitive::Mul => primitives::mul(args),
            Primitive::Sub => primitives::sub(args),
            Primitive::Div => primitives::div(args),
            Primitive::Eq => primitives::eq(args),
        },
        _ => Err(Error::cast(function, "a fn")),
    }
}

fn eval_list(value: &Value, env: &Env) -> Result<Value> {
    match value {
        Value::Pair(pair) => Ok(Value::cons(
            eval(pair.car(), env)?,
            eval_list(pair.cdr(), env)?,
        )),
        _ => eval(value, env),
    }
}

fn eval_def(args: &Value, env: &Env) -> Result<Value> {
    let (x, y) = args::get_2(args)?;
    env.set(cast::sym(x)?.to_string(), eval(y, env)?);
    Ok(Value::Nil)
}

fn eval_fn(args: &Value, env: &Env) -> Result<Value> {
    Ok(Value::closure(
        cast::car(args)?.clone(),
        cast::cdr(args)?.clone(),
        env.clone(),
    ))
}

fn eval_if(mut args: &Value, env: &Env) -> Result<Value> {
    loop {
        if args.is_nil() {
            return Ok(Value::Nil);
        } else if cast::cdr(args)?.is_nil() {
            return eval(cast::car(args)?, env);
        } else if !eval(cast::car(args)?, env)?.is_nil() {
            return eval(cast::cadr(args)?, env);
        } else {
            args = cast::cddr(args)?;
        }
    }
}

pub fn eval(value: &Value, env: &Env) -> Result<Value> {
    match value {
        Value::Sym(sym) => env.get(sym).ok_or(Error::unknown_sym(value)),
        Value::Pair(pair) => {
            let car = pair.car();
            if let Value::Sym(sym) = car {
                match sym.as_str() {
                    "def" => return eval_def(pair.cdr(), env),
                    "fn" => return eval_fn(pair.cdr(), env),
                    "if" => return eval_if(pair.cdr(), env),
                    _ => (),
                }
            };
            apply(&eval(car, env)?, &eval_list(pair.cdr(), env)?)
        }
        _ => Ok(value.clone()),
    }
}

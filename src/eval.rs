use crate::{
    args, cast,
    error::{Error, Result},
    primitives,
    value::{Env, Primitive, Proc, Value},
};

fn apply(proc: &Value, args: &Value) -> Result<Value> {
    match proc {
        Value::Proc(Proc::Closure(closure)) => todo!(),
        Value::Proc(Proc::Primitive(primitive)) => match primitive {
            Primitive::Cons => primitives::cons(args),
            Primitive::Car => primitives::car(args),
            Primitive::Cdr => primitives::cdr(args),
            Primitive::Add => primitives::add(args),
            Primitive::Mul => primitives::mul(args),
            Primitive::Sub => primitives::sub(args),
            Primitive::Div => primitives::div(args),
            Primitive::Eq => primitives::eq(args),
        },
        _ => Err(Error::cast(proc, "a proc")),
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

fn eval_def(args: &Value, env: &Env) -> Result<Value> {
    let (x, y) = args::get_2(args)?;
    env.set(cast::sym(x)?.to_string(), eval(y, env)?);
    Ok(Value::Nil)
}

pub fn eval(value: &Value, env: &Env) -> Result<Value> {
    match value {
        Value::Sym(sym) => env.get(sym).ok_or(Error::unknown_sym(value)),
        Value::Pair(pair) => {
            let car = pair.car();
            if let Value::Sym(sym) = car {
                match sym.as_str() {
                    "if" => return eval_if(pair.cdr(), env),
                    "def" => return eval_def(pair.cdr(), env),
                    _ => (),
                }
            };
            apply(&eval(car, env)?, &eval_list(pair.cdr(), env)?)
        }
        _ => Ok(value.clone()),
    }
}

use crate::{
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
        _ => Err(Error::cast(proc, "proc")),
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

pub fn eval(value: &Value, env: &Env) -> Result<Value> {
    match value {
        Value::Sym(sym) => env.get(sym).ok_or(Error::unknown_sym(value)),
        Value::Pair(pair) => apply(&eval(pair.car(), env)?, &eval_list(pair.cdr(), env)?),
        _ => Ok(value.clone()),
    }
}

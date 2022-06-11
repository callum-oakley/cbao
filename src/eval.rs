use {
    crate::{
        args, cast,
        error::{Error, Result},
        primitives,
        value::{Closure, Env, Fn, Primitive, Value},
    },
    std::collections::HashMap,
};

fn quasiquote(arg: &Value) -> Result<Value> {
    match arg {
        Value::Sym(_) => Ok(Value::cons(
            Value::sym("quote".to_string()),
            Value::cons(arg.clone(), Value::Nil),
        )),
        Value::Pair(pair) => {
            let car = pair.car();
            match car {
                Value::Sym(sym) if sym.as_str() == "unquote" => {
                    return Ok(cast::car(pair.cdr())?.clone());
                }
                Value::Pair(inner_pair) => {
                    if let Value::Sym(sym) = inner_pair.car() {
                        if sym.as_str() == "splice-unquote" {
                            return Ok(Value::cons(
                                Value::sym("cat".to_string()),
                                Value::cons(
                                    cast::car(inner_pair.cdr())?.clone(),
                                    Value::cons(quasiquote(pair.cdr())?, Value::Nil),
                                ),
                            ));
                        }
                    }
                }
                _ => (),
            }
            Ok(Value::cons(
                Value::sym("cons".to_string()),
                Value::cons(
                    quasiquote(car)?,
                    Value::cons(quasiquote(pair.cdr())?, Value::Nil),
                ),
            ))
        }
        _ => Ok(arg.clone()),
    }
}

fn apply_closure(closure: &Closure, args: &Value) -> Result<Value> {
    let mut frame = HashMap::new();
    args::bind(&closure.params, args, &mut frame)?;
    let env = closure.env.extend(frame);
    let mut body = &closure.body;
    while !cast::cdr(body)?.is_nil() {
        eval(cast::car(body)?.clone(), &env)?;
        body = cast::cdr(body)?;
    }
    eval(cast::car(body)?.clone(), &env)
}

fn apply(function: &Value, args: &Value) -> Result<Value> {
    match function {
        Value::Fn(Fn::Closure(closure), _) => apply_closure(closure, args),
        Value::Fn(Fn::Primitive(primitive), _) => match primitive {
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
            eval(pair.car().clone(), env)?,
            eval_list(pair.cdr(), env)?,
        )),
        _ => eval(value.clone(), env),
    }
}

fn eval_def(args: &Value, env: &Env) -> Result<Value> {
    match cast::car(args)? {
        Value::Pair(pair) => env.set(
            cast::sym(pair.car())?.to_string(),
            eval_fn(pair.cdr(), cast::cdr(args)?, env)?,
        ),
        v => env.set(
            cast::sym(v)?.to_string(),
            eval(cast::cadr(args)?.clone(), env)?,
        ),
    };
    Ok(Value::Nil)
}

fn eval_defmacro(args: &Value, env: &Env) -> Result<Value> {
    let arg_0 = cast::car(args)?;
    env.set(
        cast::car(arg_0)?.to_string(),
        Value::as_macro(
            cast::function(&eval_fn(cast::cdr(arg_0)?, cast::cdr(args)?, env)?)?.clone(),
        ),
    );
    Ok(Value::Nil)
}

fn eval_fn(params: &Value, body: &Value, env: &Env) -> Result<Value> {
    Ok(Value::closure(params.clone(), body.clone(), env.clone()))
}

fn eval_if(mut args: &Value, env: &Env) -> Result<Value> {
    loop {
        if args.is_nil() {
            return Ok(Value::Nil);
        } else if cast::cdr(args)?.is_nil() {
            return eval(cast::car(args)?.clone(), env);
        } else if !eval(cast::car(args)?.clone(), env)?.is_nil() {
            return eval(cast::cadr(args)?.clone(), env);
        } else {
            args = cast::cddr(args)?;
        }
    }
}

fn macroexpand(mut value: Value, env: &Env) -> Result<Value> {
    loop {
        if let Value::Pair(ref pair) = value {
            if let Value::Sym(sym) = pair.car() {
                if let Some(f) = env.get(sym) {
                    if let Value::Fn(_, true) = f {
                        value = apply(&f, pair.cdr())?;
                        continue;
                    }
                }
            }
        }
        return Ok(value);
    }
}

pub fn eval(mut value: Value, env: &Env) -> Result<Value> {
    value = macroexpand(value.clone(), env)?;
    match value {
        Value::Sym(sym) => env.get(&sym).ok_or_else(|| Error::unknown_sym(&sym)),
        Value::Pair(pair) => {
            let car = pair.car();
            if let Value::Sym(sym) = car {
                match sym.as_str() {
                    "def" => return eval_def(pair.cdr(), env),
                    "defmacro" => return eval_defmacro(pair.cdr(), env),
                    "fn" => return eval_fn(cast::car(pair.cdr())?, cast::cdr(pair.cdr())?, env),
                    "if" => return eval_if(pair.cdr(), env),
                    "quasiquote" => return eval(quasiquote(cast::car(pair.cdr())?)?, env),
                    "quote" => return Ok(cast::car(pair.cdr())?.clone()),
                    "macroexpand" => return macroexpand(cast::car(pair.cdr())?.clone(), env),
                    _ => (),
                }
            };
            apply(&eval(car.clone(), env)?, &eval_list(pair.cdr(), env)?)
                .map_err(|err| Error::function(car.clone()).source(err))
        }
        _ => Ok(value.clone()),
    }
}

use {
    crate::{
        error::{Error, Result},
        value::{Primitive, Value},
    },
    std::collections::HashMap,
};

pub type Env = HashMap<String, Value>;

pub fn prelude() -> Env {
    let mut env = HashMap::new();
    env.insert("+".to_string(), Value::Primitive(Primitive::Plus));
    env.insert("*".to_string(), Value::Primitive(Primitive::Star));
    env.insert("-".to_string(), Value::Primitive(Primitive::Minus));
    env.insert("/".to_string(), Value::Primitive(Primitive::Slash));
    env
}

pub fn eval(value: Value, env: &Env) -> Result<Value> {
    match value {
        Value::Sym(sym) => match env.get(&sym) {
            Some(v) => Ok(v.clone()),
            None => Err(Error::UnknownSymbol { target: sym }),
        },
        Value::List(ref list) => {
            if list.is_empty() {
                Ok(value.clone())
            } else {
                apply(
                    eval(list[0].clone(), env)?,
                    list[1..]
                        .iter()
                        .map(|v| eval(v.clone(), env))
                        .collect::<Result<_>>()?,
                )
            }
        }
        _ => Ok(value),
    }
}

fn apply(f: Value, args: Vec<Value>) -> Result<Value> {
    match f {
        Value::Primitive(p) => p.apply(args),
        _ => Err(Error::NotFn { target: f }),
    }
}

// Not sure it actually makes sense for this to be a trait. We need to check if a type implements
// it at runtime, it's a Bao concept, not a Rust one.
trait Apply {
    fn apply(&self, args: Vec<Value>) -> Result<Value>;
}

// TODO at some point we have to check types and unwrap them to apply primitives, I haven't decided
// where yet.
fn unwrap_int(value: Value) -> i32 {
    match value {
        Value::Int(int) => int,
        _ => todo!(),
    }
}

impl Apply for Primitive {
    fn apply(&self, args: Vec<Value>) -> Result<Value> {
        let args: Vec<i32> = args.into_iter().map(unwrap_int).collect();
        Ok(Value::Int(match self {
            Primitive::Plus => args.iter().sum(),
            Primitive::Star => args.iter().product(),
            Primitive::Minus => match args.len() {
                0 => 0,
                1 => -args[0],
                _ => args[0] - args[1..].iter().sum::<i32>(),
            },
            // TODO this should return a ratio
            Primitive::Slash => match args.len() {
                0 => 1,
                1 => 1 / args[0],
                _ => args[0] / args[1..].iter().product::<i32>(),
            },
        }))
    }
}

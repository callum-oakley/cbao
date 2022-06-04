use {
    crate::{
        error::{Error, Result},
        value::{Closure, Env, Primitive, Value},
    },
    std::rc::Rc,
};

fn eval_def(args: &[Value], env: &Env) -> Result<Value> {
    if args.len() % 2 == 1 {
        Err(Error::Arity {
            target: "def".to_string(),
            n: args.len(),
        })
    } else {
        for pair in args.chunks(2) {
            let value = eval(pair[1].clone(), env)?;
            env.set(unwrap_sym(pair[0].clone()), value);
        }
        Ok(Value::Nil)
    }
}

fn eval_fn(args: &[Value], env: &Env) -> Result<Value> {
    if args.len() < 2 {
        Err(Error::Arity {
            target: "fn".to_string(),
            n: args.len(),
        })
    } else {
        Ok(Value::Closure(Rc::new(Closure {
            args: unwrap_vec(args[0].clone())
                .iter()
                .cloned()
                .map(unwrap_sym)
                .collect(),
            body: args[1..].iter().cloned().collect(),
            env: env.clone(),
        })))
    }
}

pub fn eval(value: Value, env: &Env) -> Result<Value> {
    match value {
        Value::Sym(sym) => match env.get(&sym) {
            Some(v) => Ok(v),
            None => Err(Error::UnknownSymbol { target: sym }),
        },
        Value::List(ref list) => {
            if list.is_empty() {
                return Ok(value.clone());
            }
            if let Value::Sym(sym) = &list[0] {
                match sym.as_str() {
                    "def" => return eval_def(&list[1..], env),
                    "fn" => return eval_fn(&list[1..], env),
                    _ => (),
                }
            }
            apply(
                eval(list[0].clone(), env)?,
                list[1..]
                    .iter()
                    .cloned()
                    .map(|v| eval(v, env))
                    .collect::<Result<_>>()?,
            )
        }
        Value::Vec(ref vec) => vec
            .iter()
            .cloned()
            .map(|v| eval(v, env))
            .collect::<Result<_>>()
            .map(|vec| Value::Vec(Rc::new(vec))),
        _ => Ok(value),
    }
}

fn apply(f: Value, args: Vec<Value>) -> Result<Value> {
    match f {
        Value::Primitive(p) => p.apply(args),
        Value::Closure(c) => c.apply(args),
        _ => Err(Error::NotFn { target: f }),
    }
}

// Not sure it actually makes sense for this to be a trait. We need to check if a type implements
// it at runtime, it's a Bao concept, not a Rust one.
trait Apply {
    fn apply(&self, args: Vec<Value>) -> Result<Value>;
}

// TODO at some point we have to check types and unwrap them to apply primitives, I haven't decided
// where yet. Maybe these unwrap functions should return Result<_>?

fn unwrap_int(value: Value) -> i32 {
    match value {
        Value::Int(int) => int,
        _ => todo!(),
    }
}

fn unwrap_sym(value: Value) -> String {
    match value {
        Value::Sym(sym) => sym,
        _ => todo!(),
    }
}

fn unwrap_vec(value: Value) -> Rc<Vec<Value>> {
    match value {
        Value::Vec(vec) => vec,
        _ => todo!(),
    }
}

impl Apply for Closure {
    fn apply(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != self.args.len() {
            // TODO refer to the closure by name if it has one
            // TODO show expected number of args
            Err(Error::Arity {
                target: "<closure>".to_string(),
                n: args.len(),
            })
        } else {
            let env = self
                .env
                .extend(self.args.iter().cloned().zip(args).collect());
            for i in 0..self.body.len() - 1 {
                eval(self.body[i].clone(), &env)?;
            }
            eval(self.body[self.body.len() - 1].clone(), &env)
        }
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

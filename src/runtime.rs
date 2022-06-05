use {
    crate::{
        error::{Error, ErrorData, Result},
        value::{Closure, Env, Primitive, Value},
    },
    std::rc::Rc,
};

fn eval_def(args: &[Value], env: &Env) -> Result<Value> {
    if args.len() % 2 == 1 {
        Err(Error::new(ErrorData::Todo))
    } else {
        for pair in args.chunks(2) {
            let value = eval(pair[1].clone(), env)?;
            env.set(as_sym(pair[0].clone())?, value);
        }
        Ok(Value::Nil)
    }
}

fn eval_fn(args: &[Value], env: &Env) -> Result<Value> {
    if args.len() < 2 {
        Err(Error::new(ErrorData::Todo))
    } else {
        Ok(Value::Closure(Rc::new(Closure {
            params: as_vec(args[0].clone())?
                .iter()
                .cloned()
                .map(as_sym)
                .collect::<Result<_>>()?,
            body: args[1..].iter().cloned().collect(),
            env: env.clone(),
        })))
    }
}

fn eval_if(args: &[Value], env: &Env) -> Result<Value> {
    for chunk in args.chunks(2) {
        if chunk.len() == 1 {
            return eval(chunk[0].clone(), env);
        }
        if eval(chunk[0].clone(), env)?.truthy() {
            return eval(chunk[1].clone(), env);
        }
    }
    todo!()
}

pub fn eval(value: Value, env: &Env) -> Result<Value> {
    match value {
        Value::Sym(sym, meta) => match env.get(&sym) {
            Some(v) => Ok(v),
            None => Err(Error::new(ErrorData::UnknownSym(sym)).with(meta)),
        },
        Value::List(ref list, ref meta) => {
            if list.is_empty() {
                return Ok(value.clone());
            }
            if let Value::Sym(sym, _) = &list[0] {
                match sym.as_str() {
                    "def" => return eval_def(&list[1..], env),
                    "fn" => return eval_fn(&list[1..], env),
                    "if" => return eval_if(&list[1..], env),
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
            .map_err(|err| {
                err.wrap(ErrorData::Apply(list[0].clone()))
                    .with(meta.clone())
            })
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
        _ => Err(Error::new(ErrorData::Todo)),
    }
}

// Not sure it actually makes sense for this to be a trait. We need to check if a type implements
// it at runtime, it's a Bao concept, not a Rust one.
trait Apply {
    fn apply(&self, args: Vec<Value>) -> Result<Value>;
}

impl Apply for Closure {
    fn apply(&self, args: Vec<Value>) -> Result<Value> {
        if args.len() != self.params.len() {
            Err(Error::new(ErrorData::Todo))
        } else {
            let env = self
                .env
                .extend(self.params.iter().cloned().zip(args).collect());
            for i in 0..self.body.len() - 1 {
                eval(self.body[i].clone(), &env)?;
            }
            eval(self.body[self.body.len() - 1].clone(), &env)
        }
    }
}

impl Apply for Primitive {
    fn apply(&self, args: Vec<Value>) -> Result<Value> {
        let args: Vec<i32> = args.into_iter().map(as_int).collect::<Result<_>>()?;
        Ok(match self {
            Primitive::Plus => Value::Int(args.iter().sum()),
            Primitive::Star => Value::Int(args.iter().product()),
            Primitive::Minus => Value::Int(match args.len() {
                0 => 0,
                1 => -args[0],
                _ => args[0] - args[1..].iter().sum::<i32>(),
            }),
            // TODO this should return a ratio
            Primitive::Slash => Value::Int(match args.len() {
                0 => 1,
                1 => 1 / args[0],
                _ => args[0] / args[1..].iter().product::<i32>(),
            }),
            Primitive::Eq => Value::Bool(match args.len() {
                0 => true,
                _ => args[1..].iter().all(|v| *v == args[0]),
            }),
        })
    }
}

fn as_bool(value: Value) -> Result<bool> {
    match value {
        Value::Bool(bool) => Ok(bool),
        _ => Err(Error::new(ErrorData::Type(value, "Bool".to_string()))),
    }
}

fn as_int(value: Value) -> Result<i32> {
    match value {
        Value::Int(int) => Ok(int),
        _ => Err(Error::new(ErrorData::Type(value, "Int".to_string()))),
    }
}

fn as_sym(value: Value) -> Result<String> {
    match value {
        Value::Sym(sym, _) => Ok(sym),
        _ => Err(Error::new(ErrorData::Type(value, "Sym".to_string()))),
    }
}

fn as_list(value: Value) -> Result<Rc<Vec<Value>>> {
    match value {
        Value::List(list, _) => Ok(list),
        _ => Err(Error::new(ErrorData::Type(value, "List".to_string()))),
    }
}

fn as_vec(value: Value) -> Result<Rc<Vec<Value>>> {
    match value {
        Value::Vec(vec) => Ok(vec),
        _ => Err(Error::new(ErrorData::Type(value, "Vec".to_string()))),
    }
}

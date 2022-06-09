use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

#[derive(Debug)]
pub struct Closure {
    pub params: Value,
    pub body: Value,
    pub env: Env,
}

#[derive(Debug, Clone)]
pub enum Primitive {
    Cons,
    Car,
    Cdr,
    Add,
    Mul,
    Sub,
    Div,
    Eq,
}

#[derive(Debug, Clone)]
pub struct Pair(Value, Value);

#[derive(Debug, Clone)]
pub enum Fn {
    Closure(Rc<Closure>),
    Primitive(Primitive),
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Int(i32),
    Sym(Rc<String>),
    Pair(Rc<Pair>),
    Fn(Fn),
}

impl Value {
    pub fn cons(x: Value, y: Value) -> Value {
        Value::Pair(Rc::new(Pair(x, y)))
    }

    pub fn sym(s: String) -> Value {
        Value::Sym(Rc::new(s))
    }

    pub fn primitive(p: Primitive) -> Value {
        Value::Fn(Fn::Primitive(p))
    }

    pub fn closure(params: Value, body: Value, env: Env) -> Value {
        Value::Fn(Fn::Closure(Rc::new(Closure { params, body, env })))
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Value::Nil => true,
            _ => false,
        }
    }
}

impl Pair {
    pub fn car(&self) -> &Value {
        let Pair(ref x, _) = self;
        x
    }

    pub fn cdr(&self) -> &Value {
        let Pair(_, ref y) = self;
        y
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Int(int) => write!(f, "{}", int),
            Value::Sym(sym) => write!(f, "{}", sym),
            Value::Pair(ref pair) => {
                let mut pair = pair;
                write!(f, "(")?;
                loop {
                    write!(f, "{}", pair.car())?;
                    match pair.cdr() {
                        Value::Nil => {
                            break;
                        }
                        Value::Pair(p) => {
                            write!(f, " ")?;
                            pair = p;
                        }
                        _ => {
                            write!(f, " . {}", pair.cdr())?;
                            break;
                        }
                    }
                }
                write!(f, ")")
            }
            Value::Fn(_) => write!(f, "<fn>"),
        }
    }
}

#[derive(Debug)]
struct EnvData {
    frame: HashMap<String, Value>,
    outer: Option<Env>,
}

#[derive(Debug, Clone)]
pub struct Env(Rc<RefCell<EnvData>>);

impl Env {
    pub fn prelude() -> Env {
        Env(Rc::new(RefCell::new(EnvData {
            frame: vec![
                ("nil".to_string(), Value::Nil),
                ("cons".to_string(), Value::primitive(Primitive::Cons)),
                ("car".to_string(), Value::primitive(Primitive::Car)),
                ("cdr".to_string(), Value::primitive(Primitive::Cdr)),
                ("+".to_string(), Value::primitive(Primitive::Add)),
                ("*".to_string(), Value::primitive(Primitive::Mul)),
                ("-".to_string(), Value::primitive(Primitive::Sub)),
                ("=".to_string(), Value::primitive(Primitive::Eq)),
            ]
            .into_iter()
            .collect(),
            outer: None,
        })))
    }

    pub fn extend(&self, frame: HashMap<String, Value>) -> Env {
        Env(Rc::new(RefCell::new(EnvData {
            frame,
            outer: Some(self.clone()),
        })))
    }

    pub fn set(&self, sym: String, value: Value) {
        self.0.borrow_mut().frame.insert(sym, value);
    }

    pub fn get(&self, sym: &str) -> Option<Value> {
        let env_data = self.0.borrow();
        env_data
            .frame
            .get(sym)
            .cloned()
            .or_else(|| env_data.outer.as_ref().and_then(|outer| outer.get(sym)))
    }
}

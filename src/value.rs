use std::{cell::RefCell, cmp::Ordering, collections::HashMap, fmt, rc::Rc};

#[derive(Debug)]
pub struct Closure {
    pub params: Value,
    pub body: Value,
    pub env: Env,
}

impl PartialEq for Closure {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl PartialOrd for Closure {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Primitive {
    Cons,
    Car,
    Cdr,
    Add,
    Mul,
    Sub,
    Div,
    Eq,
    Lt,
    Lte,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Pair(Value, Value);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Fn {
    Closure(Rc<Closure>),
    Primitive(Primitive),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Nil,
    Int(i32),
    Sym(Rc<String>),
    Pair(Rc<Pair>),
    Fn(Fn, bool),
}

impl Value {
    pub fn cons(x: Value, y: Value) -> Value {
        Value::Pair(Rc::new(Pair(x, y)))
    }

    // TODO find all static symbol definitions and refer to the same one.
    pub fn sym(s: String) -> Value {
        Value::Sym(Rc::new(s))
    }

    pub fn primitive(p: Primitive) -> Value {
        Value::Fn(Fn::Primitive(p), false)
    }

    pub fn closure(params: Value, body: Value, env: Env) -> Value {
        Value::Fn(Fn::Closure(Rc::new(Closure { params, body, env })), false)
    }

    pub fn as_macro(f: Fn) -> Value {
        Value::Fn(f, true)
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Value::Nil)
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
            Value::Fn(v, _) => match v {
                Fn::Closure(_) => write!(f, "<closure>"),
                Fn::Primitive(p) => write!(f, "<primitive: {:?}>", p),
            },
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
                ("primitive/nil".to_string(), Value::Nil),
                (
                    "primitive/cons".to_string(),
                    Value::primitive(Primitive::Cons),
                ),
                (
                    "primitive/car".to_string(),
                    Value::primitive(Primitive::Car),
                ),
                (
                    "primitive/cdr".to_string(),
                    Value::primitive(Primitive::Cdr),
                ),
                (
                    "primitive/add".to_string(),
                    Value::primitive(Primitive::Add),
                ),
                (
                    "primitive/mul".to_string(),
                    Value::primitive(Primitive::Mul),
                ),
                (
                    "primitive/sub".to_string(),
                    Value::primitive(Primitive::Sub),
                ),
                (
                    "primitive/div".to_string(),
                    Value::primitive(Primitive::Div),
                ),
                ("primitive/eq".to_string(), Value::primitive(Primitive::Eq)),
                ("primitive/lt".to_string(), Value::primitive(Primitive::Lt)),
                (
                    "primitive/lte".to_string(),
                    Value::primitive(Primitive::Lte),
                ),
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

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

#[derive(Debug, Clone)]
pub enum Primitive {
    Cons,
    Car,
    Cdr,
    Add,
    Mul,
    Sub,
    Eq,
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i32),
    Sym(String),
    Pair(Rc<Value>, Rc<Value>),
    Function {
        params: Rc<Value>,
        body: Rc<Value>,
        env: Env,
    },
    Primitive(Primitive),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::Int(int) => write!(f, "{}", int),
            Value::Sym(sym) => write!(f, "{}", sym),
            Value::Pair(car, cdr) => write!(f, "({car} . {cdr})"),
            Value::Function { .. } => write!(f, "<function>"),
            Value::Primitive(p) => write!(f, "<primitive {:?}>", p),
        }
    }
}

#[derive(Debug, Clone)]
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
                ("true".to_string(), Value::Bool(true)),
                ("false".to_string(), Value::Bool(false)),
                ("cons".to_string(), Value::Primitive(Primitive::Cons)),
                ("car".to_string(), Value::Primitive(Primitive::Car)),
                ("cdr".to_string(), Value::Primitive(Primitive::Cdr)),
                ("+".to_string(), Value::Primitive(Primitive::Add)),
                ("*".to_string(), Value::Primitive(Primitive::Mul)),
                ("-".to_string(), Value::Primitive(Primitive::Sub)),
                ("=".to_string(), Value::Primitive(Primitive::Eq)),
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

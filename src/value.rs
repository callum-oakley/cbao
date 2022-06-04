use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

#[derive(Debug, Clone)]
pub enum Primitive {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug)]
pub struct Closure {
    pub args: Vec<String>,
    pub body: Vec<Value>,
    pub env: Env,
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Int(i32),
    Sym(String),
    List(Rc<Vec<Value>>),
    Vec(Rc<Vec<Value>>),
    Closure(Rc<Closure>),
    Primitive(Primitive),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Int(int) => write!(f, "{}", int),
            Value::Sym(sym) => write!(f, "{}", sym),
            Value::List(list) => write_coll(f, "(", list.iter().cloned(), ")"),
            Value::Vec(vec) => write_coll(f, "[", vec.iter().cloned(), "]"),
            Value::Closure(_) => write!(f, "<closure>"),
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
        let env = Env(Rc::new(RefCell::new(EnvData {
            frame: HashMap::new(),
            outer: None,
        })));
        env.set("+".to_string(), Value::Primitive(Primitive::Plus));
        env.set("*".to_string(), Value::Primitive(Primitive::Star));
        env.set("-".to_string(), Value::Primitive(Primitive::Minus));
        env.set("/".to_string(), Value::Primitive(Primitive::Slash));
        env
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

    pub fn extend(&self, frame: HashMap<String, Value>) -> Env {
        Env(Rc::new(RefCell::new(EnvData {
            frame,
            outer: Some(self.clone()),
        })))
    }
}

fn write_coll<I>(f: &mut fmt::Formatter<'_>, open: &str, mut coll: I, close: &str) -> fmt::Result
where
    I: Iterator<Item = Value>,
{
    write!(f, "{}", open)?;
    if let Some(element) = coll.next() {
        write!(f, "{}", element)?;
        for element in coll {
            write!(f, " {}", element)?;
        }
    }
    write!(f, "{}", close)
}

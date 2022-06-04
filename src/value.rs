use std::{fmt, rc::Rc};

#[derive(Debug, Clone)]
pub enum Primitive {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Int(i32),
    Sym(String),
    List(Rc<Vec<Value>>),
    Primitive(Primitive),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Int(int) => write!(f, "{}", int),
            Value::Sym(sym) => write!(f, "{}", sym),
            Value::List(list) => {
                let mut list = list.iter();
                write!(f, "(")?;
                if let Some(element) = list.next() {
                    write!(f, "{}", element)?;
                    for element in list {
                        write!(f, " {}", element)?;
                    }
                }
                write!(f, ")")
            }
            Value::Primitive(p) => write!(f, "(primitive {:?})", p),
        }
    }
}

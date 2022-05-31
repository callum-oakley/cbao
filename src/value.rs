use std::fmt;

#[derive(Debug)]
pub enum Value {
    Int(i32),
    Symbol(String),
    List(Vec<Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(int) => write!(f, "{}", int),
            Value::Symbol(sym) => write!(f, "{}", sym),
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
        }
    }
}

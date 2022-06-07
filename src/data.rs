use {
    crate::{error::Result, value::Value},
    std::rc::Rc,
};

pub fn cons(x: Value, y: Value) -> Value {
    Value::Pair(Rc::new(x), Rc::new(y))
}

use {
    crate::value::{Primitive, Value},
    std::collections::HashMap,
};

pub struct Env<'a> {
    frame: HashMap<String, Value>,
    outer: Option<&'a Env<'a>>,
}

impl<'a> Env<'a> {
    pub fn prelude() -> Env<'static> {
        let mut env = Env {
            frame: HashMap::new(),
            outer: None,
        };
        env.set("+".to_string(), Value::Primitive(Primitive::Plus));
        env.set("*".to_string(), Value::Primitive(Primitive::Star));
        env.set("-".to_string(), Value::Primitive(Primitive::Minus));
        env.set("/".to_string(), Value::Primitive(Primitive::Slash));
        env
    }

    pub fn set(&mut self, sym: String, value: Value) {
        self.frame.insert(sym, value);
    }

    pub fn get(&self, sym: &str) -> Option<&Value> {
        self.frame
            .get(sym)
            .or_else(|| self.outer.and_then(|outer| outer.get(sym)))
    }

    pub fn extend(&self) -> Env {
        Env {
            frame: HashMap::new(),
            outer: Some(self),
        }
    }
}

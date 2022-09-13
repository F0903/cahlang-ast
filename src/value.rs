use std::fmt::{Debug, Display};

use crate::{environment::Env, interpreter::Interpreter};

#[derive(Debug, Clone)]
pub struct NativeFunc {
    name: String,
    arg_count: usize,
    func: fn(env: Env, args: Vec<Value>) -> Value,
}

impl NativeFunc {
    pub fn new(
        name: impl ToString,
        arg_count: usize,
        func: fn(env: Env, args: Vec<Value>) -> Value,
    ) -> Self {
        Self {
            name: name.to_string(),
            arg_count,
            func,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl CallableClone for NativeFunc {
    fn clone_box(&self) -> Box<dyn Callable> {
        Box::new(self.clone())
    }
}

impl Callable for NativeFunc {
    fn call(&self, interpreter: &Interpreter, args: Vec<Value>) -> Value {
        (self.func)(interpreter.get_current_env(), args)
    }

    fn get_arity(&self) -> usize {
        self.arg_count
    }
}

pub trait CallableClone {
    fn clone_box(&self) -> Box<dyn Callable>;
}

pub trait Callable: CallableClone + Debug {
    fn call(&self, interpreter: &Interpreter, args: Vec<Value>) -> Value;
    fn get_arity(&self) -> usize;
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Callable(Box<dyn Callable>),
    None,
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Self::String(x) => Self::String(x.clone()),
            Self::Number(x) => Self::Number(x.clone()),
            Self::Boolean(x) => Self::Boolean(x.clone()),
            Self::Callable(x) => Self::Callable(x.clone_box()),
            Self::None => Self::None,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(x) => f.write_fmt(format_args!("{x}")),
            Value::Number(x) => f.write_fmt(format_args!("{x}")),
            Value::Boolean(x) => f.write_fmt(format_args!("{x}")),
            Value::Callable(_) => f.write_str("<function>"),
            Value::None => f.write_str("none"),
        }
    }
}

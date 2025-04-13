use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::errors::PluginError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Signature {
    pub callee: String,
    pub command_name: String,
    pub return_type: Type,
    pub args: Vec<(String, Type)>,
    pub command_type: CommandType,
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CommandType {
    InlineCommand,
    BlockCommand,
    Macro,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Type {
    String,
    Integer,
    Float,
    Boolean,
    List(Box<Type>),
    Tuple(Vec<Type>),
    Option(Box<Type>),
    Record(HashMap<String, Type>),
    Varargs(Box<Type>),
    IR,
    Invalid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value<T: fmt::Display> {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<Value<T>>),
    Tuple(Vec<Value<T>>),
    Option(Option<Box<Value<T>>>),
    Record(HashMap<String, Value<T>>),
    Varargs(Vec<Value<T>>),
    IR(T),
}

impl<T: fmt::Display> Value<T> {
    pub fn to_type(&self) -> Type {
        match self {
            Value::String(_) => Type::String,
            Value::Integer(_) => Type::Integer,
            Value::Float(_) => Type::Float,
            Value::Boolean(_) => Type::Boolean,
            Value::List(l) => Type::List(Box::new(l[0].to_type())),
            Value::Tuple(t) => Type::Tuple(t.iter().map(|v| v.to_type()).collect()),
            Value::Option(o) => Type::Option(Box::new(o.as_ref().unwrap().to_type())),
            Value::Record(r) => {
                let mut map = HashMap::new();
                for (k, v) in r.iter() {
                    map.insert(k.clone(), v.to_type());
                }
                Type::Record(map)
            }
            Value::Varargs(v) => Type::Varargs(Box::new(v[0].to_type())),
            Value::IR(_) => Type::IR,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueVec<T: fmt::Display>(pub Vec<Value<T>>);

impl<T: fmt::Display> fmt::Display for ValueVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (i, v) in self.0.iter().enumerate() {
            if i > 0 {
                s.push_str(", ");
            }
            s.push_str(&v.to_string());
        }
        write!(f, "[{}]", s)
    }
}

impl<T: fmt::Display> ValueVec<T> {
    pub fn new() -> Self {
        ValueVec(Vec::new())
    }
}

impl<T: fmt::Display> fmt::Display for Value<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::List(l) => {
                let mut s = String::new();
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&v.to_string());
                }
                write!(f, "#({})", s)
            }
            Value::Tuple(t) => {
                let mut s = String::new();
                for (i, v) in t.iter().enumerate() {
                    if i > 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&v.to_string());
                }
                write!(f, "!({})", s)
            }
            Value::Option(o) => {
                if let Some(v) = o {
                    write!(f, "{}", v)
                } else {
                    write!(f, "None")
                }
            }
            Value::Record(r) => {
                let mut s = String::new();
                for (k, v) in r.iter() {
                    if !s.is_empty() {
                        s.push_str(", ");
                    }
                    s.push_str(&format!("{}: {}", k, v));
                }
                write!(f, "({})", s)
            }
            Value::Varargs(v) => {
                let mut s = String::new();
                for (i, v) in v.iter().enumerate() {
                    if i > 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&v.to_string());
                }
                write!(f, "...({})", s)
            }
            Value::IR(ir) => write!(f, "{}", ir),
        }
    }
}

pub trait Plugin {
    type IR: Serialize + for<'de> Deserialize<'de> + fmt::Display;

    fn call(
        &mut self,
        signature: &Signature,
        args: &ValueVec<Self::IR>,
    ) -> Result<Value<Self::IR>, PluginError>;
}

pub type Plugins<IR> = HashMap<String, Box<dyn Plugin<IR = IR>>>;

use core::fmt;
use std::{collections::HashMap, ops::Index};

use serde::{Deserialize, Serialize};

use crate::{
    errors::PluginError,
    ir::{IRKind, IR},
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub callee: String,
    pub command_name: String,
    pub return_type: Type,
    pub args: Vec<(String, Type)>,
    pub command_type: CommandType,
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (i, (k, v)) in self.args.iter().enumerate() {
            if i > 0 {
                s.push_str(", ");
            }
            s.push_str(&format!("{}: {}", k, v));
        }
        write!(
            f,
            "{} {}({}) -> {}",
            self.command_type, self.command_name, s, self.return_type
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CommandType {
    InlineCommand,
    BlockCommand,
    Macro,
}

impl fmt::Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandType::InlineCommand => write!(f, "inline"),
            CommandType::BlockCommand => write!(f, "block"),
            CommandType::Macro => write!(f, "macro"),
        }
    }
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

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::String => write!(f, "string"),
            Type::Integer => write!(f, "integer"),
            Type::Float => write!(f, "float"),
            Type::Boolean => write!(f, "boolean"),
            Type::List(t) => write!(f, "[{}]", t),
            Type::Tuple(t) => {
                let mut s = String::new();
                for (i, v) in t.iter().enumerate() {
                    if i > 0 {
                        s.push_str(", ");
                    }
                    s.push_str(&v.to_string());
                }
                write!(f, "!({})", s)
            }
            Type::Option(t) => write!(f, "Option<{}>", t),
            Type::Record(r) => {
                let mut s = String::new();
                for (k, v) in r.iter() {
                    if !s.is_empty() {
                        s.push_str(", ");
                    }
                    s.push_str(&format!("{}: {}", k, v));
                }
                write!(f, "{{{}}}", s)
            }
            Type::Varargs(t) => write!(f, "...[{}]", t),
            Type::IR => write!(f, "IR"),
            Type::Invalid => write!(f, "Invalid"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    List(Vec<Value>),
    Tuple(Vec<Value>),
    Option(Option<Box<Value>>),
    Record(HashMap<String, Value>),
    Varargs(Vec<Value>),
    IR(IR),
}

impl From<Value> for Type {
    fn from(v: Value) -> Self {
        match v {
            Value::String(_) => Type::String,
            Value::Integer(_) => Type::Integer,
            Value::Float(_) => Type::Float,
            Value::Boolean(_) => Type::Boolean,
            Value::List(l) => Type::List(Box::new(l[0].clone().into())),
            Value::Tuple(t) => Type::Tuple(t.iter().map(|v| v.clone().into()).collect()),
            Value::Option(o) => Type::Option(Box::new((*o.unwrap()).into())),
            Value::Record(r) => {
                let mut map = HashMap::new();
                for (k, v) in r.iter() {
                    map.insert(k.clone(), v.clone().into());
                }
                Type::Record(map)
            }
            Value::Varargs(v) => Type::Varargs(Box::new(v[0].clone().into())),
            Value::IR(_) => Type::IR,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueVec(pub Vec<Value>);

impl fmt::Display for ValueVec {
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

impl ValueVec {
    pub fn new() -> Self {
        ValueVec(Vec::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for ValueVec {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl fmt::Display for Value {
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

pub type SignatureTable = HashMap<(String, CommandType), Vec<Signature>>;

pub trait Plugin {
    fn call(
        &mut self,
        command_name: &str,
        command_type: &CommandType,
        args: &ValueVec,
    ) -> Result<Value, PluginError>;

    fn get_signatures(
        &self,
        command_name: &str,
        command_type: &CommandType,
    ) -> Option<Vec<Signature>>;

    fn match_signature(
        &self,
        command_name: &str,
        command_type: &CommandType,
        args: &ValueVec,
    ) -> Option<Signature>;

    fn ir_kind(&self) -> IRKind;
}

pub type Plugins = HashMap<String, Box<dyn Plugin>>;

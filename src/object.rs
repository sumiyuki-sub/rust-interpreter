use std::fmt;

use crate::ast::Statement;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    String(String),
    Boolean(bool),
    Null,
    Return(Box<Object>),
    Function {
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    Builtin(fn(Vec<Object>) -> Object),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(n) => write!(f, "{}", n),
            Object::String(s) => write!(f, "{}", s),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Null => write!(f, "null"),
            Object::Return(val) => write!(f, "{}", val),
            Object::Function { parameters, .. } => {
                write!(f, "fn({}) {{ ... }}", parameters.join(", "))
            }
            Object::Builtin(_) => write!(f, "<builtin function>"),
        }
    }
}

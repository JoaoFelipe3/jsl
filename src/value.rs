use std::fmt;

use super::parse;

#[derive(Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Function(parse::AST),
    List(Vec<Value>),
    Null,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(x), Value::Number(y)) => x == y,
            (Value::String(x), Value::String(y)) => x == y,
            (Value::List(x), Value::List(y)) => x == y,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}

impl Value {
    pub fn type_str(&self) -> &str {
        match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::List(_) => "list",
            Value::Function(_) => "function",
            Value::Null => "null",
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => {
                write!(f, "\"")?;
                for c in s.chars() {
                    match c {
                        '\r' => write!(f, "\\r"),
                        '\n' => write!(f, "\\n"),
                        '\t' => write!(f, "\\t"),
                        '\"' => write!(f, "\\\""),
                        '\\' => write!(f, "\\\\"),
                        _ if c.is_control() => write!(f, "\\x{:02x}", c as u8),
                        _ => write!(f, "{c}"),
                    }?;
                }
                write!(f, "\"")
            }
            Value::Function(_) => write!(f, "{{…}}"),
            Value::List(l) => {
                write!(f, "[ ")?;
                for e in l {
                    write!(f, "{e:?} ")?;
                }
                write!(f, "]")
            }
            Value::Null => write!(f, "∅"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Function(_) => write!(f, "{{…}}"),
            Value::List(l) => {
                write!(f, "[ ")?;
                for e in l {
                    write!(f, "{e:?} ")?;
                }
                write!(f, "]")
            }
            Value::Null => write!(f, "∅"),
        }
    }
}

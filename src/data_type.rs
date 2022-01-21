use std::{intrinsics::transmute};

use crate::map::Map;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Data {
    Unum(u64),
    Inum(i64),
    Char(char),
    List(Vec<Data>),
    Map(Map<Data, Data>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    U64,
    I64,
    F64,
    Char,
    List,
    Map,
    Compound(Map<String, Type>),
    Typedef(String, Box<Type>),
}

pub struct DataType {
    pub data: Data,
    pub typ: Type,
}

impl DataType {
    pub fn print(&self) -> String {
        match self.data {
            Data::Unum(n) => match &self.typ {
                Type::U64 => n.to_string(),
                Type::F64 => unsafe { transmute::<u64, f64>(n).to_string() },
                _ => "Mismatched types".to_string(),
            },
            Data::Inum(n) => n.to_string(),
            Data::Char(c) => {
                let inside = match c {
                    '\n' => "\\n".to_string(),
                    '\r' => "\\r".to_string(),
                    '\t' => "\\t".to_string(),
                    '\\' => "\\\\".to_string(),
                    '\'' => "\\'".to_string(),
                    _ => c.to_string(),
                };
                format!("'{}'", inside)
            },
            _ => "Err Unimplmented".to_string(),
        }
    }
}

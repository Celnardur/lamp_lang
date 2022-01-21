use std::intrinsics::transmute;

use crate::data_type::{Data, DataType, Type};
use crate::map::Map;
use crate::parse;
use crate::token::tokenize_from_str;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Code {
    Integer(i128),
    Float(u64),
    Character(char),
    StringLiteral(String),
    Identifier(String),
    List(Vec<Code>),
    Map(Map<Code, Code>),
}

impl Code {
    pub fn from_str(code: &str) -> Result<Vec<Code>, String> {
        parse::parse(&tokenize_from_str(code)?)
    }

    pub fn eval(&self) -> Result<DataType, String> {
        let (data, typ) = match self {
            Code::Integer(num) => {
                if *num > (i64::MAX as i128) && *num <= (u64::MAX as i128) {
                    (Data::Unum(*num as u64), Type::U64)
                } else if *num >= (i64::MIN as i128) {
                    (Data::Inum(*num as i64), Type::I64)
                } else {
                    return Err("Number out of bounds".to_string());
                }
            }
            Code::Float(num) => (Data::Unum(*num), Type::F64),
            Code::Character(c) => (Data::Char(*c), Type::Char),
            Code::StringLiteral(s) => (
                Data::List(s.bytes().map(|c| Data::Unum(c as u64)).collect()),
                Type::List,
            ),
            _ => return Err("Unimplmented".to_string()),
        };
        Ok(DataType { data, typ })
    }
}

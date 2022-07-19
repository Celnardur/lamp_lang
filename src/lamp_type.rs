use std::vec;

use crate::code::Code;
use crate::datum::ToDatum;
use crate::map::*;
use crate::utils::ts;

use LampType::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LampType {
    U8,
    U64,
    I64,
    F64,
    Char,

    List(Vec<LampType>),
    Vector(Box<LampType>),
    UnTypedList,

    Maping(Map<Code, (LampType, LampType)>),
    Dict(Box<LampType>, Box<LampType>),
    UnTypedMap,

    Struct(Map<String, LampType>),
    Enum(Map<String, Option<LampType>>),

    Code,

    Type,
}

impl LampType {
}

impl ToDatum for LampType {
    fn to_lamp_type() -> LampType {
        LampType::Type
    }

    fn to_code(&self) -> Code {
        Code::Integer(0)
    }
}
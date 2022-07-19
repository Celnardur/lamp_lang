use core::panic;
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
    DynList,

    // called mapping here so it doesn't conflict with map class
    Maping(Map<Code, (LampType, LampType)>),
    Dict(Box<LampType>, Box<LampType>),
    DynMap,

    Struct(Map<String, LampType>),
    Enum(Map<String, Option<LampType>>),

    Code,

    Type,
}

impl LampType {
}

// needed for the ToDatum implemtation for LampType
impl ToDatum for (LampType, LampType) {
    fn to_lamp_type() -> LampType {
        LampType::Vector(Box::new(LampType::Type))
    }

    fn to_code(&self) -> Code {
        Code::List(vec![self.0.to_code(), self.1.to_code()])
    }
}

impl ToDatum for LampType {
    fn to_lamp_type() -> LampType {
        LampType::Type
    }

    fn to_code(&self) -> Code {
        match self {
            U8 => Code::List(vec![Code::Identifier(ts("u8"))]),
            U64 => Code::List(vec![Code::Identifier(ts("u64"))]),
            I64 => Code::List(vec![Code::Identifier(ts("i64"))]),
            F64 => Code::List(vec![Code::Identifier(ts("f64"))]),
            Char => Code::List(vec![Code::Identifier(ts("char"))]),
            List(v) => Code::List(vec![Code::Identifier(ts("List")), v.to_code()]),
            Vector(t) => Code::List(vec![Code::Identifier(ts("Vec")), t.to_code()]),
            DynList => Code::List(vec![Code::Identifier(ts("DynList"))]),
            Maping(m) => Code::List(vec![Code::Identifier(ts("Map")), m.to_code()]),
            Dict(k, v) => Code::List(vec![Code::Identifier(ts("Dict")), Code::List(vec![k.to_code(), v.to_code()])]),
            DynMap => Code::List(vec![Code::Identifier(ts("DynMap"))]),
            Struct(m) => Code::List(vec![Code::Identifier(ts("Struct")), m.to_code()]),
            Enum(m) => Code::List(vec![Code::Identifier(ts("Struct")), m.to_code()]),
            Code => Code::List(vec![Code::Identifier(ts("Code"))]),
            Type => Code::List(vec![Code::Identifier(ts("Type"))]),
        }
    }
}
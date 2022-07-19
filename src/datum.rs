use std::vec;
use std::hash::Hash;

use crate::code::Code;
use crate::lamp_type::LampType;
use crate::map::*;
use crate::utils::ts;

#[derive(PartialEq, Eq)]
pub struct Datum {
    pub typ: LampType,
    pub data: Code,
}

pub trait ToDatum {
    fn to_lamp_type() -> LampType;
    fn to_code(&self) -> Code;
    fn to_datum(&self) -> Datum {
        Datum {
            typ: Self::to_lamp_type(),
            data: self.to_code(),
        }
    }
}

impl ToDatum for u8 {
    fn to_lamp_type() -> LampType {
        LampType::U8
    }

    fn to_code(&self) -> Code {
        Code::Integer((*self).into())
    }
}

impl ToDatum for u64 {
    fn to_lamp_type() -> LampType {
        LampType::U64
    }

    fn to_code(&self) -> Code {
        Code::Integer((*self).into())
    }
}

impl ToDatum for i64 {
    fn to_lamp_type() -> LampType {
        LampType::I64
    }

    fn to_code(&self) -> Code {
        Code::Integer((*self).into())
    }
}

impl ToDatum for f64 {
    fn to_lamp_type() -> LampType {
        LampType::F64
    }

    fn to_code(&self) -> Code {
        Code::Float(self.to_bits())
    }
}

impl ToDatum for char {
    fn to_lamp_type() -> LampType {
        LampType::Char
    }

    fn to_code(&self) -> Code {
        Code::Character(*self)
    }
}

impl<T: ToDatum> ToDatum for Vec<T> {
    fn to_lamp_type() -> LampType {
        LampType::Vector(Box::new(T::to_lamp_type()))
    }
    
    fn to_code(&self) -> Code {
        Code::List(self.iter().map(|t| t.to_code()).collect())
    }
}

impl ToDatum for String {
    fn to_lamp_type() -> LampType {
        LampType::Struct(map![
            {ts("data"), LampType::Vector(Box::new(LampType::U8))}
        ])
    }

    fn to_code(&self) -> Code {
        Code::Map(map![
            {Code::Identifier(ts("data")), Code::List(self.as_bytes().iter().map(|c| c.to_code()).collect())}
        ])
    }
}

impl<T: ToDatum> ToDatum for Option<T> {
    fn to_lamp_type() -> LampType {
        LampType::Enum(map![
            {ts("Some"), Some(T::to_lamp_type())},
            {ts("None"), None},
        ])
    }

    fn to_code(&self) -> Code {
        match self {
            Some(s) => Code::List(vec![Code::Identifier(ts("Some")), s.to_code()]),
            None => Code::List(vec![Code::Identifier(ts("None"))]),
        }
    }
}

impl<K: Eq + Hash + Ord + ToDatum, V: PartialEq + Hash + Ord + ToDatum> ToDatum for Map<K, V> {
    fn to_lamp_type() -> LampType {
        LampType::Dict(Box::new(K::to_lamp_type()), Box::new(V::to_lamp_type()))
    }

    fn to_code(&self) -> Code {
        let mut code = Map::new();
        for (key, val) in self.iter() {
            code.insert(key.to_code(), val.to_code());
        }
        Code::Map(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_datum_f64() {
        assert_eq!(f64::to_lamp_type(), LampType::F64);
        assert_eq!(3.14.to_code(), Code::Float(3.14_f64.to_bits()))
    }
}
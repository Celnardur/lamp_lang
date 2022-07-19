use crate::datum::ToDatum;
use crate::map::Map;
use crate::parse;
use crate::token::tokenize_from_str;
use crate::lamp_type::LampType;
use Code::*;

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

    pub fn eval(&self) -> Result<Code, String> {
        match self {
            &Integer(num) => Ok(Integer(num)),
            &Float(num) => Ok(Float(num)),
            &Character(c) => Ok(Character(c)),
            StringLiteral(s) => Ok(StringLiteral(s.clone())),
            Identifier(i) => Ok(Identifier(i.clone())),
            List(fun) => {
                let mut args = fun.iter();
                match args.next().map(|c| c.eval()) {
                    Some(Err(err)) => Err(err),
                    None => Ok(List(Vec::new())),
                    _ => Err("unimplmented".to_string()),
                }
            }
            _ => Err("unimplmented".to_string()),
        }
    }

    pub fn from_float(num: f64) -> Code {
        Code::Float(num.to_bits())
    }
}

impl ToDatum for Code {
    fn to_lamp_type() -> LampType {
        LampType::Code
    }

    fn to_code(&self) -> Code {
        self.clone()
    }
}
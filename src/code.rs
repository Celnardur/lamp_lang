use crate::map::Map;
use crate::parse;
use crate::token::tokenize_from_str;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Code {
    Integer(i128),
    Float(String),
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
}

use std::vec;

use crate::map::*;
use crate::code::Code;
use crate::utils::ts;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LampType {
    U8,
    U64,
    I64,
    F64,
    Char,

    List(Vec<LampType>),
    Vec(Box<LampType>),
    UnTypedList,

    Map(Map<Code,LampType>, Map<Code,LampType>),
    Dict(Box<LampType>, Box<LampType>),
    UnTypedMap,

    Struct(Map<String,LampType>),
    Enum(Map<String, Option<LampType>>),

    Code,

    Type,
}


// rust mockup of the function type
struct Arg {
    name: String,
    typ: Option<LampType>,
    default: Option<Code>,
}

// don't want people to add own runnables or use builtin runnables
// don't want people to make their own Function types
enum Runable {
    BuiltIn(String),
    Code(Code),
}

enum FnReturn {
    None,
    Code,
    Value(LampType),
}

struct Function {
    args: Vec<Arg>,
    eval_args: bool,
    runable: Runable,
    returns: FnReturn,
}

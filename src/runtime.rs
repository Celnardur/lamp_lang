use std::vec;

use crate::{code::Code, lamp_type::LampType, utils::ts};
use crate::datum::Datum;


pub struct Runtime {
    variables: Vec<(String, Datum)>,
}

impl Runtime {
    pub fn new() -> Runtime {
        let mut runtime = Runtime {
            variables: Vec::new(),
        };

        runtime.add_variable("pi", LampType::F64, Code::from_float(std::f64::consts::PI));
        runtime
    }

    fn add_variable(&mut self, name: &str, typ: LampType, data: Code) {
        self.variables.push((ts(name), Datum { typ, data }));
    }

    fn add_function(&mut self, func: Function) {
    }
}

// rust representation of the function type
struct Arg {
    name: String,
    typ: LampType,
    default: Option<Code>,
}

// don't want people to add own runnables or use builtin runnables
// don't want people to make their own Function types
enum Runable {
    BuiltIn(String),
    Code(Code),
}

struct Function {
    args: Vec<Arg>,
    runable: Runable,
    returns: Option<LampType>,
}

// the calling stack frame will be an environment variable that can be used
// to implement looping functions like for and the like

impl Function {
    fn plus() -> Function {
        Function {
            args: vec![Arg{
                name: ts("lhs"),
                typ: LampType::F64,
                default: None,
            },
            Arg{
                name: ts("rhs"),
                typ: LampType::F64,
                default: None,

            }],
            runable: Runable::BuiltIn(ts("plus")),
            returns: Some(LampType::F64),
        }
    }
}

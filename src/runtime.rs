use crate::{code::Code, lamp_type::LampType, utils::ts};

pub struct Datum {
    typ: LampType,
    data: Code,
}

pub struct Runtime {
    variables: Vec<(String, Datum)>,
}


impl Runtime {
    pub fn new() -> Runtime {
        let runtime = Runtime {
            variables: Vec::new(),
        };
        runtime
    }

    fn add_variable(&mut self, name: &str, typ: LampType, data: Code) {
        self.variables.push((
            ts(name),
            Datum {
                typ,
                data,
            }
        ));
    }
}
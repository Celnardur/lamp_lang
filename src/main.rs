use std::io::{self, Write};

use lamp_lang::code::Code;
use lamp_lang::parse;
use lamp_lang::token;

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        print!("> ");
        stdout.flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        let code = parse::parse(&token::tokenize_from_str(&input.trim()).unwrap()).unwrap();
        if code.get(0) == Some(&Code::Identifier("exit".to_string())) {
            return;
        }
        for expr in code {
            println!("{}", expr.eval().unwrap().print());
        }
        input.clear();
    }
}

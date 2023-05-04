#![warn(clippy::pedantic)]

use std::{env, fs, io};

use interpreter::Interpreter;

mod interpreter;
mod parser;

fn main() {
    let code = fs::read_to_string(env::args().nth(1).expect("missing filepath")).unwrap();
    let code = parser::parse_code(&code);
    Interpreter::new(io::stdin(), io::stdout()).interpret(&code);
}

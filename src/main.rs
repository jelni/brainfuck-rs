#![warn(clippy::pedantic)]

use std::{env, fs, io};

use brainfuck_rs::interpreter::{InterpretError, Interpreter};
use brainfuck_rs::parser::{self, ParseError};

#[derive(Debug)]
enum Error {
    Parse(ParseError),
    Interpret(InterpretError),
}

fn main() -> Result<(), Error> {
    let code = fs::read_to_string(env::args().nth(1).expect("missing filepath")).unwrap();
    let code = parser::parse_code(&code).map_err(Error::Parse)?;
    Interpreter::new(Box::new(io::stdin()), Box::new(io::stdout()))
        .interpret(&code)
        .map_err(Error::Interpret)?;

    Ok(())
}

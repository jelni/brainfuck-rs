#![warn(clippy::pedantic)]

use std::io::Write;
use std::{env, fs, io};

use brainfuck_rs::interpreter::{InterpretError, Interpreter};
use brainfuck_rs::parser::{self, ParseError};

#[derive(Debug)]
enum Error {
    Parse(ParseError),
    Interpret(InterpretError),
    Io(io::Error),
}

fn main() -> Result<(), Error> {
    match env::args().nth(1) {
        Some(path) => {
            let code = fs::read_to_string(path).map_err(Error::Io)?;
            let code = parser::parse_code(&code).map_err(Error::Parse)?;
            Interpreter::new(Box::new(io::stdin()), Box::new(io::stdout()))
                .interpret(&code)
                .map_err(Error::Interpret)?;
        }
        None => repl(),
    }

    Ok(())
}

fn repl() {
    println!("Welcome to REPL! Type \"reset\" to reset state or \"exit\" to exit.");

    let mut interpreter = Interpreter::new(Box::new(io::stdin()), Box::new(io::stdout()));

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut code = String::new();
        io::stdin().read_line(&mut code).unwrap();

        match code.trim().to_ascii_lowercase().as_str() {
            "exit" => return,
            "reset" => {
                interpreter.reset();
                println!("State reset.");
            }
            _ => (),
        }

        match parser::parse_code(&code) {
            Ok(code) => {
                if let Err(err) = interpreter.interpret(&code) {
                    eprintln!("{err:?}");
                    continue;
                }
            }
            Err(err) => {
                eprintln!("{err:?}");
                continue;
            }
        };
    }
}

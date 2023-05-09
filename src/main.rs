#![warn(clippy::pedantic)]

use std::io::Write;
use std::{env, fs, io};

use brainfuck_rs::interpreter::Interpreter;
use brainfuck_rs::parser;

fn main() {
    match env::args().nth(1) {
        Some(path) => {
            let code = match fs::read_to_string(path) {
                Ok(code) => code,
                Err(err) => {
                    eprintln!("failed to read file: {err}");
                    return;
                }
            };

            let code = match parser::parse_code(&code) {
                Ok(code) => code,
                Err(err) => {
                    eprint!("parser error: {err}");
                    return;
                }
            };

            let mut interpreter = Interpreter::new(Box::new(io::stdin()), Box::new(io::stdout()));

            if let Err(err) = interpreter.interpret(&code) {
                eprintln!("interpreter error: {err}");
                return;
            }

            eprintln!("{}", interpreter.stats());
        }
        None => repl(),
    }
}

fn repl() {
    eprintln!("Welcome to REPL! Available commands: \"stats\", \"reset\", \"exit\".");

    let mut interpreter = Interpreter::new(Box::new(io::stdin()), Box::new(io::stdout()));

    loop {
        eprint!("> ");
        io::stderr().flush().unwrap();

        let mut code = String::new();
        io::stdin().read_line(&mut code).unwrap();

        match code.trim().to_ascii_lowercase().as_str() {
            "exit" => return,
            "reset" => {
                interpreter.reset();
                eprintln!("state reset");
            }
            "stats" => eprintln!("{}", interpreter.stats()),
            _ => (),
        }

        let code = match parser::parse_code(&code) {
            Ok(code) => code,
            Err(err) => {
                eprintln!("{err}");
                continue;
            }
        };

        if let Err(err) = interpreter.interpret(&code) {
            eprintln!("{err}");
            continue;
        }
    }
}

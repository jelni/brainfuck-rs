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

            if let Err(err) =
                Interpreter::new(Box::new(io::stdin()), Box::new(io::stdout())).interpret(&code)
            {
                eprintln!("interpreter error: {err}");
            }
        }
        None => repl(),
    }
}

fn repl() {
    eprintln!("Welcome to REPL! Type \"reset\" to reset state or \"exit\" to exit.");

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

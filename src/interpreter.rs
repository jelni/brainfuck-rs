use std::io::{self, Read, Write};

use crate::parser::Token;

pub struct Interpreter {
    data: Vec<u8>,
    data_pointer: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            data: vec![0],
            data_pointer: 0,
        }
    }

    pub fn interpret(&mut self, code: &[Token]) {
        for token in code {
            self.evaluate(token);
        }
    }

    fn evaluate(&mut self, instruction: &Token) {
        match instruction {
            Token::IncrementDataPointer => {
                self.data_pointer += 1;
                if self.data_pointer >= self.data.len() {
                    self.data.push(0);
                }
            }
            Token::DecrementDataPointer => self.data_pointer -= 1,
            Token::IncrementByte => self.set_value(self.get_value().wrapping_add(1)),
            Token::DecrementByte => self.set_value(self.get_value().wrapping_sub(1)),
            Token::WriteByte => {
                print!("{}", char::from(self.get_value()));
                io::stdout().flush().unwrap();
            }
            Token::ReadByte => self.set_value(io::stdin().bytes().next().unwrap().unwrap()),
            Token::Loop(code) => {
                while self.get_value() != 0 {
                    self.interpret(code);
                }
            }
        }
    }

    fn get_value(&self) -> u8 {
        self.data[self.data_pointer]
    }

    fn set_value(&mut self, value: u8) {
        self.data[self.data_pointer] = value;
    }
}

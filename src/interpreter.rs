use std::io::{Read, Write};
use std::slice;

use crate::parser::Token;

pub struct Interpreter<'a> {
    data: Vec<u8>,
    data_pointer: usize,
    input: Box<dyn Read + 'a>,
    output: Box<dyn Write + 'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(input: impl Read + 'a, output: impl Write + 'a) -> Self {
        Self {
            data: vec![0],
            data_pointer: 0,
            input: Box::new(input),
            output: Box::new(output),
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
                self.output.write_all(&[self.get_value()]).unwrap();
                self.output.flush().unwrap();
            }
            Token::ReadByte => {
                let mut byte = 0;
                self.input.read_exact(slice::from_mut(&mut byte)).unwrap();
                self.set_value(byte);
            }
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

#![warn(clippy::pedantic)]

use std::io::{self, Read, Write};
use std::{env, fs};

fn main() {
    let code = fs::read_to_string(env::args().nth(1).expect("missing filepath")).unwrap();
    Interpreter::new(code).interpret();
}

struct Interpreter {
    code: String,
    instruction_pointer: usize,
    data: Vec<u8>,
    data_pointer: usize,
}

impl Interpreter {
    pub fn new(code: String) -> Self {
        Self {
            code,
            instruction_pointer: 0,
            data: vec![0],
            data_pointer: 0,
        }
    }

    pub fn interpret(&mut self) {
        while let Some(char) = self.code.chars().nth(self.instruction_pointer) {
            self.evaluate(char);
            self.instruction_pointer += 1;
        }
    }

    fn evaluate(&mut self, instruction: char) {
        match instruction {
            '>' => {
                self.data_pointer += 1;
                if self.data_pointer >= self.data.len() {
                    self.data.push(0);
                }
            }
            '<' => self.data_pointer -= 1,
            '+' => self.set_value(self.get_value().wrapping_add(1)),
            '-' => self.set_value(self.get_value().wrapping_sub(1)),
            '.' => {
                print!("{}", char::from(self.get_value()));
                io::stdout().flush().unwrap();
            }
            ',' => self.set_value(io::stdin().bytes().next().unwrap().unwrap()),
            '[' => {
                if self.get_value() == 0 {
                    let mut depth = 1;
                    for instruction in self.code[self.instruction_pointer + 1..].bytes() {
                        self.instruction_pointer += 1;

                        if instruction == b'[' {
                            depth += 1;
                        } else if instruction == b']' {
                            depth -= 1;
                            if depth == 0 {
                                return;
                            }
                        }
                    }

                    panic!("missing loop end")
                }
            }
            ']' => {
                if self.get_value() != 0 {
                    let mut depth = 1;
                    for instruction in self.code[..self.instruction_pointer].bytes().rev() {
                        self.instruction_pointer -= 1;

                        if instruction == b'[' {
                            depth -= 1;
                            if depth == 0 {
                                return;
                            }
                        } else if instruction == b']' {
                            depth += 1;
                        }
                    }

                    panic!("missing loop start")
                }
            }
            _ => (),
        }
    }

    fn get_value(&self) -> u8 {
        self.data[self.data_pointer]
    }

    fn set_value(&mut self, value: u8) {
        self.data[self.data_pointer] = value;
    }
}

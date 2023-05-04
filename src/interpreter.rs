use std::io::{self, Read, Write};
use std::slice;

use crate::parser::Token;

pub struct Interpreter<'a> {
    data: Vec<u8>,
    data_pointer: usize,
    input: Box<dyn Read + 'a>,
    output: Box<dyn Write + 'a>,
}

#[derive(Debug)]
pub enum InterpretError {
    DataPointerOutsideMemory,
    WriteError(io::Error),
}

impl<'a> Interpreter<'a> {
    pub fn new(input: Box<impl Read + 'a>, output: Box<impl Write + 'a>) -> Self {
        Self {
            data: vec![0; 64],
            data_pointer: 0,
            input,
            output,
        }
    }

    pub fn interpret(&mut self, code: &[Token]) -> Result<(), InterpretError> {
        for token in code {
            self.evaluate(token)?;
        }

        Ok(())
    }

    fn evaluate(&mut self, instruction: &Token) -> Result<(), InterpretError> {
        match instruction {
            Token::IncrementDataPointer(i) => {
                self.data_pointer = self
                    .data_pointer
                    .checked_add(*i)
                    .ok_or(InterpretError::DataPointerOutsideMemory)?;

                while self.data_pointer >= self.data.len() {
                    self.data.push(0);
                }
            }
            Token::DecrementDataPointer(i) => {
                self.data_pointer = self
                    .data_pointer
                    .checked_sub(*i)
                    .ok_or(InterpretError::DataPointerOutsideMemory)?;
            }
            Token::IncrementByte(i) => self.set_value(self.get_value().wrapping_add(*i)),
            Token::DecrementByte(i) => self.set_value(self.get_value().wrapping_sub(*i)),
            Token::WriteByte => {
                self.output
                    .write_all(&[self.get_value()])
                    .map_err(InterpretError::WriteError)?;
                self.output.flush().map_err(InterpretError::WriteError)?;
            }
            Token::ReadByte => {
                let mut byte = 0;
                if self.input.read_exact(slice::from_mut(&mut byte)).is_ok() {
                    self.set_value(byte);
                }
            }
            Token::Loop(code) => {
                while self.get_value() != 0 {
                    self.interpret(code)?;
                }
            }
        };

        Ok(())
    }

    fn get_value(&self) -> u8 {
        self.data[self.data_pointer]
    }

    fn set_value(&mut self, value: u8) {
        self.data[self.data_pointer] = value;
    }
}

#[cfg(test)]
mod test {
    use std::io::{self, Cursor};

    use super::*;
    use crate::parser::parse_code;

    #[test]
    fn test_interpreter() {
        let mut output = Vec::new();
        Interpreter::new(Box::new(Cursor::new(b"1234")), Box::new(&mut output))
            .interpret(&[
                Token::IncrementByte(4),
                Token::Loop(vec![
                    Token::IncrementDataPointer(1),
                    Token::ReadByte,
                    Token::WriteByte,
                    Token::DecrementDataPointer(1),
                    Token::DecrementByte(1),
                ]),
            ])
            .unwrap();
        assert_eq!(output, b"1234");
    }

    #[test]
    fn test_hello_world() {
        let mut output = Vec::new();
        Interpreter::new(Box::new(io::empty()), Box::new(&mut output))
            .interpret(
                &parse_code(concat!(
                    "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]",
                    ">>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
                ))
                .unwrap(),
            )
            .unwrap();
        assert_eq!(output, b"Hello World!\n");
    }

    #[test]
    fn test_hello_world_negative() {
        let mut output = Vec::new();
        // example from https://esolangs.org/wiki/Brainfuck
        Interpreter::new(Box::new(io::empty()), Box::new(&mut output))
            .interpret(
                &parse_code(concat!(
                    ">++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<+++>]<<]>-----.>->",
                    "+++..+++.>-.<<+[>[+>+]>>]<--------------.>>.+++.------.--------.>+.>+."
                ))
                .unwrap(),
            )
            .unwrap();
        assert_eq!(output, b"Hello World!\n");
    }

    #[test]
    fn test_obscure_problems() {
        let mut output = Vec::new();
        // test from http://brainfuck.org/tests.b
        Interpreter::new(Box::new(Cursor::new("\n")), Box::new(&mut output))
            .interpret(
                &parse_code(concat!(
                    "[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]",
                    "\"A*$\";?@![#>>+<<]>[>>]<<<<[>++<[-]]>.>."
                ))
                .unwrap(),
            )
            .unwrap();
        assert_eq!(output, b"H\n");
    }

    #[test]
    fn test_errors() {
        let mut interpreter = Interpreter::new(Box::new(io::empty()), Box::new(Vec::new()));

        let err = interpreter
            .interpret(&[Token::DecrementDataPointer(1)])
            .unwrap_err();

        let InterpretError::DataPointerOutsideMemory = err else {
            panic!();
        };
    }
}

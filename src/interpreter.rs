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
            Token::ModifyDataPointer(i) => {
                let current: i32 = self.data_pointer.try_into().unwrap();
                self.data_pointer = current.wrapping_add(*i).try_into().unwrap();
                while self.data_pointer >= self.data.len() {
                    self.data.push(0);
                }
            }
            Token::ModifyByte(i) => {
                let current: i32 = self.get_value().into();
                self.set_value((current.wrapping_add(*i) % 256).try_into().unwrap());
            }
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

#[cfg(test)]
mod test {
    use std::io::{self, Cursor};

    use super::*;
    use crate::parser::parse_code;

    #[test]
    fn test_interpreter() {
        let mut output = Vec::new();
        Interpreter::new(Cursor::new(b"1234"), &mut output).interpret(&[
            Token::ModifyByte(4),
            Token::Loop(vec![
                Token::ModifyDataPointer(1),
                Token::ReadByte,
                Token::WriteByte,
                Token::ModifyDataPointer(-1),
                Token::ModifyByte(-1),
            ]),
        ]);
        assert_eq!(output, b"1234");
    }

    #[test]
    fn hello_world() {
        let mut output = Vec::new();
        Interpreter::new(io::empty(), &mut output).interpret(&parse_code(concat!(
            "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]",
            ">>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."
        )));
        assert_eq!(output, b"Hello World!\n");
    }
}

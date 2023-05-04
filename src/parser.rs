#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    ModifyDataPointer(i32),
    ModifyByte(i32),
    WriteByte,
    ReadByte,
    Loop(Vec<Token>),
}

pub fn parse_code(code: &str) -> Vec<Token> {
    let mut stack = vec![vec![]];

    for instruction in code.bytes() {
        match instruction {
            b'>' => {
                let last = stack.last_mut().unwrap();
                match last.last_mut() {
                    Some(Token::ModifyDataPointer(ref mut i)) => {
                        if *i == -1 {
                            last.pop();
                        } else {
                            *i += 1;
                        }
                    }
                    _ => last.push(Token::ModifyDataPointer(1)),
                }
            }
            b'<' => {
                let last = stack.last_mut().unwrap();
                match last.last_mut() {
                    Some(Token::ModifyDataPointer(ref mut i)) => {
                        if *i == 1 {
                            last.pop();
                        } else {
                            *i -= 1;
                        }
                    }
                    _ => last.push(Token::ModifyDataPointer(-1)),
                }
            }
            b'+' => {
                let last = stack.last_mut().unwrap();
                match last.last_mut() {
                    Some(Token::ModifyByte(ref mut i)) => {
                        if *i == -1 {
                            last.pop();
                        } else {
                            *i += 1;
                        }
                    }
                    _ => last.push(Token::ModifyByte(1)),
                }
            }
            b'-' => {
                let last = stack.last_mut().unwrap();
                match last.last_mut() {
                    Some(Token::ModifyByte(ref mut i)) => {
                        if *i == 1 {
                            last.pop();
                        } else {
                            *i -= 1;
                        }
                    }
                    _ => last.push(Token::ModifyByte(-1)),
                }
            }
            b'.' => stack.last_mut().unwrap().push(Token::WriteByte),
            b',' => stack.last_mut().unwrap().push(Token::ReadByte),
            b'[' => stack.push(Vec::new()),
            b']' => {
                let inner = stack.pop().unwrap();
                stack
                    .last_mut()
                    .expect("unmatched `]`")
                    .push(Token::Loop(inner));
            }
            _ => (),
        }
    }

    assert!(stack.len() == 1, "unmatched `[`");

    stack.pop().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let code = parse_code("> + < - . , [ + ]");
        assert_eq!(
            code,
            &[
                Token::ModifyDataPointer(1),
                Token::ModifyByte(1),
                Token::ModifyDataPointer(-1),
                Token::ModifyByte(-1),
                Token::WriteByte,
                Token::ReadByte,
                Token::Loop(vec![Token::ModifyByte(1)])
            ]
        )
    }

    #[test]
    fn test_optimizations() {
        let code = parse_code("++++--><>++--");
        assert_eq!(code, &[Token::ModifyByte(2), Token::ModifyDataPointer(1)]);
    }
}

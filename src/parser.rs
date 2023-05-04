#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    IncrementDataPointer(usize),
    DecrementDataPointer(usize),
    IncrementByte(u8),
    DecrementByte(u8),
    WriteByte,
    ReadByte,
    Loop(Vec<Token>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    TooBigDataPointerIncrement,
    TooBigDataPointerDecrement,
    UnmatchedSymbol(char),
}

pub fn parse_code(code: &str) -> Result<Vec<Token>, ParseError> {
    let mut stack = vec![vec![]];

    for instruction in code.bytes() {
        match instruction {
            b'>' => {
                let last = stack.last_mut().unwrap();
                match last.last_mut() {
                    Some(Token::IncrementDataPointer(ref mut i)) => {
                        *i = i
                            .checked_add(1)
                            .ok_or(ParseError::TooBigDataPointerIncrement)?;
                    }
                    Some(Token::DecrementDataPointer(ref mut i)) => {
                        if *i == 1 {
                            last.pop();
                        } else {
                            *i = i.checked_sub(1).unwrap();
                        }
                    }
                    _ => last.push(Token::IncrementDataPointer(1)),
                }
            }
            b'<' => {
                let last = stack.last_mut().unwrap();
                match last.last_mut() {
                    Some(Token::IncrementDataPointer(ref mut i)) => {
                        if *i == 1 {
                            last.pop();
                        } else {
                            *i = i.checked_sub(1).unwrap();
                        }
                    }
                    Some(Token::DecrementDataPointer(ref mut i)) => {
                        *i = i
                            .checked_add(1)
                            .ok_or(ParseError::TooBigDataPointerDecrement)?;
                    }
                    _ => last.push(Token::DecrementDataPointer(1)),
                }
            }
            b'+' => {
                let last = stack.last_mut().unwrap();
                match last.last_mut() {
                    Some(Token::IncrementByte(ref mut i)) => *i = i.wrapping_add(1),
                    Some(Token::DecrementByte(ref mut i)) => {
                        if *i == 1 {
                            last.pop();
                        } else {
                            *i = i.wrapping_sub(1);
                        }
                    }
                    _ => last.push(Token::IncrementByte(1)),
                }
            }
            b'-' => {
                let last = stack.last_mut().unwrap();
                match last.last_mut() {
                    Some(Token::IncrementByte(ref mut i)) => {
                        if *i == 1 {
                            last.pop();
                        } else {
                            *i = i.wrapping_sub(1);
                        }
                    }
                    Some(Token::DecrementByte(ref mut i)) => *i = i.wrapping_add(1),
                    _ => last.push(Token::DecrementByte(1)),
                }
            }
            b'.' => stack.last_mut().unwrap().push(Token::WriteByte),
            b',' => stack.last_mut().unwrap().push(Token::ReadByte),
            b'[' => stack.push(Vec::new()),
            b']' => {
                let inner = stack.pop().unwrap();
                stack
                    .last_mut()
                    .ok_or(ParseError::UnmatchedSymbol(']'))?
                    .push(Token::Loop(inner));
            }
            _ => (),
        }
    }

    if stack.len() != 1 {
        return Err(ParseError::UnmatchedSymbol('['));
    }

    Ok(stack.pop().unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let code = parse_code("> + < - . , [ + ]").unwrap();
        assert_eq!(
            code,
            &[
                Token::IncrementDataPointer(1),
                Token::IncrementByte(1),
                Token::DecrementDataPointer(1),
                Token::DecrementByte(1),
                Token::WriteByte,
                Token::ReadByte,
                Token::Loop(vec![Token::IncrementByte(1)])
            ]
        )
    }

    #[test]
    fn test_optimizations() {
        let code = parse_code(">><< ++-- <<>> --++").unwrap();
        assert_eq!(code, &[]);

        let code = parse_code(">>>< +++- <<<> ---+").unwrap();
        assert_eq!(
            code,
            &[
                Token::IncrementDataPointer(2),
                Token::IncrementByte(2),
                Token::DecrementDataPointer(2),
                Token::DecrementByte(2)
            ]
        );

        let code = parse_code("><<< +--- <>>> -+++").unwrap();
        assert_eq!(
            code,
            &[
                Token::DecrementDataPointer(2),
                Token::DecrementByte(2),
                Token::IncrementDataPointer(2),
                Token::IncrementByte(2)
            ]
        );
    }

    #[test]
    fn test_errors() {
        let err = parse_code("[+][").unwrap_err();
        assert_eq!(err, ParseError::UnmatchedSymbol('['));

        let err = parse_code("[+]]").unwrap_err();
        assert_eq!(err, ParseError::UnmatchedSymbol(']'));
    }
}

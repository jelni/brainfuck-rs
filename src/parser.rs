#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    IncrementDataPointer,
    DecrementDataPointer,
    IncrementByte,
    DecrementByte,
    WriteByte,
    ReadByte,
    Loop(Vec<Token>),
}

pub fn parse_code(code: &str) -> Vec<Token> {
    let mut stack = vec![vec![]];

    for instruction in code.bytes() {
        match instruction {
            b'>' => stack.last_mut().unwrap().push(Token::IncrementDataPointer),
            b'<' => stack.last_mut().unwrap().push(Token::DecrementDataPointer),
            b'+' => stack.last_mut().unwrap().push(Token::IncrementByte),
            b'-' => stack.last_mut().unwrap().push(Token::DecrementByte),
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
        let code = parse_code("> < + - . , [ + ]");
        assert_eq!(
            code,
            vec![
                Token::IncrementDataPointer,
                Token::DecrementDataPointer,
                Token::IncrementByte,
                Token::DecrementByte,
                Token::WriteByte,
                Token::ReadByte,
                Token::Loop(vec![Token::IncrementByte])
            ]
        )
    }
}

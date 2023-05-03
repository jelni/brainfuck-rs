#[derive(Debug)]
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
    let mut tokens = vec![vec![]];

    for instruction in code.bytes() {
        match instruction {
            b'>' => tokens.last_mut().unwrap().push(Token::IncrementDataPointer),
            b'<' => tokens.last_mut().unwrap().push(Token::DecrementDataPointer),
            b'+' => tokens.last_mut().unwrap().push(Token::IncrementByte),
            b'-' => tokens.last_mut().unwrap().push(Token::DecrementByte),
            b'.' => tokens.last_mut().unwrap().push(Token::WriteByte),
            b',' => tokens.last_mut().unwrap().push(Token::ReadByte),
            b'[' => tokens.push(Vec::new()),
            b']' => {
                let inner = tokens.pop().unwrap();
                tokens
                    .last_mut()
                    .expect("unmatched `]`")
                    .push(Token::Loop(inner));
            }
            _ => (),
        }
    }

    assert!(tokens.len() == 1, "unmatched `[`");

    tokens.pop().unwrap()
}

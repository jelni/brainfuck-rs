use std::{fmt, io};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    DataPointerIncrementOverflow,
    DataPointerDecrementOverflow,
    UnmatchedSymbol(char),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::DataPointerIncrementOverflow => {
                write!(f, "data pointer increment overflow")
            }
            ParseError::DataPointerDecrementOverflow => {
                write!(f, "data pointer decrement overflow")
            }
            ParseError::UnmatchedSymbol(symbol) => {
                write!(f, "unmatched `{symbol}`")
            }
        }
    }
}

#[derive(Debug)]
pub enum InterpretError {
    DataPointerOutsideMemory,
    EmptyLoop,
    WriteError(io::Error),
}

impl fmt::Display for InterpretError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpretError::DataPointerOutsideMemory => {
                write!(f, "data pointer outside available memory")
            }
            InterpretError::EmptyLoop => {
                write!(f, "interpreter stuck in an empty loop")
            }
            InterpretError::WriteError(err) => {
                write!(f, "output error: {err}")
            }
        }
    }
}

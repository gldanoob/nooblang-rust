use core::fmt;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum Errors {
    SyntaxError(String, (usize, usize)),
    IOError,
}

impl Error for Errors {}
impl Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SyntaxError(e, (line, col)) => {
                writeln!(f, "Goofy syntax error")?;
                writeln!(f, "{}", e)?;
                write!(f, "line: {}, col: {}", line, col)
            }

            Self::IOError => {
                write!(f, "Failed to read file rip")
            }
        }
    }
}

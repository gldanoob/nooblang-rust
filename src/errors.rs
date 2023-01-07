use core::fmt;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum Errors {
    SyntaxError(String, (usize, usize), String),
    IOError,
}

impl Error for Errors {}
impl Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SyntaxError(e, (line, col), context) => {
                writeln!(f, "--------- TYPO ---------")?;
                writeln!(f, "{}", e)?;
                writeln!(f, "IN LINE: {}, COL: {}", line, col)?;
                writeln!(f)?;

                // Cool stuff
                write!(f, "--> ")?;
                write!(f, "{}", context)?;

                writeln!(f, "\n{}^-- LOOK", " ".repeat(col + 3))?;
                write!(f, "------- YOU SUCK -------")
            }

            Self::IOError => {
                write!(f, "Failed to read file rip")
            }
        }
    }
}

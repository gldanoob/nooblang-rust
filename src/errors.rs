use core::fmt;
use std::{error::Error, fmt::Display};

use crate::lexer::Pos;

#[derive(Debug)]
pub enum Errors {
    ArgumentError,
    SyntaxError(String, Pos, String),
    IOError,
    RuntimeError(String, Pos, String),
    Interrupt(usize, usize),
    EndProg,
}

impl Error for Errors {}
impl Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ArgumentError => {
                writeln!(f, "NEED SOURCE FILE AS ARGUMENT")
            },
            Self::SyntaxError(e, Pos(line, col), context) => {
                writeln!(f, "--------- TYPO ---------")?;
                writeln!(f, "{}", e)?;
                writeln!(f, "IN LINE: {}, COL: {}", line, col)?;
                writeln!(f)?;

                // Cool stuff
                write!(f, "--> ")?;
                write!(f, "{}", context)?;

                write!(f, "\n{}^-- SEE", " ".repeat(col + 3))
            }

            Self::IOError => {
                write!(f, "Failed to read file :^) sorry")
            }
            Self::RuntimeError(e, Pos(line, col), context) => {
                writeln!(f, "--------- ERROR ---------")?;
                writeln!(f, "{}", e)?;
                writeln!(f, "IN LINE: {}, COL: {}", line, col)?;
                writeln!(f)?;

                // Cool stuff
                write!(f, "--> ")?;
                write!(f, "{}", context)?;

                write!(f, "\n{}^-- SEE", " ".repeat(col + 3))
            },
            Self::Interrupt(from, to) => {
                writeln!(f, "Control flow changed to lines: {}-{}", from, to)
            }
            Self::EndProg => {
                write!(f, "Program ended")
            },
        }
    }
}

use crate::lexer::Pos;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Literals
    Id(String),
    String(String),
    Number(u128),

    // Keywords
    Write,
    Read,
    Be,
    Run,
    If,
    To,
    Dot,
    Plus,
    Minus,
    Times,
    Over,
    ToThe,
    Mod,
    Is,
    Isnt,
    Below,
    Above,
    AtMost,
    AtLeast,
    And,
    Or,
    Not,
    Num,
    Text,
    Choice,
    Yes,
    No,
    Open,
    Close,
    Lf,
    Eof,
    Note,
    Neg,
    End,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub location: Pos,
}

impl TokenType {
    pub fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            "write" => Self::Write,
            "read" => Self::Read,
            "be" => Self::Be,
            "run" => Self::Run,
            "if" => Self::If,
            "to" => Self::To,
            "dot" => Self::Dot,
            "plus" => Self::Plus,
            "minus" => Self::Minus,
            "times" => Self::Times,
            "over" => Self::Over,
            "mod" => Self::Mod,
            "tothe" => Self::ToThe,
            "is" => Self::Is,
            "isnt" => Self::Isnt,
            "below" => Self::Below,
            "above" => Self::Above,
            "atmost" => Self::AtMost,
            "atleast" => Self::AtLeast,
            "and" => Self::And,
            "or" => Self::Or,
            "not" => Self::Not,
            "num" => Self::Num,
            "text" => Self::Text,
            "choice" => Self::Choice,
            "yes" => Self::Yes,
            "no" => Self::No,
            "neg" => Self::Neg,
            "close" => Self::Close,
            "open" => Self::Open,
            "note" => Self::Note,
            "end" => Self::End,
            _ => return None,
        })
    }
}

impl Token {
    pub fn new(token_type: TokenType, location: Pos) -> Self {
        Self {
            token_type,
            location,
        }
    }

    pub fn from_str(lexeme: &str, location: Pos) -> Option<Self> {
        match TokenType::from_str(&lexeme) {
            Some(token_type) => Some(Self {
                token_type,
                location,
            }),
            None => None,
        }
    }
}

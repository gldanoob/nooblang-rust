#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Literals
    Id(String),
    String(String),
    Number(u128),

    // Keywords
    Oof,
    Noob,
    Be,
    Eef,
    Elz,
    For,
    In,
    To,
    Wow,
    Lol,
    Dot,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Inc,
    Dec,
    Eq,
    Nq,
    Las,
    Mor,
    Laq,
    Moq,
    Not,
    Num,
    Str,
    Tru,
    Foz,
    Lbr,
    Rbr,
    Lf,
    Eof,
    Com,
    Neg,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub location: (usize, usize),
}

impl TokenType {
    pub fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            "oof" => Self::Oof,
            "noob" => Self::Noob,
            "be" => Self::Be,
            "eef" => Self::Eef,
            "elz" => Self::Elz,
            "for" => Self::For,
            "in" => Self::In,
            "to" => Self::To,
            "wow" => Self::Wow,
            "lol" => Self::Lol,
            "dot" => Self::Dot,
            "add" => Self::Add,
            "sub" => Self::Sub,
            "mul" => Self::Mul,
            "div" => Self::Div,
            "pow" => Self::Pow,
            "inc" => Self::Inc,
            "dec" => Self::Dec,
            "eq" => Self::Eq,
            "nq" => Self::Nq,
            "las" => Self::Las,
            "mor" => Self::Mor,
            "laq" => Self::Laq,
            "moq" => Self::Moq,
            "not" => Self::Not,
            "num" => Self::Num,
            "str" => Self::Str,
            "tru" => Self::Tru,
            "foz" => Self::Foz,
            "neg" => Self::Neg,
            "rbr" => Self::Rbr,
            "lbr" => Self::Lbr,
            "com" => Self::Com,
            _ => return None,
        })
    }
}

impl Token {
    pub fn new(token_type: TokenType, location: (usize, usize)) -> Self {
        Self {
            token_type,
            location,
        }
    }

    pub fn from_str(lexeme: &str, location: (usize, usize)) -> Option<Self> {
        match TokenType::from_str(&lexeme) {
            Some(token_type) => Some(Self {
                token_type,
                location,
            }),
            None => None,
        }
    }
}

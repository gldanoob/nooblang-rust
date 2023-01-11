use crate::lexer::Pos;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Be,
    Is,
    Isnt,
    Below,
    Above,
    AtMost,
    AtLeast,
    Plus,
    Minus,
    Times,
    Over,
    Mod,
    ToThe,
    Neg,
    Read,
    And,
    Or,
    Not,
    Write,
    Text,
    Num,
    Choice,
}

// Abstract types
#[derive(Debug)]
pub enum Stmt {
    Expr(Box<Expr>),
}

#[derive(Debug)]
pub enum Expr {
    Unary(Operator, Box<Expr>, Pos),
    Binary(Operator, Box<Expr>, Box<Expr>, Pos),
    Nullary(Operator, Pos),
    Literal(Literal, Pos),
    Id(String, Pos),
}

#[derive(Debug)]
pub enum Literal {
    Integer(u128),
    Decimal(f64),
    Text(String),
    Choice(bool),
}

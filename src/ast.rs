use crate::lexer::Pos;

#[derive(Debug, PartialEq)]
pub enum Operator {
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
    Text,
    Num,
    Choice,
}

// Abstract types
#[derive(Debug)]
pub enum Stmt {
    // Id, Expr
    Asgn(Box<Expr>, Box<Expr>, usize),

    // From, To
    RunFrom(Box<Expr>, Box<Expr>, usize),
    RunAt(Box<Expr>, usize),
    Write(Box<Expr>, usize),

    // Statement, condition
    Switch(Box<Stmt>, Box<Expr>, usize),

    Expr(Box<Expr>),

    // Blankline
    Blank,

    End,
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

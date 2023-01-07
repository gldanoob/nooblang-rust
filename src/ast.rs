#[derive(PartialEq)]
pub enum Operator {
    Id,
    Inc,
    Dec,
    Be,
    Eq,
    Nq,
    Las,
    Mor,
    Laq,
    Moq,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Neg,
    Dot,
    Noob,
    Not,
    Oof,
    Str,
    Num,
}


// Abstract types

#[derive(PartialEq)]
pub enum Stmt {
    Expr(Box<Expr>),
}


#[derive(PartialEq)]
pub enum Expr {
    Unary(Operator, Box<Expr>),
    Binary(Operator, Box<Expr>, Box<Expr>),
    Literal(Literal),
    Id(String),
}

#[derive(PartialEq)]
pub enum Literal {
    Number(u128),
    String(String),
    Boolean(bool),
}

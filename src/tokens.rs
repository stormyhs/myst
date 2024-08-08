#[derive(Debug, Clone)]
pub enum Operator {
    Add, Subtract, Multiply, Divide,
    Equality, Lesser, Greater,
    Declare, Assign
}

#[derive(Debug, Clone)]
pub enum Expr {
    Let,
    If(Box<Expr>, Box<Vec<Expr>>),
    Else(Box<Expr>, Box<Vec<Expr>>),
    While(Box<Expr>, Box<Vec<Expr>>),
    Number(i64),
    String(String),
    Identifier(String),
    Function(String),
    Call(String, Box<Vec<Expr>>),
    BinOp(Operator, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Token {
    LParen, RParen,
    LCurly, RCurly,
    LArrow, RArrow,
    DoubleQuote,
    Plus, Minus, Star, Slash,
    Equal, Equality,
    Semicolon,
    Identifier(String),
    Declaration(String, Box<Expr>),
    Assignment(String, Box<Expr>),
    String(String), Number(i64),

    // Keywords
    Let,
    If(Box<Expr>, Vec<Expr>),
    Else(Box<Expr>, Vec<Expr>),
    While(Box<Expr>, Vec<Expr>),
}


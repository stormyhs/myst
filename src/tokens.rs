#[derive(Debug, Clone)]
pub enum Operator {
    Add, Subtract, Multiply, Divide,
    Declare, Assign
}

#[derive(Debug, Clone)]
pub enum Expr {
    Let,
    Number(i16),
    String(String),
    Identifier(String),
    Function(String),
    Call(String, Box<Vec<Expr>>),
    BinOp(Operator, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Token {
    LParen, RParen,
    DoubleQuote,
    Plus, Minus, Star, Slash,
    Equal,
    Semicolon,
    Let,
    Identifier(String),
    Declaration(String, Box<Expr>),
    Assignment(String, Box<Expr>),
    String(String), Number(i16)
}


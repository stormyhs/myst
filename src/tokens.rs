#[derive(Debug, Clone)]
pub enum Operator {
    Add, Subtract, Multiply, Divide,
    Declare, Assign
}

#[derive(Debug, Clone)]
pub enum Expr {
    Let,
    If(Box<Expr>, Box<Vec<Expr>>, Box<Vec<Expr>>),
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
    LCurly, RCurly,
    DoubleQuote,
    Plus, Minus, Star, Slash,
    Equal,
    Semicolon,
    Identifier(String),
    Declaration(String, Box<Expr>),
    Assignment(String, Box<Expr>),
    String(String), Number(i16),

    // Keywords
    Let,
    If(Box<Expr>, Vec<Expr>, Vec<Expr>),
}


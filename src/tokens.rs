
#[derive(Debug, Clone)]
pub enum Value {
    Number(i16),
    String(String),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i16),
    String(String),
    Identifier(String),
    BinOp(char, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Token {
    LParen, RParen,
    DoubleQuote,
    Plus, Minus, Star, Slash,
    Equal,
    Semicolon,
    Identifier(String),
    String(String), Number(i16)
}


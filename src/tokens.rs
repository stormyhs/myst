// This file is a lot more than just tokens.
// Surely, one day, I will reorganize this file. :clueless:

#[derive(Debug, Clone)]
pub enum Operator {
    Add, Subtract, Multiply, Divide,
    Equality, Lesser, Greater,
    Declare, Assign
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    String(String),
    BinOp(Operator, Box<Expr>, Box<Expr>),

    If(Box<Expr>, Box<Vec<Expr>>),
    Else(Box<Expr>, Box<Vec<Expr>>),
    While(Box<Expr>, Box<Vec<Expr>>),

    Identifier(String),
    Func(String, Box<Vec<Expr>>, Box<Vec<Expr>>),
    Call(String, Box<Vec<Expr>>),
}

#[derive(Debug, Clone)]
pub enum Token {
    LParen, RParen,
    LCurly, RCurly,
    LArrow, RArrow,
    Plus, Minus, Star, Slash,
    Equal, Equality,
    Semicolon,
    Comma,
    Identifier(String),
    Declaration(String, Box<Expr>),
    Assignment(String, Box<Expr>),
    String(String), Number(i64),

    // Keywords
    Let,
    If,
    Else,
    While,
    Func,
}


// This file is a lot more than just tokens.
// Surely, one day, I will reorganize this file. :clueless:

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add, Subtract, Multiply, Divide,
    Equality, Lesser, Greater,
    Declare, Assign
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(i64),
    String(String),
    Array(Box<Vec<Expr>>),

    BinOp(Operator, Box<Expr>, Box<Expr>),
    AccessProperty(Box<Expr>, String),

    If(Box<Expr>, Box<Vec<Expr>>),
    Else(Box<Expr>, Box<Vec<Expr>>),
    While(Box<Expr>, Box<Vec<Expr>>),

    // for item of array { ... }
    For(String, Box<Expr>),

    Identifier(String),

    Func(String, Box<Vec<Expr>>, Box<Vec<Expr>>),
    Call(Box<Expr>, Box<Vec<Expr>>),

    Import(String),
    Include(String),
}

#[derive(Debug, Clone)]
pub enum Token {
    LParen, RParen,
    LCurly, RCurly,
    LBracket, RBracket,
    LArrow, RArrow,
    Plus, Minus, Star, Slash,
    Equal, Equality,
    Semicolon,
    Comma, Dot,

    Identifier(String),
    String(String), Number(i64),
    Array(Box<Vec<Token>>),

    Declaration(String, Box<Expr>),
    Assignment(String, Box<Expr>),
    AccessProperty,

    // Keywords
    Let,
    If,
    Else,
    While,
    For,
    Func,
    Import,
    Include,
}


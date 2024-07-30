// TODO: This file is named `tokens`. But it contains more than just tokens.
// This entire structure needs to be redone. Notably, `Variable` probably shouldn't even exist.
// Or something. I don't know. I'm tired. I'll come back to this later.

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Token {
    Number(i16),
    Operator(Operator),
    Identifier(String),
    Variable(String, i16),
    Assign,
    Semicolon
}

#[derive(Debug)]
pub enum Expr {
    Number(i16),
    BinOp(char, Box<Expr>, Box<Expr>),
}

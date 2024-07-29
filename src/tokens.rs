
/*this is the token definitions*/

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
    Assign
}

#[derive(Debug)]
pub enum Expr {
    Number(i16),
    BinOp(char, Box<Expr>, Box<Expr>),
}

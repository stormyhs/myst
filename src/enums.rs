
#[derive(Debug, Clone)]
pub enum Expr {
    BinOp(Operator, Box<Expr>, Box<Expr>),

    Number(i64),
    String(String),
    Array(Box<Vec<Expr>>),
    Identifier(String),

    ArrayAccess(String, Box<Expr>),
    PropertyAccess(Box<Expr>, Box<Expr>),
    
    If(Box<Expr>, Box<Vec<Expr>>, Box<Vec<Expr>>),
    While(Box<Expr>, Box<Vec<Expr>>),
    For(String, Box<Expr>, Box<Vec<Expr>>),

    DecFunc(String, Box<Vec<String>>, Box<Vec<Expr>>),
    CallFunc(Box<Expr>, Box<Vec<Expr>>),

    DecClass(String, Box<Vec<Expr>>),
    InstantiateClass(String, Box<Vec<Expr>>),

    Import(String),
    Include(String),

    Return(Box<Expr>),
    Pass
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add, Subtract, Multiply, Divide,
    Equality, Lesser, Greater,
    Declare, Assign
}

#[derive(Debug, Clone)]
pub enum Token {
    LParen, RParen,
    LCurly, RCurly,
    LBracket, RBracket,
    LArrow, RArrow,
    Plus, Minus, Star, Slash,
    Semicolon, Comma, Dot, Colon,
    Equal, Equality,

    String(String),
    Number(i64),
    Identifier(String),

    // Keywords
    Let,
    If,
    Else,
    While,
    For,
    Of,
    Func,
    Import,
    Include,
    Class,
    Return,

    Pass,

    EOF
}

impl Token {
    pub fn stringify(&self) -> String {
        match self {
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
            Token::LCurly => "{".to_string(),
            Token::RCurly => "}".to_string(),
            Token::LBracket => "[".to_string(),
            Token::RBracket => "]".to_string(),
            Token::LArrow => "<".to_string(),
            Token::RArrow => ">".to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Star => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Comma => ",".to_string(),
            Token::Dot => ".".to_string(),
            Token::Colon => ":".to_string(),
            Token::Equal => "=".to_string(),
            Token::Equality => "==".to_string(),

            Token::String(s) => s.to_string(),
            Token::Number(n) => n.to_string(),
            Token::Identifier(s) => s.to_string(),

            _ => {
                panic!("Cannot stringify token: {:?}", self);
            }
        }
    }
}

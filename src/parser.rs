use crate::tokens::{Token, Expr};

pub fn parse(tokens: Vec<Token>, debug_mode: bool) -> Vec<Expr> {
    let mut i = 0;
    let mut result: Vec<Expr> = Vec::new();
    
    while i < tokens.len() {
        if debug_mode {
            println!("Parsing token: {:?}", tokens[i]);
        }
        match &tokens[i] {
            Token::Number(n) => {
                // If the last token was a number, append this number to the last number.
                if result.len() > 0 {
                    let last = result.pop().unwrap();
                    match last {
                        Expr::Number(last_n) => {
                            result.push(Expr::Number(last_n * 10 + n));
                        },
                        _ => {
                            result.push(last);
                            result.push(Expr::Number(*n));
                        }
                    }
                } else {
                    result.push(Expr::Number(*n));
                }
            },
            Token::Plus => {
                if tokens.len() < i + 1 {
                    panic!("Expected number after operator");
                }

                let left = result.pop().expect("Expected number before operator");
                let right = match tokens[i + 1] {
                    Token::Number(n) => Expr::Number(n),
                    _ => panic!("Expected number after operator")
                };

                i += 1;

                result.push(Expr::BinOp('+', Box::new(left), Box::new(right)));
            },
            Token::Minus => {
                if tokens.len() < i + 1 {
                    panic!("Expected number after operator");
                }

                let left = result.pop().expect("Expected number before operator");
                let right = match tokens[i + 1] {
                    Token::Number(n) => Expr::Number(n),
                    _ => panic!("Expected number after operator")
                };

                i += 1;

                result.push(Expr::BinOp('-', Box::new(left), Box::new(right)));
            },
            Token::Star => {
                if tokens.len() < i + 1 {
                    panic!("Expected number after operator");
                }

                let left = result.pop().expect("Expected number before operator");
                let right = match tokens[i + 1] {
                    Token::Number(n) => Expr::Number(n),
                    _ => panic!("Expected number after operator")
                };

                i += 1;

                result.push(Expr::BinOp('*', Box::new(left), Box::new(right)));
            },
            Token::Slash => {
                if tokens.len() < i + 1 {
                    panic!("Expected number after operator");
                }

                let left = result.pop().expect("Expected number before operator");
                let right = match tokens[i + 1] {
                    Token::Number(n) => Expr::Number(n),
                    _ => panic!("Expected number after operator")
                };

                i += 1;

                result.push(Expr::BinOp('/', Box::new(left), Box::new(right)));
            },
            Token::Equal => {
                if tokens.len() < i + 1 {
                    panic!("Expected number after operator");
                }

                // Get every next token until a semicolon, parse it, then push a
                // `Expr::BinOp('=')` with the left side being the variable and the right side being the parsed expression

                let left = match result.pop().expect("Expected variable before operator") {
                    Expr::Identifier(s) => s,
                    _ => panic!("Expected variable before operator")
                };

                let mut right: Vec<Token> = Vec::new();
                i += 1;
                while i < tokens.len() {
                    match &tokens[i] {
                        Token::Semicolon => {
                            break;
                        },
                        _ => {}
                    }
                    right.push(tokens[i].clone());
                    i += 1;
                }

                let right = parse(right, debug_mode);

                result.push(Expr::BinOp('=', Box::new(Expr::Identifier(left)), Box::new(right[0].clone())));
            },
            Token::String(c) => {
                if result.len() > 0 {
                    let last = result.pop().unwrap();
                    match last {
                        Expr::String(s) => {
                            result.push(Expr::String(s + c));
                        },
                        _ => {
                            result.push(last);
                            result.push(Expr::String(c.to_string()));
                        }
                    }
                } else {
                    result.push(Expr::String(c.to_string()));
                }
            },
            Token::Identifier(s) => {
                result.push(Expr::Identifier(s.to_string()));
            },

            _ => {
                println!("Ignoring token: {:?}", tokens[i]);
            }
        }

        i += 1;
    }

    return result;
}

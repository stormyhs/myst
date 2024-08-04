use crate::tokens::{Token, Expr, Operator};

pub fn parse(tokens: Vec<Token>, debug_mode: bool) -> Vec<Expr> {
    let mut i = 0;
    let mut result: Vec<Expr> = Vec::new();

    let mut declaring_variable = false;
    
    while i < tokens.len() {
        if debug_mode {
            println!("Parsing token: {:?}", tokens[i]);
        }

        // NOTE: The `Plus` match is the only one right now that works as intended. I will continue
        // to work on that one specifically, then move on to the others when the logic has been figured out.
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

                let left = result.pop().expect("Expected value or variable before operator");
                let right = match &tokens[i + 1] {
                    Token::Number(n) => Expr::Number(*n),
                    Token::Identifier(s) => Expr::Identifier(s.to_string()),
                    _ => panic!("Expected value or variable before operator")
                };

                i += 1;

                result.push(Expr::BinOp(Operator::Add, Box::new(left), Box::new(right)));
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

                result.push(Expr::BinOp(Operator::Subtract, Box::new(left), Box::new(right)));
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

                result.push(Expr::BinOp(Operator::Multiply, Box::new(left), Box::new(right)));
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

                result.push(Expr::BinOp(Operator::Divide, Box::new(left), Box::new(right)));
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

                if declaring_variable {
                    result.push(Expr::BinOp(Operator::Declare, Box::new(Expr::Identifier(left)), Box::new(right[0].clone())));
                    declaring_variable = false;
                } else {
                    result.push(Expr::BinOp(Operator::Assign, Box::new(Expr::Identifier(left)), Box::new(right[0].clone())));
                }

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
            Token::Declaration(s, e) => {
                result.push(Expr::BinOp(Operator::Declare, Box::new(Expr::Identifier(s.to_string())), e.clone()));
            },
            Token::Assignment(s, e) => {
                result.push(Expr::BinOp(Operator::Assign, Box::new(Expr::Identifier(s.to_string())), e.clone()));
            },
            Token::Let => {
                declaring_variable = true;
            }

            _ => {
                if debug_mode {
                    println!("Ignoring token: {:?}", tokens[i]);
                }
            }
        }

        i += 1;
    }

    return result;
}

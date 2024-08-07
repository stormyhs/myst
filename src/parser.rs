use crate::tokens::{Token, Expr, Operator};

pub fn parse(tokens: Vec<Token>, debug_mode: bool) -> Vec<Expr> {
    let mut i = 0;
    let mut result: Vec<Expr> = Vec::new();

    let mut declaring_variable = false;

    while i < tokens.len() {
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

                let left = result.pop().expect("Expected value or variable before operator");
                let right = match &tokens[i + 1] {
                    Token::Number(n) => Expr::Number(*n),
                    Token::Identifier(s) => Expr::Identifier(s.to_string()),
                    _ => panic!("Expected value or variable before operator")
                };

                i += 1;

                result.push(Expr::BinOp(Operator::Subtract, Box::new(left), Box::new(right)));
            },
            Token::Star => {
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

                result.push(Expr::BinOp(Operator::Multiply, Box::new(left), Box::new(right)));
            },
            Token::Slash => {
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

                result.push(Expr::BinOp(Operator::Divide, Box::new(left), Box::new(right)));
            },
            Token::Equal => {
                if tokens.len() < i + 1 {
                    panic!("Expected number after operator");
                }

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
            Token::LParen => {
                let last = result.pop().unwrap();
                match last {
                    Expr::Identifier(n) => {
                        result.push(Expr::Function(n.to_string()));
                    },
                    _ => {
                        result.push(last);
                    }
                }
            }
            Token::RParen => {
                // Get every previous token up until `LParen`, find identifier that comes before
                // them, and convert it into a Function call.

                let mut args: Vec<Expr> = Vec::new();
                let mut created_call = false;

                loop {
                    let arg = match result.pop() {
                        Some(token) => token,
                        None => break
                    };

                    match arg {
                        Expr::Function(n) => {
                            result.push(Expr::Call(n.to_string(), Box::new(args)));
                            created_call = true;
                            break;
                        },
                        _ => {
                            args.push(arg.clone());
                        }
                    }
                }


                if !created_call {
                    panic!("Could not correctly parse function call and arguments");
                }
            }
            Token::LCurly => {
                if result.len() == 0 {
                    panic!("Expected condition before block");
                }
            },
            Token::RCurly => {
                if result.len() == 0 {
                    panic!("Unexpected end of block");
                }

                let mut block: Vec<Expr> = Vec::new();

                loop {
                    let expr = match result.pop() {
                        Some(token) => token,
                        None => break
                    };

                    match expr {
                        Expr::If(c, t, f) => {
                            let condition = block.pop().unwrap(); // TODO: For now, the last thing is always the condition,
                            // which shouldn't be in the block. This will be fixed later.
                            
                            block.reverse(); // Because we got the block's contents in reverse, we
                            // have to cope.

                            result.push(Expr::If(Box::new(condition.clone()), Box::new(block), Box::new(Vec::new())));
                            break;
                        },
                        _ => {
                            block.push(expr.clone());
                        }
                    }
                }
            },
            Token::LArrow => {
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

                result.push(Expr::BinOp(Operator::Lesser, Box::new(left), Box::new(right)));
            },
            Token::RArrow => {
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

                result.push(Expr::BinOp(Operator::Greater, Box::new(left), Box::new(right)));
            },
            Token::If(c, t, f) => {
                result.push(Expr::If(Box::new(*c.clone()), Box::new(t.clone()), Box::new(f.clone())));
            },
            Token::Equality => {
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

                result.push(Expr::BinOp(Operator::Equality, Box::new(left), Box::new(right)));
            },
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

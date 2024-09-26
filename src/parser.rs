use crate::tokens::{Token, Expr, Operator};

pub fn parse(tokens: Vec<Token>, debug_mode: bool) -> Vec<Expr> {
    let mut i = 0;
    let mut result: Vec<Expr> = Vec::new();

    let mut declaring_variable = false;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Number(n) => {
                result.push(Expr::Number(*n))
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
                result.push(Expr::String(c.to_string()))
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
                        result.push(Expr::Call(n.to_string(), Box::new(Vec::new())));
                    },
                    _ => {
                        result.push(last);
                    }
                }
            }
            Token::RParen => {
                let mut created_call = false;

                let mut args: Vec<Expr> = Vec::new();
                loop {
                    let arg = match result.pop() {
                        Some(token) => token,
                        None => break
                    };

                    match arg {
                        Expr::Call(n, _) => {
                            result.push(
                                Expr::Call(n.to_string(),
                                    Box::new(args.clone())
                                )
                            );
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
            Token::LCurly => { },
            Token::RCurly => { },
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
            Token::If => {
                let mut condition: Vec<Token> = Vec::new();

                loop {
                    i += 1; // Skip the `if` token
                    if i >= tokens.len() {
                        break;
                    }
                    let token = tokens[i].clone();
                    match token {
                        Token::LCurly => {
                            break;
                        },
                        _ => {
                            condition.push(token);
                        }
                    }
                }

                let condition = parse(condition, debug_mode);

                let mut block: Vec<Token> = Vec::new();
                let mut skip_curlys = 0;
                loop {
                    if i >= tokens.len() {
                        break;
                    }

                    let token = tokens[i].clone();
                    match token {
                        Token::LCurly => {
                            skip_curlys += 1;
                            block.push(token);
                        },
                        Token::RCurly => {
                            skip_curlys -= 1;
                            block.push(token);
                            if skip_curlys == 0 {
                                break;
                            }
                        },
                        _ => {
                            block.push(token);
                        }
                    }
                    i += 1;
                }

                let block = parse(block, debug_mode);

                result.push(Expr::If(Box::new(condition[0].clone()), Box::new(block.clone())));
            },
            Token::Else => {
                let mut block: Vec<Token> = Vec::new();
                let mut skip_curlys = 0;

                loop {
                    i += 1;
                    if i >= tokens.len() {
                        break;
                    }
                    let token = tokens[i].clone();
                    match token {
                        Token::LCurly => {
                            skip_curlys += 1;
                            block.push(token);
                        },
                        Token::RCurly => {
                            skip_curlys -= 1;
                            block.push(token);
                            if skip_curlys == 0 {
                                break;
                            }
                        },
                        _ => {
                            block.push(token);
                        }
                    }
                }

                let block = parse(block, debug_mode);

                let if_statement = result.pop().expect("Expected if statement before else");
                let condition = match &if_statement {
                    Expr::If(c, _) => c,
                    _ => panic!("Expected if statement before else")
                };

                result.push(if_statement.clone());
                result.push(Expr::Else(condition.clone(), Box::new(block.clone())));
            },
            Token::While => {
                let mut condition: Vec<Token> = Vec::new();

                loop {
                    i += 1;
                    if i >= tokens.len() {
                        break;
                    }
                    let token = tokens[i].clone();
                    match token {
                        Token::LCurly => {
                            condition.push(token);
                            break;
                        },
                        _ => {
                            condition.push(token);
                        }
                    }
                }

                let condition = parse(condition, debug_mode);

                let mut block: Vec<Token> = Vec::new();
                let mut skip_curlys = 0;

                loop {
                    if i >= tokens.len() {
                        break;
                    }

                    let token = tokens[i].clone();
                    match token {
                        Token::LCurly => {
                            skip_curlys += 1;
                            block.push(token);
                        },
                        Token::RCurly => {
                            skip_curlys -= 1;
                            block.push(token);
                            if skip_curlys == 0 {
                                break;
                            }
                        },
                        _ => {
                            block.push(token);
                        }
                    }
                    i += 1;
                }

                let block = parse(block, debug_mode);

                result.push(Expr::While(Box::new(condition[0].clone()), Box::new(block.clone())));
            },
            Token::Func => {
                let name = match &tokens[i + 1] {
                    Token::Identifier(name) => name,
                    _ => panic!("Expected function name after func keyword")
                };

                // Move to the start of the list of arguments
                loop {
                    i += 1;
                    if i >= tokens.len() {
                        break;
                    }
                    let token = tokens[i].clone();
                    match token {
                        Token::LParen => {
                            break;
                        },
                        _ => { }
                    }
                }

                // Collect the arguments
                let mut args: Vec<Token> = Vec::new();
                loop {
                    i += 1;
                    if i >= tokens.len() {
                        break;
                    }
                    let token = tokens[i].clone();
                    match token {
                        Token::RParen => {
                            break;
                        },
                        _ => args.push(token)
                    }
                }

                // Move to the start of the block
                let mut block: Vec<Token> = Vec::new();
                loop {
                    i += 1;
                    if i >= tokens.len() {
                        break;
                    }
                    let token = tokens[i].clone();
                    match token {
                        Token::LCurly => {
                            break;
                        },
                        _ => { }
                    }
                }

                // Collect the block
                let mut ignore_rcurlys = 0;
                loop {
                    i += 1;
                    if i >= tokens.len() {
                        break;
                    }
                    let token = tokens[i].clone();
                    match token {
                        Token::RCurly => {
                            if ignore_rcurlys > 0 {
                                ignore_rcurlys -= 1;
                                block.push(token);
                                continue;
                            }
                            break;
                        },
                        Token::LCurly => {
                            ignore_rcurlys += 1;
                            block.push(token);
                        },
                        _ => {
                            block.push(token);
                        }
                    }
                }

                let block = parse(block, debug_mode);
                let args = parse(args, debug_mode);

                result.push(Expr::Func(name.to_string(), Box::new(args), Box::new(block.clone())));
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
                    //println!("Ignoring token: {:?}", tokens[i]);
                }
            }
        }

        i += 1;
    }

    return result;
}

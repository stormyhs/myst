use crate::tokens::{Token, Expr, Operator};

fn parse_array(tokens: Vec<Token>, debug_mode: bool) -> Vec<Expr> {
    let mut array: Vec<Token> = Vec::new();
    
    let mut i = 0;
    loop {
        i += 1;
        if i >= tokens.len() {
            break;
        }
        let token = tokens[i].clone();
        match token {
            Token::RBracket => {
                break;
            },
            _ => {
                array.push(token);
            }
        }
    }

    let parsed_array = parse(array, debug_mode);

    return parsed_array;
}

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
                    Token::String(c) => Expr::String(c.to_string()),
                    Token::LBracket => {
                        parse_array(tokens[i + 1..].to_vec(), debug_mode)[0].clone()
                    },
                    _ => panic!("Expected value or variable before operator, got {:?}", tokens[i + 1])
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
                let next = tokens.get(i + 1);
                match next {
                    Some(Token::LBracket) => {
                        let index = match tokens.get(i + 2) {
                            Some(Token::Number(n)) => n.to_string(),
                            Some(Token::Identifier(s)) => s.to_string(),
                            _ => panic!("Expected number after bracket")
                        };

                        result.push(Expr::AccessProperty(Box::new(Expr::Identifier(s.to_string())), index));
                        i += 3;

                        continue;
                    },
                    _ => { }
                }

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
            Token::Import => {
                loop {
                    i += 1;
                    if i >= tokens.len() {
                        break;
                    }
                    let token = tokens[i].clone();
                    match token {
                        Token::Semicolon => {
                            break;
                        },
                        Token::Identifier(name) => {
                            result.push(Expr::Import(name.to_string()));
                            break;
                        }
                        _ => { }
                    }
                }
            },
            Token::Include => {
                loop {
                    i += 1;
                    if i >= tokens.len() {
                        break;
                    }
                    let token = tokens[i].clone();
                    match token {
                        Token::Semicolon => {
                            break;
                        },
                        Token::Identifier(name) => {
                            result.push(Expr::Include(name.to_string()));
                            break;
                        }
                        _ => { }
                    }
                }
            },

            Token::LParen => {
                let name = result.pop().expect("Expected function name before LParen");

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
                        _ => {
                            args.push(token)
                        }
                    }
                }

                let parsed_args = parse(args, debug_mode);

                result.push(Expr::Call(Box::new(name), Box::new(parsed_args)));
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
                            args.reverse();
                            result.push(
                                Expr::Call(
                                    n,
                                    Box::new(args.clone())
                                )
                            );
                            created_call = true;
                            break;
                        },
                        Expr::AccessProperty(expr, prop) => {
                            match *expr {
                                Expr::Identifier(name) => {
                                    args.reverse();
                                    result.push(
                                        Expr::Call(
                                            Box::new(Expr::AccessProperty(Box::new(Expr::Identifier(name.to_string())), prop)),
                                            Box::new(args.clone())
                                        )
                                    );
                                },
                                _ => { }
                            }
                            created_call = true;
                            break;
                        },
                        _ => { args.push(arg); }
                    }
                }

                if !created_call {
                    panic!("Could not correctly parse function call and arguments");
                }
            }
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
            Token::LBracket => {
                let mut array: Vec<Token> = Vec::new();

                loop {
                    i += 1;
                    if i >= tokens.len() {
                        break;
                    }
                    let token = tokens[i].clone();
                    match token {
                        Token::RBracket => {
                            break;
                        },
                        _ => {
                            array.push(token);
                        }
                    }
                }

                let parsed_array = parse(array, debug_mode);

                result.push(Expr::Array(Box::new(parsed_array)));
            },
            Token::RBracket => { },

            Token::Dot => {
                let last = result.pop().expect("Expected value or variable before operator");
                let property = match &tokens[i + 1] {
                    Token::Identifier(s) => s,
                    _ => panic!("Expected property after dot operator")
                };

                i += 1;

                result.push(Expr::AccessProperty(Box::new(last), property.to_string()));
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
            Token::For => {
                let iterator = match &tokens[i + 1] {
                    Token::Identifier(s) => s,
                    _ => panic!("Expected variable after for keyword")
                };

                let mut is_variable = true;
                let array = match &tokens[i + 3] {
                    Token::Identifier(s) => Expr::Identifier(s.to_string()),
                    Token::LBracket => {
                        is_variable = false;
                        parse_array(tokens[i + 2..].to_vec(), debug_mode)[0].clone()
                    }
                    _ => panic!("Expected array after variable in for loop")
                };

                if !is_variable {
                    loop {
                        i += 1;
                        if i >= tokens.len() {
                            break;
                        }
                        match &tokens[i] {
                            Token::RBracket => {
                                break;
                            },
                            _ => { }
                        }
                    }
                } else {
                    i += 3;
                }

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

                result.push(Expr::For(iterator.to_string(), Box::new(array), Box::new(block.clone())));
            }

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
                    Token::String(c) => Expr::String(c.to_string()),
                    Token::Identifier(s) => Expr::Identifier(s.to_string()),
                    _ => panic!("Expected value or variable before operator")
                };

                i += 1;

                result.push(Expr::BinOp(Operator::Equality, Box::new(left), Box::new(right)));
            },

            Token::LCurly => { },
            Token::RCurly => { },
            Token::Semicolon => { },
            Token::Comma => { },

            _ => {
                if debug_mode {
                    println!("Unhandled token: {:?}", tokens[i]);
                }
            }
        }

        i += 1;
    }

    return result;
}

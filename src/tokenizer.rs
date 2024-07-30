use crate::tokens::{Token, Operator};

pub fn tokenize(source: String, debug_mode: bool) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let lines: Vec<&str> = source.lines().collect();
    
    for line in lines {
        let mut in_string = false;

        for c in line.chars() {
            if debug_mode {
                // println!("char: {}", c);
                // println!("tokens: {:?}", tokens);
            }

            match c {
                '0'..='9' => {
                    if tokens.len() == 0 {
                        tokens.push(Token::Number(c.to_digit(10).unwrap() as i16));
                        continue;
                    }

                    let last = tokens.pop().unwrap();
                    match last {
                        Token::Number(n) => {
                            let new_number = n * 10 + c.to_digit(10).unwrap() as i16;
                            tokens.push(Token::Number(new_number));
                        },
                        Token::Identifier(name) => {
                            tokens.pop();
                            tokens.push(Token::Variable(name, c.to_digit(10).unwrap() as i16));
                        },
                        Token::Variable(name, value) => {
                            // Rebuid the variable with `value` + `c` (append, not add)
                            tokens.push(Token::Variable(name, value * 10 + c.to_digit(10).unwrap() as i16));
                        },
                        Token::Assign => {
                            let identifier = match tokens.pop().unwrap() {
                                Token::Identifier(name) => name,
                                _ => panic!("Expected identifier after assign")
                            };

                            tokens.push(Token::Variable(identifier, c.to_digit(10).unwrap() as i16));
                        },
                        _ => {
                            tokens.push(last);
                            tokens.push(Token::Number(c.to_digit(10).unwrap() as i16))
                        }
                    }
                },
                '+' => tokens.push(Token::Operator(Operator::Add)),
                '-' => tokens.push(Token::Operator(Operator::Subtract)),
                '*' => tokens.push(Token::Operator(Operator::Multiply)),
                '/' => tokens.push(Token::Operator(Operator::Divide)),
                '=' => tokens.push(Token::Assign),
                ';' => tokens.push(Token::Semicolon),
                '(' => {
                    let last = tokens.pop().unwrap();
                    match last {
                        Token::Identifier(name) => {
                            tokens.push(Token::Call(name, Vec::new()));
                        },
                        _ => panic!("Expected identifier before argument list")
                    }
                },
                ')' => {
                    // Pop everything up to `Call`, and add it into the `Call` arguments list
                    let mut args: Vec<Token> = Vec::new();
                    loop {
                        let last = tokens.pop().unwrap();
                        if debug_mode {
                            println!("popped arg: {:?}", last);
                        }
                        match last {
                            Token::Call(_, _) => {
                                tokens.push(last);
                                break;
                            },
                            _ => args.push(last)
                        }
                    }

                    match tokens.pop().unwrap() {
                        Token::Call(name, _) => tokens.push(Token::Call(name, args)),
                        _ => panic!("Expected call token")
                    };
                },
                '"' => {
                    if in_string {
                        in_string = false;
                    } else {
                        in_string = true;
                    }
                },
                ' ' => {
                    if in_string {
                        let last = tokens.pop().unwrap();
                        match last {
                            Token::String(s) => {
                                tokens.push(Token::String(s + &c.to_string()));
                            },
                            _ => {
                                tokens.push(last);
                                tokens.push(Token::String(c.to_string()));
                            }
                        }
                    }
                }
                _ => {
                    if in_string {
                        if tokens.len() == 0 {
                            tokens.push(Token::String(c.to_string()));
                            continue;
                        }

                        let last = tokens.pop().unwrap();
                        match last {
                            Token::String(s) => {
                                tokens.push(Token::String(s + &c.to_string()));
                            },
                            _ => {
                                tokens.push(last);
                                tokens.push(Token::String(c.to_string()));
                            }
                        }

                        continue;
                    }

                    if tokens.len() == 0 {
                        tokens.push(Token::Identifier(c.to_string()));
                        continue;
                    }

                    let last = tokens.pop().unwrap();
                    match last {
                        Token::Identifier(name) => {
                            if c.is_alphabetic() {
                                tokens.push(Token::Identifier(name + &c.to_string()));
                            } else {
                                panic!("Identifier must be alphanumeric");
                            }
                        },
                        Token::Assign => {
                            tokens.push(Token::Variable(c.to_string(), 0));
                        }
                        Token::Semicolon => {
                            tokens.push(Token::Identifier(c.to_string()));
                        },
                        _ => {
                            tokens.push(last);
                            tokens.push(Token::Identifier(c.to_string()));
                        }
                        //_ => panic!("Unexpected token while building identifier: {:?} - char: {}", last, c)
                    }
                }
            }
        }
    }

    return tokens;
}


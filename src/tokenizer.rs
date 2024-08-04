use crate::tokens::{ Token, Expr };

pub fn tokenize(source: String, debug_mode: bool) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let lines: Vec<&str> = source.lines().collect();

    let mut in_string = false;

    for line in lines {
        for c in line.chars() {
            if debug_mode {
                println!("tokens: {:?}", tokens);
            }

            if c != '"' && in_string {
                let last = tokens.pop().unwrap_or(Token::String(String::new()));
                
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

            match c {
                '0'..='9' => {
                    // If the last token is a number, append the digit to it.
                    // Otherwise, push a new number token.
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
                        Token::String(s) => {
                            tokens.pop();
                            tokens.push(Token::String(s + &c.to_string()));
                        },
                        _ => {
                            tokens.push(last);
                            tokens.push(Token::Number(c.to_digit(10).unwrap() as i16));
                        }
                    }
                },
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Star),
                '/' => tokens.push(Token::Slash),
                '=' => {
                    /*
                    let last = tokens.pop().expect("Expected identifier before assignment");
                    match &last {
                        Token::Identifier(s) => {
                            if s == "let" {
                                tokens.push(Token::Let);
                            }
                        },
                        _ => { }
                    }
                    tokens.push(last);
                    tokens.push(Token::Equal)
                    */
                    tokens.push(Token::Equal)
                },
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                '"' => {
                    in_string = !in_string;
                },
                ';' => tokens.push(Token::Semicolon),
                ' ' => {
                    if tokens.len() == 0 {
                        continue;
                    }

                    let last = tokens.pop().unwrap();
                    match last {
                        Token::Identifier(s) => {
                            if s == "let" {
                                tokens.push(Token::Let);
                            } else {
                                tokens.push(Token::Identifier(s));
                            }
                        },
                        _ => {
                            tokens.push(last);
                        }
                    }
                },
                _ => {
                    if in_string {
                        let last = tokens.pop().unwrap_or(Token::String(String::new()));

                        match last {
                            Token::String(s) => {
                                tokens.push(Token::String(s + &c.to_string()));
                            },
                            _ => {
                                tokens.push(last);
                                tokens.push(Token::String(c.to_string()));
                            }
                        }
                    } else {
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
                            Token::Equal => {
                                tokens.push(last);
                                tokens.push(Token::Identifier(c.to_string()));
                            },
                            Token::Semicolon => {
                                tokens.push(last);
                                tokens.push(Token::Identifier(c.to_string()));
                            },
                            _ => {
                                tokens.push(last);
                                tokens.push(Token::Identifier(c.to_string()));
                            }
                        }
                    }
                }
            }
        }
    }

    return tokens;
}

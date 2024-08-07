use crate::tokens::{ Token, Expr };

pub fn tokenize(source: String, debug_mode: bool) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let lines: Vec<&str> = source.lines().collect();

    let mut in_string = false;
    let mut in_comment = false;

    for line in lines {
        in_comment = false;
        for c in line.chars() {
            if in_comment {
                continue;
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
                        tokens.push(Token::Number(c.to_digit(10).unwrap() as i64));
                        continue;
                    }

                    let last = tokens.pop().unwrap();

                    match last {
                        Token::Number(n) => {
                            let new_number = n * 10 + c.to_digit(10).unwrap() as i64;
                            tokens.push(Token::Number(new_number));
                        },
                        Token::String(s) => {
                            tokens.push(Token::String(s + &c.to_string()));
                        },
                        Token::Identifier(s) => {
                            tokens.push(Token::Identifier(s + &c.to_string()));
                        }
                        _ => {
                            tokens.push(last);
                            tokens.push(Token::Number(c.to_digit(10).unwrap() as i64));
                        }
                    }
                },
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Star),
                '/' => {
                    if tokens.len() == 0 {
                        tokens.push(Token::Slash);
                        continue;
                    }

                    let last = tokens.pop().unwrap();
                    match last {
                        Token::Slash => {
                            in_comment = true;
                        },
                        _ => {
                            tokens.push(last);
                            tokens.push(Token::Slash);
                        }
                    }
                },
                '=' => {
                    if tokens.len() == 0 {
                        tokens.push(Token::Equal);
                        continue;
                    }

                    let last = tokens.pop().unwrap();
                    match last {
                        Token::Equal => {
                            tokens.push(Token::Equality);
                        },
                        _ => {
                            tokens.push(last);
                            tokens.push(Token::Equal);
                        }
                    }
                },
                '(' => tokens.push(Token::LParen),
                ')' => tokens.push(Token::RParen),
                '"' => in_string = !in_string,
                ';' => tokens.push(Token::Semicolon),
                '{' => tokens.push(Token::LCurly),
                '}' => tokens.push(Token::RCurly),
                '>' => tokens.push(Token::RArrow),
                '<' => tokens.push(Token::LArrow),
                ' ' => {
                    if tokens.len() == 0 {
                        continue;
                    }

                    let last = tokens.pop().unwrap();
                    match last {
                        Token::Identifier(s) => {
                            if s == "let" {
                                tokens.push(Token::Let);
                            } else if s == "if" {
                                tokens.push(Token::If(Box::new(Expr::Number(0)), Vec::new(), Vec::new()));
                            } else if s == "while" {
                                tokens.push(Token::While(Box::new(Expr::Number(0)), Vec::new()));
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
                                    panic!("Identifier must be alphanumeric: {name}{c}");
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

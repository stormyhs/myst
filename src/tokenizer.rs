use crate::tokens::{ Token, Expr };

pub fn tokenize(source: String, debug_mode: bool) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let lines: Vec<&str> = source.lines().collect();

    let mut in_string = false;
    let mut in_comment = false;
    let mut declaring_for = false;

    for line in lines {
        in_comment = false;
        for c in line.chars() {
            if in_comment {
                continue;
            }

            if c != '"' && in_string {
                let last = tokens.pop().unwrap_or(Token::String(String::new()));
                
                match last {
                    Token::Comma => {
                        tokens.push(last);
                        tokens.push(Token::String(c.to_string()));
                    },
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
                '_' => {
                    if tokens.len() == 0 {
                        panic!("Unexpected character: _");
                    }

                    let last = tokens.pop().unwrap();
                    match last {
                        Token::Identifier(s) => {
                            tokens.push(Token::Identifier(s + &c.to_string()));
                        },
                        Token::Number(_n) => {
                            tokens.push(last);
                            // Do nothing. This means that we can have numbers like 1_000_000,
                            // which are more human readable.
                        },
                        _ => {
                            tokens.push(last);
                            tokens.push(Token::Identifier(c.to_string()));
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
                '{' => {
                    tokens.push(Token::LCurly);
                    if declaring_for {
                        declaring_for = false;
                    }
                },
                '}' => tokens.push(Token::RCurly),
                '>' => tokens.push(Token::RArrow),
                '<' => tokens.push(Token::LArrow),
                '[' => tokens.push(Token::LBracket),
                ']' => tokens.push(Token::RBracket),
                ',' => tokens.push(Token::Comma),
                '.' => tokens.push(Token::Dot),
                ' ' => {
                    if tokens.len() == 0 {
                        continue;
                    }

                    let last = tokens.pop().unwrap();
                    match last {
                        Token::Identifier(s) => {
                            if s == "let" {
                                tokens.push(Token::Let);
                            }
                            else if s == "if" {
                                tokens.push(Token::If);
                            }
                            else if s == "else" {
                                tokens.push(Token::Else);
                            }
                            else if s == "while" {
                                tokens.push(Token::While);
                            }
                            else if s == "for" {
                                tokens.push(Token::For);
                                declaring_for = true;
                            }
                            else if s == "of" {
                                tokens.push(Token::Of);
                            }
                            else if s == "func" {
                                tokens.push(Token::Func);
                            }
                            else if s == "import" {
                                tokens.push(Token::Import);
                            }
                            else if s == "include" {
                                tokens.push(Token::Include);
                            }
                            else {
                                tokens.push(Token::Identifier(s.clone()));
                                if declaring_for {
                                    tokens.push(Token::Identifier("".to_string()));
                                    declaring_for = false;
                                }
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
                    }
                    else {
                        if tokens.len() == 0 {
                            tokens.push(Token::Identifier(c.to_string()));
                            continue;
                        }

                        let last = tokens.pop().unwrap();
                        match last {
                            Token::Identifier(name) => {
                                if !c.is_alphabetic() {
                                    panic!("Identifier must be alphanumeric: {name}{c}");
                                }

                                tokens.push(Token::Identifier(name + &c.to_string()));
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

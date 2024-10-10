use crate::enums::Token;

pub fn tokenize(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let lines: Vec<&str> = source.lines().collect();

    let mut in_string = false;
    let mut declaring_for = false;

    for line in lines {
        let mut in_comment = false;

        // Poor man's line tracking.
        // This is to prevent `//` comments from triggering even if they are in seperate lines.
        let mut slash_in_line = false; 

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
                        Token::Minus => {
                            let behind = tokens.pop().unwrap();
                            match behind {
                                Token::Plus | Token::Minus | Token::Equal => {
                                    tokens.push(behind);
                                    let number = c.to_digit(10).unwrap() as i64;
                                    tokens.push(Token::Number(-number));
                                },
                                _ => {
                                    tokens.push(behind);
                                    tokens.push(last);
                                    let number = c.to_digit(10).unwrap() as i64;
                                    tokens.push(Token::Number(number));
                                }
                            }
                        }
                        Token::Number(n) => {
                            if n < 0 {
                                let new_number = n * 10 - c.to_digit(10).unwrap() as i64;
                                tokens.push(Token::Number(new_number));
                            } else {
                                let new_number = n * 10 + c.to_digit(10).unwrap() as i64;
                                tokens.push(Token::Number(new_number));
                            }
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
                            if slash_in_line {
                                in_comment = true;
                            }
                        },
                        _ => {
                            tokens.push(last);
                            tokens.push(Token::Slash);
                        }
                    }

                    slash_in_line = true;
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
                '"' => {
                    if !in_string{
                        tokens.push(Token::String(String::new()));
                    }

                    in_string = !in_string
                },
                ';' => {
                    let last = tokens.pop().unwrap();

                    match last {
                        Token::Identifier(s) => {
                            if s == "return" {
                                tokens.push(Token::Return);
                                tokens.push(Token::Semicolon);
                            }
                            else {
                                tokens.push(Token::Identifier(s));
                                tokens.push(Token::Semicolon);
                            }
                        },
                        _ => {
                            tokens.push(last);
                            tokens.push(Token::Semicolon);
                        }
                    }
                },
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
                ':' => tokens.push(Token::Colon),
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
                            else if s == "fn" {
                                tokens.push(Token::Func);
                            }
                            else if s == "import" {
                                tokens.push(Token::Import);
                            }
                            else if s == "include" {
                                tokens.push(Token::Include);
                            }
                            else if s == "class" {
                                tokens.push(Token::Class);
                            }
                            else if s == "return" {
                                tokens.push(Token::Return);
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

    // Remove multiline comments
    let mut new_tokens: Vec<Token> = Vec::new();

    let mut in_multiline_comment = false;
    let mut i = 0;
    loop {
        if i >= tokens.len() {
            break;
        }

        match tokens[i] {
            Token::Slash => {
                if i + 1 < tokens.len() {
                    match tokens[i + 1] {
                        Token::Star => {
                            in_multiline_comment = true;
                        },
                        _ => {
                            if !in_multiline_comment {
                                new_tokens.push(tokens[i].clone());
                            }
                        }
                    }
                }
            },
            Token::Star => {
                if i + 1 < tokens.len() {
                    match tokens[i + 1] {
                        Token::Slash => {
                            in_multiline_comment = false;
                            i += 1;
                        },
                        _ => {
                            if !in_multiline_comment {
                                new_tokens.push(tokens[i].clone());
                            }
                        }
                    }
                }
            }
            _ => {
                if !in_multiline_comment {
                    new_tokens.push(tokens[i].clone());
                }
            }
        }

        i +=1 ;
    }

    return new_tokens;
}

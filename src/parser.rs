use std::collections::HashMap;

use crate::tokens::{Token, Operator, Expr, Value};

pub fn parse(tokens: Vec<Token>) -> Vec<Expr> {
    // [Number(1), Operator('+'), Number(4), Operator('-'), Number(3), Semicolon]
    
    let mut i = 0;
    let mut state: HashMap<String, Value> = HashMap::new();
    let mut result: Vec<Expr> = Vec::new();
    
    while i < tokens.len() {
        match &tokens[i] {
            Token::Number(n) => {
                result.push(Expr::Number(*n));
            },
            Token::Operator(op) => {
                match op {
                    Operator::Add => {
                        if tokens.len() < i + 1 {
                            panic!("Expected number after operator");
                        }

                        let left = result.pop().expect("Expected number before operator");

                        let right = match &tokens[i + 1] {
                            Token::Number(n) => Expr::Number(*n),
                            Token::Variable(_, value) => {
                                let value = match value {
                                    Value::Number(n) => n,
                                    _ => panic!("Expected number after variable")
                                };
                                Expr::Number(value.clone())
                            }
                            Token::Identifier(name) => {
                                if state.contains_key(&name.to_string()) {
                                    let value = &state[&name.to_string()];
                                    let value = match value {
                                        Value::Number(value) => value,
                                        _ => panic!("Expected number after variable")
                                    };
                                    Expr::Number(*value)
                                } else {
                                    panic!("Identifier '{}' does not coorelate to a value", name);
                                }
                            },
                            _ => panic!("Expected number after operator")
                        };

                        // Skip the next token, which is the right operand
                        i += 1;

                        result.push(Expr::BinOp('+', Box::new(left), Box::new(right)));
                    },
                    Operator::Subtract => {
                        if tokens.len() < i + 1 {
                            panic!("Expected number after operator");
                        }

                        let left = result.pop().expect("Expected number before operator");
                        let right = match tokens[i + 1] {
                            Token::Number(n) => Expr::Number(n),
                            _ => panic!("Expected number after operator")
                        };

                        // Skip the next token, which is the right operand
                        i += 1;

                        result.push(Expr::BinOp('-', Box::new(left), Box::new(right)));
                    },
                    Operator::Multiply => {
                        if tokens.len() < i + 1 {
                            panic!("Expected number after operator");
                        }

                        let left = result.pop().expect("Expected number before operator");
                        let right = match tokens[i + 1] {
                            Token::Number(n) => Expr::Number(n),
                            _ => panic!("Expected number after operator")
                        };

                        // Skip the next token, which is the right operand
                        i += 1;

                        result.push(Expr::BinOp('*', Box::new(left), Box::new(right)));
                    }
                    Operator::Divide => {
                        if tokens.len() < i + 1 {
                            panic!("Expected number after operator");
                        }

                        let left = result.pop().expect("Expected number before operator");
                        let right = match tokens[i + 1] {
                            Token::Number(n) => Expr::Number(n),
                            _ => panic!("Expected number after operator")
                        };

                        // Skip the next token, which is the right operand
                        i += 1;

                        result.push(Expr::BinOp('/', Box::new(left), Box::new(right)));
                    },
                }
            },

            Token::Variable(name, value) => {
                let owned_value: Value = match value {
                    Value::Number(n) => Value::Number(*n),
                    Value::String(s) => Value::String(s.to_string())
                };
                state.insert(name.to_string(), owned_value);
            },

            Token::Identifier(name) => {
                if state.contains_key(name) {
                    let value = &state[name];

                    match value {
                        Value::Number(n) => result.push(Expr::Number(*n)),
                        Value::String(s) => result.push(Expr::String(s.to_string()))
                    }
                } else {
                    panic!("Identifier '{}' does not coorelate to a value", name);
                }
            },

            Token::Semicolon => {
                // Do nothing :)
            }

            Token::Call(name, args) => {
                if name == "print" {
                    for arg in args {
                        let value = match arg {
                            Token::Number(n) => n.to_string(),
                            Token::Variable(_, value) => {
                                let value = match value {
                                    Value::Number(n) => n,
                                    _ => panic!("Expected number after variable")
                                };
                                value.to_string()
                            },
                            Token::Identifier(name) => {
                                if state.contains_key(&name.to_string()) {
                                    let value = &state[&name.to_string()];
                                    match value {
                                        Value::Number(n) => n.to_string(),
                                        Value::String(s) => s.to_string()
                                    }
                                } else {
                                    panic!("Identifier '{}' does not coorelate to a value", name);
                                }
                            },
                            Token::String(s) => s.to_string(),
                            _ => panic!("Expected number after operator")
                        };

                        println!("# {}", value);
                    }
                } else {
                    panic!("Unknown function: {}", name);
                }

            }

            _ => {
                println!("Unknown token: {:?}", tokens[i]);
            }
        }

        i += 1;
    }

    return result;
}


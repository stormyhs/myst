use crate::tokens::{Token, Operator, Expr};

pub fn parse(tokens: Vec<Token>) -> Vec<Expr> {
    // [Number(1), Operator('+'), Number(4), Operator('-'), Number(3)]
    
    let mut i = 0;
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
                        let right = match tokens[i + 1] {
                            Token::Number(n) => Expr::Number(n),
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
                println!("Variable: {} = {}", name, value);
            },

            Token::Assign => {
                println!("Skipping assign token");
            }

            _ => {
                println!("Unknown token: {:?}", tokens[i]);
            }
        }

        i += 1;
    }

    return result;
}


use std::fs;
use std::env;
use std::io::Write;

#[derive(Debug)]
enum Token {
    Number(i16),
    Operator(Operator),

    EOF
}

#[derive(Debug)]
enum Expr {
    Number(i16),
    BinOp(char, Box<Expr>, Box<Expr>),
    // UnaryOp(char, Box<Expr>)
}

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

fn tokenize(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let lines: Vec<&str> = source.lines().collect();
    
    for line in lines {
        for c in line.chars() {
            match c {
                '0'..='9' => {
                    if tokens.len() == 0 {
                        tokens.push(Token::Number(c.to_digit(10).unwrap() as i16));
                        continue;
                    }
                    match tokens[tokens.len() - 1] {
                        Token::Number(n) => {
                            let new_number = n * 10 + c.to_digit(10).unwrap() as i16;
                            tokens.pop();
                            tokens.push(Token::Number(new_number));
                        },
                        _ => tokens.push(Token::Number(c.to_digit(10).unwrap() as i16))
                    }
                },
                '+' => tokens.push(Token::Operator(Operator::Add)),
                '-' => tokens.push(Token::Operator(Operator::Subtract)),
                '*' => tokens.push(Token::Operator(Operator::Multiply)),
                '/' => tokens.push(Token::Operator(Operator::Divide)),
                _ => {}
            }
        }
    }

    tokens.push(Token::EOF);

    return tokens;
}

fn parse(tokens: Vec<Token>) -> Vec<Expr> {
    // [Number(1), Operator('+'), Number(4), Operator('-'), Number(3), EOF]
    
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
                    }
                }
            }

            _ => {}
        }

        i += 1;
    }

    return result;
}

fn evaluate(expr: Vec<Expr>) -> i16 {
    let mut result = 0;

    for e in expr {
        match e {
            Expr::Number(n) => {
                result = n;
            },
            Expr::BinOp(op, left, right) => {
                let left = evaluate(vec![*left]);
                let right = evaluate(vec![*right]);

                match op {
                    '+' => result = left + right,
                    '-' => result = left - right,
                    '*' => result = left * right,
                    '/' => result = left / right,
                    _ => {}
                }
            }
        }
    }

    return result;
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut repl_mode = false;
    let mut source = String::new();

    for arg in &args {
        match arg.as_str() {
            "--repl" => {
                repl_mode = true;
            },

            _ => {
                if source == "" {
                    source = arg.to_string();
                }
            }
        }
    }

    if repl_mode {
        loop {
            print!("> ");
            _ = std::io::stdout().flush();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "exit" {
                break;
            }

            let tokens = tokenize(input);

            let parsed = parse(tokens);

            let result = evaluate(parsed);

            println!("# {}", result);
        }
    }
    else {
        if source == "" {
            println!("No source file path provided. Did you mean to use --repl?");
            return;
        }

        let source = fs::read_to_string(source)
            .expect("Unable to read source file");

        let tokens = tokenize(source);

        let parsed = parse(tokens);

        println!("{:?}", parsed);

        let result = evaluate(parsed);

        println!("{}", result);
    }

}

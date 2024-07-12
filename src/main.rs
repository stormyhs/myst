
use std::fs;

// enum Operator {
//     Add, Subtract
// }


#[derive(Debug)]
enum Token {
    Number(i16),
    Operator(char),

    EOF
}

#[derive(Debug)]
enum Expr {
    Number(i16),
    BinOp(char, Box<Expr>, Box<Expr>),
    UnaryOp(char, Box<Expr>)
}

fn tokenize(source: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let lines: Vec<&str> = source.lines().collect();
    
    for line in lines {
        for c in line.chars() {
            match c {
                '0'..='9' => tokens.push(Token::Number(
                    c.to_digit(10).unwrap() as i16
                )),
                '+' | '-' => tokens.push(Token::Operator(c)),
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
        match tokens[i] {
            Token::Number(n) => {
                result.push(Expr::Number(n));
            },
            Token::Operator(op) => {
                match op {
                    '+' | '-' => {
                        if tokens.len() < i + 1 {
                            panic!("Expected number after operator");
                        }

                        let left = result.pop().unwrap();
                        let right = match tokens[i + 1] {
                            Token::Number(n) => Expr::Number(n),
                            _ => panic!("Expected number after operator")
                        };

                        // Skip the next token, which is the right operand
                        i += 1;

                        result.push(Expr::BinOp(op, Box::new(left), Box::new(right)));
                    },
                    _ => {}
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
                    _ => {}
                }
            },
            _ => {}
        }
    }

    return result;
}

fn main() {
    let source = fs::read_to_string("/home/stormy/code/myst/myst/calc.my")
        .expect("Unable to read source file");

    let tokens = tokenize(source);

    let parsed = parse(tokens);

    println!("{:?}", parsed);

    let result = evaluate(parsed);

    println!("{}", result);
}

use std::collections::HashMap;

use crate::tokens::Expr;

pub fn evaluate(expr: Vec<Expr>, state: &HashMap<String, i16>) -> i16 {
    let mut result = 0;
    let mut state = state.clone();

    for e in expr {
        match e {
            Expr::Number(n) => {
                result = n;
            },
            Expr::String(_s) => {
                panic!("#! Unable to evaluate string");
            },
            Expr::BinOp(op, left, right) => {
                let left = evaluate(vec![*left], &state);
                let right = evaluate(vec![*right], &state);

                match op {
                    '+' => result = left + right,
                    '-' => result = left - right,
                    '*' => result = left * right,
                    '/' => result = left / right,
                    '=' => {
                        state.insert("x".to_string(), right);
                    }
                    _ => {}
                }
            }
        }
    }

    return result;
}


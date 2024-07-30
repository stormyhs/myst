use std::collections::HashMap;

use crate::tokens::Expr;

pub fn evaluate(expr: Vec<Expr>, state: &HashMap<String, i16>) -> i16 {
    let mut return_value = 0;
    let mut state = state.clone();

    for e in expr {
        match e {
            Expr::Number(n) => {
                return_value = n;
            },
            Expr::String(_s) => {
                return_value = 0;
            },
            Expr::BinOp(op, left, right) => {
                let left = evaluate(vec![*left], &state);
                let right = evaluate(vec![*right], &state);

                match op {
                    '+' => return_value = left + right,
                    '-' => return_value = left - right,
                    '*' => return_value = left * right,
                    '/' => return_value = left / right,
                    '=' => {
                        state.insert("x".to_string(), right);
                    }
                    _ => {}
                }
            }
        }
    }

    return return_value;
}


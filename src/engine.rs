use std::collections::HashMap;
use crate::tokens::{Expr, Operator};

pub fn evaluate(expr: Vec<Expr>, state: &mut HashMap<String, Expr>, debug_mode: bool) -> Vec<Expr> {
    let mut result: Vec<Expr> = Vec::new();

    if debug_mode {
        println!("Evaluating expression: {:?}", expr);
    }

    let mut i = 0;
    while i < expr.len() {
        match &expr[i] {
            Expr::BinOp(op, left, right) => {
                // NOTE: We assume that both sides will evaluate to a vector of a single item.

                let mut did_operate = false;

                let left = match op {
                    // If its a declaration or assignment, we dont evaluate the left side.
                    // We do this to avoid acquiring the value, because we need the name instead.
                    
                    Operator::Declare | Operator::Assign => {
                        vec![*left.clone()]
                    },
                    _ => {
                        evaluate(vec![*left.clone()], state, debug_mode)
                    }
                };

                let right = evaluate(vec![*right.clone()], state, debug_mode);

                match (&left[0], &right[0]) {
                    (Expr::Number(l), Expr::Number(r)) => {
                        match *op {
                            Operator::Add => {
                                result.push(Expr::Number(l + r));
                                did_operate = true;
                            },
                            Operator::Subtract => {
                                result.push(Expr::Number(l - r));
                                did_operate = true;
                            },
                            Operator::Multiply => {
                                result.push(Expr::Number(l * r));
                                did_operate = true;
                            },
                            Operator::Divide => {
                                result.push(Expr::Number(l / r));
                                did_operate = true;
                            },
                            _ => { panic!("Invalid operator for two number values: {:?}", op) }
                        }
                    },
                    _ => { }
                }

                match (&left[0], &right[0]) {
                    (Expr::Identifier(l), Expr::Number(n)) => {
                        match *op {
                            Operator::Declare => {
                                if state.contains_key(&format!("{}", l)) {
                                    panic!("Cannot redeclare variable: {}", l);
                                }

                                state.insert(format!("{}", l), Expr::Number(*n));
                                did_operate = true;
                            },
                            Operator::Assign => {
                                if !state.contains_key(&format!("{}", l)) {
                                    panic!("Cannot assign to undeclared variable: {}", l);
                                }

                                state.insert(format!("{}", l), Expr::Number(*n));
                                did_operate = true;
                            },
                            _ => { panic!("Cannot operate on undefined variable"); }
                        }
                    },
                    _ => { }
                }

                if !did_operate {
                    panic!("Invalid operation: {:?} {:?} {:?}", left, op, right);
                }
            },
            Expr::Number(n) => {
                result.push(Expr::Number(*n));
            },
            Expr::String(s) => {
                result.push(Expr::String(s.to_string()));
            },
            Expr::Identifier(s) => {
                if state.contains_key(s) {
                    result.push(state[s].clone());
                } else {
                    result.push(Expr::Identifier(s.to_string()));
                }
            },
            _ => {}
        }

        i += 1;
    }

    return result;
}


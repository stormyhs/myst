use std::collections::HashMap;
use crate::tokens::{Expr, Operator};

pub fn evaluate(expr: Vec<Expr>, state: &mut HashMap<String, Expr>, debug_mode: bool) -> Vec<Expr> {
    let mut result: Vec<Expr> = Vec::new();

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
                            Operator::Equality => {
                                result.push(Expr::Number(if l == r { 1 } else { 0 }));
                                did_operate = true;
                            },
                            Operator::Lesser => {
                                result.push(Expr::Number(if l < r { 1 } else { 0 }));
                                did_operate = true;
                            },
                            Operator::Greater => {
                                result.push(Expr::Number(if l > r { 1 } else { 0 }));
                                did_operate = true;
                            },
                            _ => { panic!("Invalid operator for two number values: {:?}", op) }
                        }
                    },
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
                            Operator::Equality => {
                                if !state.contains_key(&format!("{}", l)) {
                                    panic!("Cannot compare undeclared variable: {}", l);
                                }

                                if debug_mode {
                                    println!("Comparing variable: {}", l);
                                }

                                let value = state[&format!("{}", l)].clone();
                                match value {
                                    Expr::Number(v) => {
                                        result.push(Expr::Number(if v == *n { 1 } else { 0 }));
                                    },
                                    _ => { panic!("Cannot compare non-number variable: {}", l); }
                                }
                                did_operate = true;
                            },
                            Operator::Lesser => {
                                if !state.contains_key(&format!("{}", l)) {
                                    panic!("Cannot compare undeclared variable: {}", l);
                                }

                                if debug_mode {
                                    println!("Comparing variable: {}", l);
                                }

                                let value = state[&format!("{}", l)].clone();
                                match value {
                                    Expr::Number(v) => {
                                        result.push(Expr::Number(if v < *n { 1 } else { 0 }));
                                    },
                                    _ => { panic!("Cannot compare non-number variable: {}", l); }
                                }
                                did_operate = true;
                            },
                            Operator::Greater => {
                                if !state.contains_key(&format!("{}", l)) {
                                    panic!("Cannot compare undeclared variable: {}", l);
                                }

                                if debug_mode {
                                    println!("Comparing variable: {}", l);
                                }

                                let value = state[&format!("{}", l)].clone();
                                match value {
                                    Expr::Number(v) => {
                                        result.push(Expr::Number(if v > *n { 1 } else { 0 }));
                                    },
                                    _ => { panic!("Cannot compare non-number variable: {}", l); }
                                }
                                did_operate = true;
                            },
                            _ => { panic!("Cannot operate on undefined variable"); }
                        }
                    },
                    (Expr::Identifier(l), Expr::String(n)) => {
                        match *op {
                            Operator::Declare => {
                                if state.contains_key(&format!("{}", l)) {
                                    panic!("Cannot redeclare variable: {}", l);
                                }

                                state.insert(format!("{}", l), Expr::String(n.to_string()));
                                did_operate = true;
                            },
                            Operator::Assign => {
                                if !state.contains_key(&format!("{}", l)) {
                                    panic!("Cannot assign to undeclared variable: {}", l);
                                }

                                state.insert(format!("{}", l), Expr::String(n.to_string()));
                                did_operate = true;
                            }
                            _ => { panic!("Invalid operator on variable and string: {:?}", op); }
                        }
                    }
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
                    println!("Variable not found: {}", s);
                    result.push(Expr::Identifier(s.to_string()));
                }
            },
            Expr::Call(function, args) => {
                let mut args = args.clone();
                args.reverse();
                if function == "print" {
                    let mut i = 0;
                    while i < args.len() {
                        let value = evaluate(vec![args[i].clone()], state, debug_mode);
                        let value = &value[0];
                        match value {
                            Expr::String(s) => {
                                print!("{s}");
                            },
                            Expr::Number(n) => {
                                print!("{n}");
                            },
                            _ => { panic!("Invalid argument to print"); }
                        };
                        i += 1;
                    }
                }
                else if function == "println" {
                    let mut i = 0;
                    if args.len() == 0 {
                        println!();
                    }
                    while i < args.len() {
                        let value = evaluate(vec![args[i].clone()], state, debug_mode);
                        let value = &value[0];
                        match value {
                            Expr::String(s) => {
                                print!("{s}");
                            },
                            Expr::Number(n) => {
                                print!("{n}");
                            },
                            _ => { panic!("Invalid argument to println: {:?}", value); }
                        };
                        i += 1;
                    }
                    println!();
                }
                else if state.contains_key(function) {
                    let function = state[function].clone();
                    match function {
                        Expr::Func(name, arg_names, block) => {
                            let mut new_state = state.clone();
                            for i in 0..args.len() {
                                let name = match &arg_names[i] {
                                    Expr::Identifier(n) => n,
                                    _ => { panic!("Invalid argument name"); }
                                };

                                new_state.insert(name.to_string(), args[i].clone());
                            }
                            result.extend(evaluate(*block, &mut new_state, debug_mode));
                        },
                        _ => { panic!("Invalid function call: {:?}", function); }
                    }
                }
                else {
                    panic!("Unknown function: {}", function);
                }
            },
            Expr::If(c, t) => {
                let condition = evaluate(vec![*c.clone()], state, debug_mode);
                let condition = &condition[0];

                match condition {
                    Expr::Number(n) => {
                        if *n == 1 {
                            result.extend(evaluate(*t.clone(), state, debug_mode));
                        }
                    },
                    _ => { panic!("Invalid condition in if statement"); }
                }
            },
            Expr::Else(c, t) => {
                let condition = evaluate(vec![*c.clone()], state, debug_mode);
                let condition = &condition[0];

                match condition {
                    Expr::Number(n) => {
                        if *n == 0 {
                            result.extend(evaluate(*t.clone(), state, debug_mode));
                        }
                    },
                    _ => { panic!("Invalid condition in else statement"); }
                }
            },
            Expr::While(c, b) => {
                loop {
                    let condition = evaluate(vec![*c.clone()], state, debug_mode);
                    let condition = &condition[0];

                    match condition {
                        Expr::Number(n) => {
                            if *n == 1 {
                                // Run the block
                                result.extend(evaluate(*b.clone(), state, debug_mode));
                            } else {
                                break;
                            }
                        },
                        _ => { panic!("Invalid condition in while statement"); }
                    }
                }
            },
            Expr::Func(name, args, block) => {
                state.insert(name.to_string(), Expr::Func(name.to_string(), args.clone(), block.clone()));
            }
            _ => {}
        }

        i += 1;
    }

    return result;
}


use std::collections::HashMap;
use std::io::Read;

use ureq;

use crate::tokens::*;
use crate::tokenizer::tokenize;
use crate::parser::parse;

fn builtin_println(args: Vec<Expr>) -> Expr {
    for arg in args {
        match arg {
            Expr::String(s) => {
                print!("{}", s);
            },
            Expr::Number(n) => {
                print!("{}", n);
            },
            Expr::Array(arr) => {
                print!("[");
                let length = arr.len();
                let mut i = 0;
                for item in arr.iter() {
                    builtin_print(vec![item.clone()]);
                    if i < length - 1 {
                        print!(", ");
                    }
                    i += 1;
                }
                print!("]");
            }
            _ => { println!("{:?} ", arg); }
        }
    }

    println!();

    return Expr::Number(0);
}

fn builtin_print(args: Vec<Expr>) -> Expr {
    for arg in args {
        match arg {
            Expr::String(s) => {
                print!("{}", s);
            },
            Expr::Number(n) => {
                print!("{}", n);
            },
            Expr::Array(arr) => {
                print!("[");
                let length = arr.len();
                let mut i = 0;
                for item in arr.iter() {
                    builtin_print(vec![item.clone()]);
                    if i < length - 1 {
                        print!(", ");
                    }
                    i += 1;
                }
                print!("]");
            }
            _ => { println!("{:?} ", arg); }
        }
    }

    return Expr::Number(0);
}

fn access_property(expr: &Expr, prop: &str, state: &HashMap<String, Expr>) -> Expr {
    match expr {
        Expr::String(s) => {
            match prop {
                "length" => {
                    return Expr::Number(s.len() as i64);
                },
                "uppercase" => {
                    return Expr::String(s.to_uppercase());
                },
                "lowercase" => {
                    return Expr::String(s.to_lowercase());
                },
                "reverse" => {
                    return Expr::String(s.chars().rev().collect());
                },
                "trim" => {
                    return Expr::String(s.trim().to_string());
                },
                _ => { panic!("Type String has no property {:?}", prop); }
            }
        },
        Expr::Number(n) => {
            match prop {
                "to_string" => {
                    return Expr::String(n.to_string());
                },
                _ => { panic!("Type Number has no property {:?}", prop); }
            }
        },
        Expr::Array(arr) => {
            match prop {
                "length" => {
                    return Expr::Number(arr.len() as i64);
                },
                _ => {
                    if let Ok(index) = prop.parse::<usize>() {
                        if index < arr.len() {
                            return arr[index].clone();
                        } else {
                            panic!("Index out of bounds: {}", index);
                        }
                    } else {
                        panic!("Type Array has no property {:?}", prop);
                    }
                }
            }
        },
        Expr::Identifier(name) => {
            if state.contains_key(name) {
                let value = state[name].clone();
                return access_property(&value, prop, state);
            } else {
                panic!("Variable not found: {}", name);
            }
        },
        Expr::Module(expr) => {
            for item in expr.iter() {
                match item {
                    Expr::Func(name, args, block) => {
                        if name == prop {
                            return Expr::Func(name.to_string(), args.clone(), block.clone());
                        }
                    },
                    _ => { }
                }
            }

            panic!("Module has no property: {}", prop);
        },
        _ => { panic!("{:?} has no property {:?}", expr, prop); }
    }
}

fn builtin_http_get(url: &str) -> Expr {
    let response = ureq::get(url).call();
    
    match response {
        Ok(response) => {
            let status = response.status();
            let text = response.into_string().unwrap();

            return Expr::Array(Box::new(vec![
                Expr::Number(status as i64),
                Expr::String(text)
            ]));
        },
        Err(e) => {
            return Expr::Array(Box::new(vec![
                Expr::Number(0),
                Expr::String(e.to_string())
            ]));
        }
    }
}

pub fn evaluate(expr: &mut Vec<Expr>, state: &mut HashMap<String, Expr>, debug_mode: bool) -> Vec<Expr> {
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
                        evaluate(&mut vec![*left.clone()], state, debug_mode)
                    }
                };

                let right = evaluate(&mut vec![*right.clone()], state, debug_mode);

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
                    (Expr::String(l), Expr::String(r)) => {
                        result.push(Expr::String(format!("{}{}", l, r)));
                        did_operate = true;
                    },
                    (Expr::String(l), Expr::Number(r)) => {
                        match *op {
                            Operator::Multiply => {
                                result.push(Expr::String(l.repeat(*r as usize)));
                                did_operate = true;
                            },
                            _ => { panic!("Invalid operator for string and number: {:?}", op); }
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
                    (Expr::Identifier(l), Expr::Array(arr)) => {
                        match *op {
                            Operator::Declare => {
                                if state.contains_key(&format!("{}", l)) {
                                    panic!("Cannot redeclare variable: {}", l);
                                }

                                state.insert(format!("{}", l), Expr::Array(arr.clone()));
                                did_operate = true;
                            },
                            Operator::Assign => {
                                if !state.contains_key(&format!("{}", l)) {
                                    panic!("Cannot assign to undeclared variable: {}", l);
                                }

                                state.insert(format!("{}", l), Expr::Array(arr.clone()));
                                did_operate = true;
                            }
                            _ => { panic!("Invalid operator on variable and array: {:?}", op); }
                        }
                    }
                    (Expr::Identifier(l), Expr::Func(name, args, block)) => {
                        match *op {
                            Operator::Declare => {
                                if state.contains_key(&format!("{}", l)) {
                                    panic!("Cannot redeclare variable: {}", l);
                                }

                                state.insert(format!("{}", l), Expr::Func(name.to_string(), args.clone(), block.clone()));
                                did_operate = true;
                            },
                            Operator::Assign => {
                                if !state.contains_key(&format!("{}", l)) {
                                    panic!("Cannot assign to undeclared variable: {}", l);
                                }

                                state.insert(format!("{}", l), Expr::Func(name.to_string(), args.clone(), block.clone()));
                                did_operate = true;
                            }
                            _ => { panic!("Invalid operator on variable and function: {:?}", op); }
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
                    let value = state[s].clone();
                    match value {
                        Expr::Identifier(value) => {
                            if state.contains_key(&value) {
                                result.push(state[&value].clone());
                            } else {
                                result.push(Expr::Identifier(value));
                            }
                        },
                        _ => {
                            result.push(value);
                        }
                    }
                } else {
                    if debug_mode {
                        println!("Variable not found: {}", s);
                    }
                    result.push(Expr::Identifier(s.to_string()));
                }
            },
            Expr::Call(function, args) => {
                let mut evaluated_args: Vec<Expr> = Vec::new();
                for arg in args.iter() {
                    let value = evaluate(&mut vec![arg.clone()], state, debug_mode);
                    evaluated_args.push(value[0].clone());
                }

                let function_name = match *function.clone() {
                    Expr::Identifier(name) => name.clone(),
                    Expr::String(name) => name.clone(),
                    Expr::AccessProperty(expr, prop) => {
                        let value = access_property(&expr, prop.as_str(), state);
                        match value {
                            Expr::Identifier(name) => name.clone(),
                            Expr::Func(ref name, ref _args, ref _block) => {
                                state.insert(name.clone(), value.clone());
                                name.clone()
                            },
                            _ => { panic!("Invalid function name"); }
                        }
                    },
                    _ => { panic!("Invalid function name"); }
                };

                if function_name == "print" {
                    builtin_print(evaluated_args);
                }
                else if function_name == "println" {
                    builtin_println(evaluated_args);
                }
                else if function_name == "http_get" {
                    let url = match &evaluated_args[0] {
                        Expr::String(s) => s,
                        _ => { panic!("Invalid argument for http_get"); }
                    };

                    result.push(builtin_http_get(url));
                }
                else if state.contains_key(&function_name) {
                    let function = state[&function_name].clone();
                    match function {
                        Expr::Func(_name, arg_names, mut block) => {
                            let mut new_state = state.clone();
                            for i in 0..args.len() {
                                let name = match &arg_names[i] {
                                    Expr::Identifier(n) => n,
                                    _ => { panic!("Invalid argument name"); }
                                };

                                new_state.insert(name.to_string(), args[i].clone());
                            }
                            result.extend(evaluate(&mut block, &mut new_state, debug_mode));
                        },
                        _ => { panic!("Invalid function call: {:?}", function); }
                    }
                }
                else {
                    panic!("Unknown function: {}", function_name);
                }
            },
            Expr::If(c, t) => {
                let condition = evaluate(&mut vec![*c.clone()], state, debug_mode);
                let condition = &condition[0];

                match condition {
                    Expr::Number(n) => {
                        if *n == 1 {
                            result.extend(evaluate(&mut t.clone(), state, debug_mode));
                        }
                    },
                    _ => { panic!("Invalid condition in if statement"); }
                }
            },
            Expr::Else(c, t) => {
                let condition = evaluate(&mut vec![*c.clone()], state, debug_mode);
                let condition = &condition[0];

                match condition {
                    Expr::Number(n) => {
                        if *n == 0 {
                            result.extend(evaluate(&mut t.clone(), state, debug_mode));
                        }
                    },
                    _ => { panic!("Invalid condition in else statement"); }
                }
            },
            Expr::While(c, b) => {
                loop {
                    let condition = evaluate(&mut vec![*c.clone()], state, debug_mode);
                    let condition = &condition[0];

                    match condition {
                        Expr::Number(n) => {
                            if *n == 1 {
                                // Run the block
                                result.extend(evaluate(&mut *b.clone(), state, debug_mode));
                            } else {
                                break;
                            }
                        },
                        _ => { panic!("Invalid condition in while statement"); }
                    }
                }
            },
            Expr::For(item, array, block) => {
                let array = evaluate(&mut vec![*array.clone()], state, debug_mode);
                let array = &array[0];

                match array {
                    Expr::Array(arr) => {
                        for value in arr.iter() {
                            state.insert(item.to_string(), value.clone());
                            result.extend(evaluate(&mut *block.clone(), state, debug_mode));
                            state.remove(item);
                        }
                    },
                    _ => { panic!("For loops can only iterate over arrays"); }
                }

            },
            Expr::Func(name, args, block) => {
                state.insert(name.to_string(), Expr::Func(name.to_string(), args.clone(), block.clone()));
            }
            Expr::Import(name) => {
                let filename = format!("{}.myst", name);
                let mut new_state = state.clone();

                let mut file = std::fs::File::open(filename).unwrap();
                let mut contents = String::new();

                file.read_to_string(&mut contents).unwrap();

                let new_tokens = tokenize(contents, debug_mode);
                let mut new_expr = parse(new_tokens, debug_mode);

                state.insert(name.to_string(), Expr::Module(Box::new(new_expr)));
            }
            Expr::Include(name) => {
                let filename = format!("{}.myst", name);
                let mut new_state = state.clone();

                let mut file = std::fs::File::open(filename).unwrap();
                let mut contents = String::new();

                file.read_to_string(&mut contents).unwrap();

                let new_tokens = tokenize(contents, debug_mode);
                let mut new_expr = parse(new_tokens, debug_mode);

                let remaining_expr = expr.split_off(i + 1);

                new_expr.extend(remaining_expr);

                evaluate(&mut new_expr, &mut new_state, debug_mode);
            }
            Expr::Array(array) => {
                let mut new_array: Vec<Expr> = Vec::new();

                for item in array.iter() {
                    let value = evaluate(&mut vec![item.clone()], state, debug_mode);
                    new_array.push(value[0].clone());
                }

                result.push(Expr::Array(Box::new(new_array)));
            }
            Expr::AccessProperty(expr, prop) => {
                let value = evaluate(&mut vec![*expr.clone()], state, debug_mode);
                result.push(access_property(&value[0], prop, &state));
            }
            _ => {
                println!("Unhandled expression: {:?}", expr[i]);
            }
        }

        i += 1;
    }

    return result;
}


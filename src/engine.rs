use ::rainbow_wrapper::rainbow_wrapper::wrapper::Wrapper;
use ::rainbow_wrapper::rainbow_wrapper::types::*;
use ::rainbow_wrapper::*;

use crate::enums::*;

pub fn eval(ast: Vec<Expr>, wrapper: &mut Wrapper) {
    let mut i = 0;
    while i < ast.len() {
        match &ast[i] {
            Expr::BinOp(op, left, right) => {
                match op {
                    Operator::Add => {
                        match (*left.clone(), *right.clone()) {
                            (Expr::Number(l), Expr::Number(r)) => {
                                let addition = add!(
                                    immediate!(SIGNED(l)),
                                    immediate!(SIGNED(r)),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(addition);
                            },
                            (Expr::Identifier(l), Expr::Number(r)) => {
                                let addition = add!(
                                    ident!(l.clone()),
                                    immediate!(SIGNED(r)),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(addition);
                            },
                            (Expr::Identifier(l), Expr::Identifier(r)) => {
                                let addition = add!(
                                    ident!(l.clone()),
                                    ident!(r.clone()),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(addition);
                            },
                            (Expr::BinOp(_, _, _), Expr::Number(r)) => {
                                eval(vec![*left.clone()], wrapper); // Saves left side BinOp result into `temp`

                                let addition = add!(
                                    ident!("temp".to_string()),
                                    immediate!(SIGNED(r)),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(addition);
                            },

                            _ => todo!("Got {:?} and {:?}", left, right)
                        }
                    },
                    Operator::Subtract => {
                        match (*left.clone(), *right.clone()) {
                            (Expr::Number(l), Expr::Number(r)) => {
                                let subtraction = sub!(
                                    immediate!(SIGNED(l)),
                                    immediate!(SIGNED(r)),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(subtraction);
                            },
                            (Expr::Identifier(l), Expr::Number(r)) => {
                                let subtraction= sub!(
                                    ident!(l.clone()),
                                    immediate!(SIGNED(r)),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(subtraction);
                            },
                            (Expr::Identifier(l), Expr::Identifier(r)) => {
                                let subtraction= sub!(
                                    ident!(l.clone()),
                                    ident!(r.clone()),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(subtraction);
                            },
                            (Expr::BinOp(_, _, _), Expr::Number(r)) => {
                                eval(vec![*left.clone()], wrapper); // Saves left side BinOp result into `temp`

                                let subtraction= sub!(
                                    ident!("temp".to_string()),
                                    immediate!(SIGNED(r)),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(subtraction);
                            },

                            _ => todo!("Got {:?} and {:?}", left, right)
                        }
                    },
                    Operator::Multiply => {
                        match (*left.clone(), *right.clone()) {
                            (Expr::Number(l), Expr::Number(r)) => {
                                let bytes = mul!(
                                    immediate!(SIGNED(l)),
                                    immediate!(SIGNED(r)),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(bytes);
                            },
                            (Expr::Identifier(l), Expr::Number(r)) => {
                                let bytes = mul!(
                                    ident!(l.clone()),
                                    immediate!(SIGNED(r)),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(bytes);
                            },
                            (Expr::Identifier(l), Expr::Identifier(r)) => {
                                let bytes = mul!(
                                    ident!(l.clone()),
                                    ident!(r.clone()),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(bytes);
                            },
                            (Expr::BinOp(_, _, _), Expr::Number(r)) => {
                                eval(vec![*left.clone()], wrapper); // Saves left side BinOp result into `temp`

                                let bytes = mul!(
                                    ident!("temp".to_string()),
                                    immediate!(SIGNED(r)),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(bytes);
                            },

                            _ => todo!("Got {:?} and {:?}", left, right)
                        }
                    }
                    Operator::Divide => {
                        match (*left.clone(), *right.clone()) {
                            (Expr::Number(l), Expr::Number(r)) => {
                                let bytes = div!(
                                    immediate!(SIGNED(l)),
                                    immediate!(SIGNED(r)),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(bytes);
                            },
                            (Expr::Identifier(l), Expr::Number(r)) => {
                                let bytes = div!(
                                    ident!(l.clone()),
                                    immediate!(SIGNED(r)),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(bytes);
                            },
                            (Expr::Identifier(l), Expr::Identifier(r)) => {
                                let bytes = div!(
                                    ident!(l.clone()),
                                    ident!(r.clone()),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(bytes);
                            },
                            (Expr::BinOp(_, _, _), Expr::Number(r)) => {
                                eval(vec![*left.clone()], wrapper); // Saves left side BinOp result into `temp`

                                let bytes = div!(
                                    ident!("temp".to_string()),
                                    immediate!(SIGNED(r)),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(bytes);
                            },

                            _ => todo!("Got {:?} and {:?}", left, right)
                        }
                    }
                    Operator::Declare => {
                        let left = match *left.clone() {
                            Expr::Identifier(name) => name.clone(),
                            _ => panic!("wtf")
                        };

                        match *right.clone() {
                            Expr::Number(n) => {
                                let declaration = var!(
                                    rbtype!(I64),
                                    name!(left.clone())
                                );

                                wrapper.push(declaration);

                                let assign = mov!(
                                    immediate!(SIGNED(n)),
                                    ident!(left)
                                );

                                wrapper.push(assign);
                            },
                            Expr::BinOp(_, _, _) => {
                                eval(vec![*right.clone()], wrapper);

                                let declaration = var!(
                                    rbtype!(I64),
                                    name!(left.clone())
                                );

                                wrapper.push(declaration);

                                let assign = mov!(
                                    ident!("temp".to_string()),
                                    ident!(left)
                                );

                                wrapper.push(assign);

                            }
                            _ => todo!()
                        }
                    },
                    Operator::Assign => {
                        let left = match *left.clone() {
                            Expr::Identifier(name) => name.clone(),
                            _ => panic!("wtf")
                        };

                        match *right.clone() {
                            Expr::Number(n) => {
                                let assign = mov!(
                                    immediate!(SIGNED(n)),
                                    ident!(left)
                                );

                                wrapper.push(assign);
                            },
                            _ => todo!()
                        }
                    }
                    _ => todo!()
                }
            },

            Expr::Return(val) => {
                let val = match *val.clone() {
                    Expr::Identifier(name) => name,
                    Expr::BinOp(_, _, _) => {
                        eval(vec![*val.clone()], wrapper);
                        "temp".to_string() // LLVM save me
                    },
                    _ => todo!()
                };

                let return_bytes = ret!(
                    ident!(val.clone())
                );

                wrapper.push(return_bytes);
                
            }
            _ => {
                println!("Ignoring instruction: {:?}", ast[i]);
            }
        }

        i+=1;
    }
}

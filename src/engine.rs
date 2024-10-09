use ::rainbow_wrapper::rainbow_wrapper::functions::{generate_function, Arg};
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
                            (Expr::CallFunc(_, _), Expr::Number(r)) => {
                                eval(vec![*left.clone()], wrapper);

                                let addition = add!(
                                    ident!("temp".to_string()),
                                    immediate!(SIGNED(r)),
                                    ident!("temp".to_string())
                                );

                                wrapper.push(addition);
                            }
                            (Expr::ArrayAccess(_, _), Expr::ArrayAccess(_, _)) => {
                                eval(vec![*left.clone()], wrapper);
                                
                                let bytes = mov!(ident!("temp".to_string()), ident!("temp2".to_string()));
                                wrapper.push(bytes);

                                eval(vec![*right.clone()], wrapper);

                                let bytes = add!(
                                    ident!("temp2".to_string()),
                                    ident!("temp".to_string()),
                                    ident!("temp".to_string())
                                );
                                wrapper.push(bytes);
                            }

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
                            Expr::CallFunc(_, _) => {
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
                            Expr::Array(items) => {
                                let declaration = var!(
                                    rbtype!(POINTER, I64),
                                    name!(left.clone())
                                );
                                wrapper.push(declaration);

                                let bytes = alloc!(
                                    rbtype!(I64),
                                    immediate!(SIGNED(items.len() as i64)),
                                    ident!(left.clone())
                                );
                                wrapper.push(bytes);

                                let mut i = 0;
                                while i < items.len() {
                                    let res = match &items[i] {
                                        Expr::Identifier(name) => name.clone(),
                                        Expr::BinOp(_, _, _) => {
                                            eval(vec![items[i].clone()], wrapper);
                                            "temp".to_string() // LLVM save me
                                        },
                                        Expr::CallFunc(_, _) => {
                                            eval(vec![items[i].clone()], wrapper);
                                            "temp".to_string()
                                        },
                                        Expr::Number(n) => {
                                            let bytes = mov!(immediate!(SIGNED(*n)), ident!("temp".to_string()));
                                            wrapper.push(bytes);
                                            "temp".to_string()
                                        }
                                        _ => todo!()
                                    };

                                    let bytes = pmov!(
                                        ident!(res.clone()),
                                        ident!(left.clone()),
                                        immediate!(SIGNED(i as i64))
                                    );
                                    wrapper.push(bytes);

                                    i += 1;
                                }
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

            Expr::ArrayAccess(name, index) => {
                eval(vec![*index.clone()], wrapper);

                let pointer = add!(
                    ident!(name.to_string()),
                    ident!("temp".to_string()),
                    ident!("temp".to_string())
                );
                wrapper.push(pointer);

                let bytes = self::deref!(ident!("temp".to_string()), ident!("temp".to_string()));
                wrapper.push(bytes);
            }

            Expr::DecFunc(name, args, body) => {
                let mut rb_args = vec![];
                let mut i = 0;
                while i < args.len() {
                    rb_args.push(Arg {
                        name: args[i].clone(),
                        typ: vec![Type::I64]
                    });

                    i += 1;
                }

                let mut func_wrapper = Wrapper::new();
                let create_var_bytes = var!(
                    Value::TYPE(vec![Type::I64]),
                    Value::NAME("temp".to_string())
                );
                func_wrapper.push(create_var_bytes);

                eval(*body.clone(), &mut func_wrapper);
                let function_bytes = func_wrapper.bytes.clone();

                wrapper.merge_data(&func_wrapper); // Note: we do this because strings an sheeit are not
                // stored in `bytes`, but are a seperate thing. @gromton12 please fix this.

                let bytes = generate_function(name, &rb_args, &vec![Type::I64], &function_bytes);

                wrapper.push(bytes);
            }

            Expr::CallFunc(name, args) => {
                let name = match *name.clone() {
                    Expr::Identifier(name) => name.clone(),
                    _ => panic!("Execpectedeeeted identified, got {:?}", name)
                };

                // Evaluate the arguments
                let mut i = 0;
                while i < args.len() {
                    eval(vec![args[i].clone()], wrapper);

                    let bytes = push!(ident!("temp".to_string()));
                    wrapper.push(bytes);

                    i += 1;
                }

                let bytes = call!(name!(name));
                wrapper.push(bytes);

                let bytes = pop!(ident!("temp".to_string()));
                wrapper.push(bytes)
            }

            Expr::Number(n) => {
                let bytes = mov!(immediate!(SIGNED(*n)), ident!("temp".to_string()));
                wrapper.push(bytes);
            }

            Expr::Identifier(name) => {
                let bytes = mov!(ident!(name.to_string()), ident!("temp".to_string()));
                wrapper.push(bytes);
            }

            Expr::Return(val) => {
                eval(vec![*val.clone()], wrapper);

                let return_bytes = ret!(
                    ident!("temp".to_string())
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

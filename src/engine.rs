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
                /* Number, Identifier, BinOp, CallFunc, ArrayAccess
                *
                * Because I am bad at rust, I'm gonna binary system this match statement.
                * Mad? I don't care.
                *
                * number - number
                * number - identifier
                * number - binop
                * number - callfunc
                * number - arrayaccess
                *
                * identifier - number
                * identifier - identifier
                * identifier - binop
                * identifier - callfunc
                * identifier - arrayaccess
                *
                * binop - number
                * binop - identifier
                * binop - binop
                * binop - callfunc
                * binop - arrayaccess
                *
                * callfunc - number
                * callfunc - identifier
                * callfunc - binop
                * callfunc - callfunc
                * callfunc - arrayaccess
                *
                * arrayaccess - number
                * arrayaccess - identifier
                * arrayaccess - binop
                * arrayaccess - callfunc
                * arrayaccess - arrayaccess
                *
                */
                match (*left.clone(), *right.clone()) {
                    (Expr::Number(l), Expr::Number(r)) => {
                        let bytes = match op {
                            Operator::Add => add!(
                                immediate!(SIGNED(l)),
                                immediate!(SIGNED(r)),
                                ident!("temp".to_string())
                            ),
                            Operator::Subtract => sub!(
                                immediate!(SIGNED(l)),
                                immediate!(SIGNED(r)),
                                ident!("temp".to_string())
                            ),
                            Operator::Multiply => mul!(
                                immediate!(SIGNED(l)),
                                immediate!(SIGNED(r)),
                                ident!("temp".to_string())
                            ),
                            Operator::Divide => div!(
                                immediate!(SIGNED(l)),
                                immediate!(SIGNED(r)),
                                ident!("temp".to_string())
                            ),
                            Operator::Assign => {
                                mov!(
                                    immediate!(SIGNED(r)),
                                    ident!(l.to_string())
                                )
                            }
                            Operator::Declare => {
                                let bytes = var!(
                                    Value::TYPE(vec![Type::I64]),
                                    Value::NAME(l.to_string())
                                );
                                wrapper.push(bytes);

                                mov!(
                                    immediate!(SIGNED(r)),
                                    ident!(l.to_string())
                                )

                            }
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::Number(l), Expr::Identifier(r)) => {
                        let bytes = match op {
                            Operator::Add => add!(
                                immediate!(SIGNED(l)),
                                ident!(r.clone()),
                                ident!("temp".to_string())
                            ),
                            Operator::Subtract => sub!(
                                immediate!(SIGNED(l)),
                                ident!(r.clone()),
                                ident!("temp".to_string())
                            ),
                            Operator::Multiply => mul!(
                                immediate!(SIGNED(l)),
                                ident!(r.clone()),
                                ident!("temp".to_string())
                            ),
                            Operator::Divide => div!(
                                immediate!(SIGNED(l)),
                                ident!(r.clone()),
                                ident!("temp".to_string())
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::Number(l), _) => {
                        eval(vec![*right.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                immediate!(SIGNED(l)),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Subtract => sub!(
                                immediate!(SIGNED(l)),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Multiply => mul!(
                                immediate!(SIGNED(l)),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Divide => div!(
                                immediate!(SIGNED(l)),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::Identifier(l), Expr::Number(r)) => {
                        let bytes = match op {
                            Operator::Add => add!(
                                ident!(l.clone()),
                                immediate!(SIGNED(r)),
                                ident!("temp".to_string())
                            ),
                            Operator::Subtract => sub!(
                                ident!(l.clone()),
                                immediate!(SIGNED(r)),
                                ident!("temp".to_string())
                            ),
                            Operator::Multiply => mul!(
                                ident!(l.clone()),
                                immediate!(SIGNED(r)),
                                ident!("temp".to_string())
                            ),
                            Operator::Divide => div!(
                                ident!(l.clone()),
                                immediate!(SIGNED(r)),
                                ident!("temp".to_string())
                            ),
                            Operator::Assign => {
                                mov!(
                                    immediate!(SIGNED(r)),
                                    ident!(l.to_string())
                                )
                            }
                            Operator::Declare => {
                                let bytes = var!(
                                    Value::TYPE(vec![Type::I64]),
                                    Value::NAME(l.to_string())
                                );
                                wrapper.push(bytes);

                                mov!(
                                    immediate!(SIGNED(r)),
                                    ident!(l.to_string())
                                )

                            }
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::Identifier(l), Expr::Identifier(r)) => {
                        let bytes = match op {
                            Operator::Add => add!(
                                ident!(l.clone()),
                                ident!(r.clone()),
                                ident!("temp".to_string())
                            ),
                            Operator::Subtract => sub!(
                                ident!(l.clone()),
                                ident!(r.clone()),
                                ident!("temp".to_string())
                            ),
                            Operator::Multiply => mul!(
                                ident!(l.clone()),
                                ident!(r.clone()),
                                ident!("temp".to_string())
                            ),
                            Operator::Divide => div!(
                                ident!(l.clone()),
                                ident!(r.clone()),
                                ident!("temp".to_string())
                            ),
                            Operator::Assign => {
                                mov!(
                                    ident!(r.clone()),
                                    ident!(l.clone())
                                )
                            }
                            Operator::Declare => {
                                let bytes = var!(
                                    Value::TYPE(vec![Type::I64]),
                                    Value::NAME(l.to_string())
                                );
                                wrapper.push(bytes);

                                mov!(
                                    ident!(r.clone()),
                                    ident!(l.clone())
                                )
                            }
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::Identifier(l), _) => {
                        eval(vec![*right.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!(l.clone()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Subtract => sub!(
                                ident!(l.clone()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Multiply => mul!(
                                ident!(l.clone()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Divide => div!(
                                ident!(l.clone()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Assign => {
                                mov!(
                                    ident!("temp".to_string()),
                                    ident!(l.clone())
                                )
                            }
                            Operator::Declare => {
                                let bytes = var!(
                                    Value::TYPE(vec![Type::I64]),
                                    Value::NAME(l.to_string())
                                );
                                wrapper.push(bytes);

                                mov!(
                                    ident!("temp".to_string()),
                                    ident!(l.clone())
                                )
                            }
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::BinOp(_, _, _), Expr::Number(r)) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp".to_string()),
                                immediate!(SIGNED(r)),
                                ident!("temp".to_string())
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp".to_string()),
                                immediate!(SIGNED(r)),
                                ident!("temp".to_string())
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp".to_string()),
                                immediate!(SIGNED(r)),
                                ident!("temp".to_string())
                            ),
                            Operator::Divide => div!(
                                ident!("temp".to_string()),
                                immediate!(SIGNED(r)),
                                ident!("temp".to_string())
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::BinOp(_, _, _), Expr::Identifier(r)) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp".to_string()),
                                ident!(r.clone()),
                                ident!("temp".to_string())
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp".to_string()),
                                ident!(r.clone()),
                                ident!("temp".to_string())
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp".to_string()),
                                ident!(r.clone()),
                                ident!("temp".to_string())
                            ),
                            Operator::Divide => div!(
                                ident!("temp".to_string()),
                                ident!(r.clone()),
                                ident!("temp".to_string())
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::BinOp(_, _, _), _) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = mov!(ident!("temp".to_string()), ident!("temp2".to_string()));
                        wrapper.push(bytes);

                        eval(vec![*right.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Divide => div!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::CallFunc(_, _), Expr::Number(r)) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp".to_string()),
                                immediate!(SIGNED(r)),
                                ident!("temp".to_string())
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp".to_string()),
                                immediate!(SIGNED(r)),
                                ident!("temp".to_string())
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp".to_string()),
                                immediate!(SIGNED(r)),
                                ident!("temp".to_string())
                            ),
                            Operator::Divide => div!(
                                ident!("temp".to_string()),
                                immediate!(SIGNED(r)),
                                ident!("temp".to_string())
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::CallFunc(_, _), Expr::Identifier(r)) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp".to_string()),
                                ident!(r.clone()),
                                ident!("temp".to_string())
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp".to_string()),
                                ident!(r.clone()),
                                ident!("temp".to_string())
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp".to_string()),
                                ident!(r.clone()),
                                ident!("temp".to_string())
                            ),
                            Operator::Divide => div!(
                                ident!("temp".to_string()),
                                ident!(r.clone()),
                                ident!("temp".to_string())
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::CallFunc(_, _), _) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = mov!(ident!("temp".to_string()), ident!("temp2".to_string()));
                        wrapper.push(bytes);

                        eval(vec![*right.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Divide => div!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::ArrayAccess(_, _), Expr::Number(r)) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = mov!(ident!("temp".to_string()), ident!("temp2".to_string()));
                        wrapper.push(bytes);

                        eval(vec![*right.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Divide => div!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::ArrayAccess(_, _), Expr::Identifier(r)) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = mov!(ident!("temp".to_string()), ident!("temp2".to_string()));
                        wrapper.push(bytes);

                        eval(vec![*right.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Divide => div!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::ArrayAccess(_, _), _) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = mov!(ident!("temp".to_string()), ident!("temp2".to_string()));
                        wrapper.push(bytes);

                        eval(vec![*right.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            Operator::Divide => div!(
                                ident!("temp2".to_string()),
                                ident!("temp".to_string()),
                                ident!("temp".to_string())
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    _ => {
                        todo!("Unhandled binary operation: {:#?}\n{:#?}\n{:#?}", op, left, right);
                    }
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

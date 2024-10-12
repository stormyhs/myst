use rainbow_wrapper::functions::{generate_function, generate_scope, Arg};
use rainbow_wrapper::wrapper::Wrapper;
use rainbow_wrapper::types::*;
use rainbow_wrapper::*;

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
                                ident!("temp")
                            ),
                            Operator::Subtract => sub!(
                                immediate!(SIGNED(l)),
                                immediate!(SIGNED(r)),
                                ident!("temp")
                            ),
                            Operator::Multiply => mul!(
                                immediate!(SIGNED(l)),
                                immediate!(SIGNED(r)),
                                ident!("temp")
                            ),
                            Operator::Divide => div!(
                                immediate!(SIGNED(l)),
                                immediate!(SIGNED(r)),
                                ident!("temp")
                            ),
                            Operator::Assign => {
                                mov!(
                                    immediate!(SIGNED(r)),
                                    ident!(l)
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
                                    ident!(l)
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
                                ident!("temp")
                            ),
                            Operator::Subtract => sub!(
                                immediate!(SIGNED(l)),
                                ident!(r.clone()),
                                ident!("temp")
                            ),
                            Operator::Multiply => mul!(
                                immediate!(SIGNED(l)),
                                ident!(r.clone()),
                                ident!("temp")
                            ),
                            Operator::Divide => div!(
                                immediate!(SIGNED(l)),
                                ident!(r.clone()),
                                ident!("temp")
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
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Subtract => sub!(
                                immediate!(SIGNED(l)),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Multiply => mul!(
                                immediate!(SIGNED(l)),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Divide => div!(
                                immediate!(SIGNED(l)),
                                ident!("temp"),
                                ident!("temp")
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
                                ident!("temp")
                            ),
                            Operator::Subtract => sub!(
                                ident!(l.clone()),
                                immediate!(SIGNED(r)),
                                ident!("temp")
                            ),
                            Operator::Multiply => mul!(
                                ident!(l.clone()),
                                immediate!(SIGNED(r)),
                                ident!("temp")
                            ),
                            Operator::Divide => div!(
                                ident!(l.clone()),
                                immediate!(SIGNED(r)),
                                ident!("temp")
                            ),
                            Operator::Assign => {
                                mov!(
                                    immediate!(SIGNED(r)),
                                    ident!(l)
                                )
                            }
                            Operator::Declare => {
                                let bytes = var!(
                                    Value::TYPE(vec![Type::I64]),
                                    Value::NAME(l.clone())
                                );
                                wrapper.push(bytes);

                                mov!(
                                    immediate!(SIGNED(r)),
                                    ident!(l)
                                )

                            }
                            Operator::Lesser => {
                                cmp!(
                                    cond!(<),
                                    ident!(l.clone()),
                                    immediate!(SIGNED(r)),
                                    ident!("temp2")
                                )
                            }
                            _ => todo!("Unhandled binary operation: {:#?}\n{:#?}\n{:#?}", op, left, right)
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::Identifier(l), Expr::Identifier(r)) => {
                        let bytes = match op {
                            Operator::Add => add!(
                                ident!(l.clone()),
                                ident!(r.clone()),
                                ident!("temp")
                            ),
                            Operator::Subtract => sub!(
                                ident!(l.clone()),
                                ident!(r.clone()),
                                ident!("temp")
                            ),
                            Operator::Multiply => mul!(
                                ident!(l.clone()),
                                ident!(r.clone()),
                                ident!("temp")
                            ),
                            Operator::Divide => div!(
                                ident!(l.clone()),
                                ident!(r.clone()),
                                ident!("temp")
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
                                    Value::NAME(l.clone())
                                );
                                wrapper.push(bytes);

                                mov!(
                                    ident!(r.clone()),
                                    ident!(l)
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
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Subtract => sub!(
                                ident!(l.clone()),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Multiply => mul!(
                                ident!(l.clone()),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Divide => div!(
                                ident!(l.clone()),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Assign => {
                                mov!(
                                    ident!("temp"),
                                    ident!(l.clone())
                                )
                            }
                            Operator::Declare => {
                                let bytes = var!(
                                    Value::TYPE(vec![Type::I64]),
                                    Value::NAME(l.clone())
                                );
                                wrapper.push(bytes);

                                mov!(
                                    ident!("temp"),
                                    ident!(l)
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
                                ident!("temp"),
                                immediate!(SIGNED(r)),
                                ident!("temp")
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp"),
                                immediate!(SIGNED(r)),
                                ident!("temp")
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp"),
                                immediate!(SIGNED(r)),
                                ident!("temp")
                            ),
                            Operator::Divide => div!(
                                ident!("temp"),
                                immediate!(SIGNED(r)),
                                ident!("temp")
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::BinOp(_, _, _), Expr::Identifier(r)) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp"),
                                ident!(r.clone()),
                                ident!("temp")
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp"),
                                ident!(r.clone()),
                                ident!("temp")
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp"),
                                ident!(r.clone()),
                                ident!("temp")
                            ),
                            Operator::Divide => div!(
                                ident!("temp"),
                                ident!(r.clone()),
                                ident!("temp")
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::BinOp(_, _, _), _) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = mov!(ident!("temp"), ident!("temp2".to_string()));
                        wrapper.push(bytes);

                        eval(vec![*right.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Divide => div!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::CallFunc(_, _), Expr::Number(r)) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp"),
                                immediate!(SIGNED(r)),
                                ident!("temp")
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp"),
                                immediate!(SIGNED(r)),
                                ident!("temp")
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp"),
                                immediate!(SIGNED(r)),
                                ident!("temp")
                            ),
                            Operator::Divide => div!(
                                ident!("temp"),
                                immediate!(SIGNED(r)),
                                ident!("temp")
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::CallFunc(_, _), Expr::Identifier(r)) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp"),
                                ident!(r.clone()),
                                ident!("temp")
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp"),
                                ident!(r.clone()),
                                ident!("temp")
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp"),
                                ident!(r.clone()),
                                ident!("temp")
                            ),
                            Operator::Divide => div!(
                                ident!("temp"),
                                ident!(r.clone()),
                                ident!("temp")
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::CallFunc(_, _), _) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = mov!(ident!("temp"), ident!("temp2".to_string()));
                        wrapper.push(bytes);

                        eval(vec![*right.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Divide => div!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::ArrayAccess(_, _), Expr::Number(r)) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = mov!(ident!("temp"), ident!("temp2".to_string()));
                        wrapper.push(bytes);

                        eval(vec![*right.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Divide => div!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::ArrayAccess(_, _), Expr::Identifier(r)) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = mov!(ident!("temp"), ident!("temp2".to_string()));
                        wrapper.push(bytes);

                        eval(vec![*right.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Divide => div!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            _ => todo!()
                        };

                        wrapper.push(bytes);
                    }
                    (Expr::ArrayAccess(_, _), _) => {
                        eval(vec![*left.clone()], wrapper);

                        let bytes = mov!(ident!("temp"), ident!("temp2".to_string()));
                        wrapper.push(bytes);

                        eval(vec![*right.clone()], wrapper);

                        let bytes = match op {
                            Operator::Add => add!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Subtract => sub!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Multiply => mul!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
                            ),
                            Operator::Divide => div!(
                                ident!("temp2"),
                                ident!("temp"),
                                ident!("temp")
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

            // NOTE: This is assuming declaration every time.
            Expr::Array(items) => {
                let bytes = alloc!(
                    rbtype!(I64),
                    immediate!(SIGNED(items.len() as i64)),
                    ident!("temp3")
                );
                wrapper.push(bytes);

                let mut i = 0;
                while i < items.len() {
                    match &items[i] {
                        Expr::Number(n) => {
                            let bytes = pmov!(
                                immediate!(SIGNED(*n)),
                                ident!("temp3"),
                                immediate!(SIGNED(i as i64))
                            );

                            wrapper.push(bytes);
                        },
                        Expr::Identifier(name) => {
                            let bytes = pmov!(
                                ident!(name.clone()),
                                ident!("temp3"),
                                immediate!(SIGNED(i as i64))
                            );

                            wrapper.push(bytes);
                        },
                        _ => {
                            eval(vec![items[i].clone()], wrapper);
                            
                            let bytes = pmov!(
                                ident!("temp"),
                                ident!("temp3"),
                                immediate!(SIGNED(i as i64))
                            );

                            wrapper.push(bytes);
                        }
                    }

                    i += 1;
                }

                let bytes = mov!(ident!("temp3"), ident!("temp"));
                wrapper.push(bytes);
            }

            Expr::ArrayAccess(name, index) => {
                eval(vec![*index.clone()], wrapper);

                let pointer = add!(
                    ident!(name),
                    ident!("temp"),
                    ident!("temp")
                );
                wrapper.push(pointer);

                let bytes = self::deref!(ident!("temp"), ident!("temp".to_string()));
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

                    let bytes = push!(ident!("temp"));
                    wrapper.push(bytes);

                    i += 1;
                }

                let bytes = call!(name!(name));
                wrapper.push(bytes);

                let bytes = pop!(ident!("temp"));
                wrapper.push(bytes)
            }

            Expr::Number(n) => {
                let bytes = mov!(immediate!(SIGNED(*n)), ident!("temp"));
                wrapper.push(bytes);
            }

            Expr::Identifier(name) => {
                let bytes = mov!(ident!(name), ident!("temp".to_string()));
                wrapper.push(bytes);
            }

            Expr::Include(name) => {
                // Note: `push_import` does not import a module, but rather just copy-pastes it C-style.
                // That is the reason I called it `include` and not `import`.
                wrapper.push_import(&format!("{}", name));
            }

            Expr::If(cond, body, else_body) => {
                eval(vec![*cond.clone()], wrapper);
                // `temp` is the condition

                // If `temp` is 0, jump to index `2`, which will be the `false` body, because this
                // jump will be inside a wrapper scope, which will only contain two scopes: `true` and `false`.
                let jump = jne!(
                    ident!("temp"),
                    immediate!(SIGNED(1)),
                    immediate!(SIGNED(2)) // what index to jump to
                );

                // Create a new scope, which stores the `true` body
                let mut body_wrapper = Wrapper::new();
                eval(*body.clone(), &mut body_wrapper);
                let true_scope = generate_scope(&body_wrapper.bytes);

                // After finishing the `true` body, jump to the end of the `if` statement
                // This it to prevent the `false` body from being executed right after the `true` body.
                let jump_after_true = jne!(
                    ident!("temp"),
                    immediate!(SIGNED(1)),
                    immediate!(SIGNED(3)) // what index to jump to
                    //immediate!(SIGNED(2)) // infinite loop test
                );

                // Create a new scope, which stores the `false` body
                let mut else_body_wrapper = Wrapper::new();
                eval(*else_body.clone(), &mut else_body_wrapper);
                let false_scope = generate_scope(&else_body_wrapper.bytes);

                let merged_scopes = [jump, true_scope, jump_after_true, false_scope].concat();
                let wrap = generate_scope(&merged_scopes);

                wrapper.push(wrap);
            }

            Expr::While(cond, body) => {
                let mut body_wrapper = Wrapper::new();
                eval(vec![*cond.clone()], &mut body_wrapper);
                // Evaluate the body of the loop
                eval(*body.clone(), &mut body_wrapper);

                // If the condition `temp` is `1` (true), repeat the loop
                let repeat = je!(
                    ident!("temp2"),
                    immediate!(SIGNED(1)),
                    immediate!(SIGNED(1))
                );
                body_wrapper.push(repeat.clone());

                // Put the entire thing in a scope, so jumping to index is `1` will jump to the start of the loop
                let body_scope = generate_scope(&body_wrapper.bytes);

                wrapper.push(body_scope);
            }

            Expr::Return(val) => {
                eval(vec![*val.clone()], wrapper);

                let return_bytes = ret!(
                    ident!("temp")
                );

                wrapper.push(return_bytes);
                
            }

            Expr::Pass => {
                wrapper.push(nop!());
            }

            _ => {
                println!("[Engine] Ignoring instruction: {:?}", ast[i]);
            }
        }

        i+=1;
    }
}

use rainbow_wrapper::generation::{generate_function, generate_scope, Arg};
use rainbow_wrapper::wrapper::Wrapper;
use rainbow_wrapper::types::*;
use rainbow_wrapper::*;

use crate::enums::*;

fn gen_cmp(op: Operator, left: Expr, right: Expr, wrapper: &mut Wrapper) -> Vec<u8> {
    match op {
        Operator::Declare => {
            let name = match left {
                Expr::Identifier(ref i) => i,
                _ => todo!()
            };

            wrapper.push(var!(
                Value::TYPE(vec![Type::I64]),
                Value::NAME(name.clone().to_string())
            ));

        }
        _ => {}
    }

    let left_macro = match left {
        Expr::Number(n) => immediate!(SIGNED(n)),
        Expr::Identifier(ref i) => ident!(i),
        _ => {
            eval(vec![left.clone()], wrapper);
            wrapper.push(mov!(ident!("temp"), ident!("temp2".to_string())));
            ident!("temp2")
        }
    };

    let right_macro = match right {
        Expr::Number(n) => immediate!(SIGNED(n)),
        Expr::Identifier(i) => ident!(i),
        _ => {
            eval(vec![right.clone()], wrapper);
            ident!("temp")
        }
    };

    match op {
        Operator::Add => add!(left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::Subtract => sub!(left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::Multiply => mul!(left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::Divide => div!(left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::Lesser => cmp!(cond!(<), left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::Greater => cmp!(cond!(>), left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::Equality => cmp!(cond!(==), left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::GreaterEqual => cmp!(cond!(>=), left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::LesserEqual => cmp!(cond!(<=), left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::NotEqual => cmp!(cond!(!=), left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::Assign => mov!(right_macro.clone(), left_macro.clone()),
        Operator::Declare => mov!(right_macro.clone(), left_macro.clone()),
    }
}

pub fn eval(ast: Vec<Expr>, wrapper: &mut Wrapper) {
    let mut i = 0;
    while i < ast.len() {
        match &ast[i] {
            Expr::BinOp(op, left, right) => {
                let bytes = gen_cmp(op.clone(), *left.clone(), *right.clone(), wrapper);
                wrapper.push(bytes);
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

                wrapper.merge_data(&func_wrapper); // Note: we do this because strings are not
                // stored in `bytes`, but are a seperate thing. @gromton12 please fix this.

                let bytes = generate_function(name, &rb_args, &vec![Type::I64], &function_bytes);

                wrapper.push(bytes);
            }

            Expr::CallFunc(name, args) => {
                let name = match *name.clone() {
                    Expr::Identifier(name) => name.clone(),
                    _ => panic!("Expected identifier, got {:?}", name)
                };

                // Evaluate the arguments
                let mut i = 0;
                while i < args.len() {
                    match &args[i] {
                        Expr::Number(n) => {
                            wrapper.push(push!(immediate!(SIGNED(*n))));
                        }
                        Expr::Identifier(name) => {
                            wrapper.push(push!(ident!(name.clone())));
                        }
                        Expr::String(s) => {
                            eval(vec![args[i].clone()], wrapper);
                        }
                        _ => {
                            // arg is stored in `temp`
                            eval(vec![args[i].clone()], wrapper);
                            let bytes = push!(ident!("temp"));
                            wrapper.push(bytes);
                        }
                    }

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

            Expr::String(s) => {
                wrapper.push_string(&s);
                wrapper.push(push!(ident!(s.clone())));

                wrapper.push(push!(immediate!(UNSIGNED(s.len()))));
            }

            Expr::Identifier(name) => {
                let bytes = mov!(ident!(name), ident!("temp".to_string()));
                wrapper.push(bytes);
            }

            Expr::Import(name) => {
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
                    immediate!(SIGNED(3)) // what index to jump to
                );

                // Create a new scope, which stores the `true` body
                let mut body_wrapper = Wrapper::new();
                eval(*body.clone(), &mut body_wrapper);
                let true_scope = generate_scope(&body_wrapper.bytes);

                // After finishing the `true` body, jump to the end of the `if` statement
                // This it to prevent the `false` body from being executed right after the `true` body.
                let jump_after_true = jmp!(
                    immediate!(SIGNED(4)) // what index to jump to
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

                // Evaluate the condition
                let mut cond_wrapper = Wrapper::new();
                eval(vec![*cond.clone()], &mut cond_wrapper);


                // If `temp` is 0, jump to the end of the `while` loop
                let jump = jne!(
                    ident!("temp"),
                    immediate!(SIGNED(1)),
                    immediate!(SIGNED(4)) // what index to jump to
                );

                // Evaluate the body
                eval(*body.clone(), &mut body_wrapper);

                // Jump back to the condition
                let jump_back = jmp!(
                    immediate!(SIGNED(0))
                );

                let merged_scopes = [generate_scope(&cond_wrapper.bytes), jump, generate_scope(&body_wrapper.bytes), jump_back].concat();
                let wrap = generate_scope(&merged_scopes);

                wrapper.push(wrap);
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

            Expr::PropertyAccess(item, prop) => {
                let item = match *item.clone() {
                    Expr::Identifier(name) => name.clone(),
                    _ => panic!("Expected identifier, got {:?}", item)
                };

                match *prop.clone() {
                    Expr::Identifier(name) => {
                        let full_name = format!("{}.{}", item, name);
                        let bytes = mov!(ident!(full_name), ident!("temp"));

                        wrapper.push(bytes);

                        i += 1;
                        continue;
                    }
                    Expr::CallFunc(_name, args) => {
                        let mut j = 0;
                        while j < args.len() {
                            match &args[j] {
                                Expr::Number(n) => {
                                    wrapper.push(push!(immediate!(SIGNED(*n))));
                                }
                                Expr::Identifier(name) => {
                                    wrapper.push(push!(ident!(name.clone())));
                                }
                                Expr::String(_s) => {
                                    eval(vec![args[j].clone()], wrapper);
                                }
                                _ => {
                                    // arg is stored in `temp`
                                    eval(vec![args[j].clone()], wrapper);
                                    let bytes = push!(ident!("temp"));
                                    wrapper.push(bytes);
                                }
                            }

                            j += 1;
                        }

                        let prop = match *prop.clone() {
                            Expr::Identifier(p) => p.clone(),
                            Expr::CallFunc(name, _) => {
                                println!("[ENGINE] CallFunc in PropertyAccess");
                                let name = match *name {
                                    Expr::Identifier(name) => name,
                                    _ => panic!("Expected identifier, got {:?}", name)
                                };

                                let bytes = call!(name!(format!("{}.{}", item, name)));
                                wrapper.push(bytes);

                                println!("[ENGINE] Pushed call for: {}.{}", item, name);

                                i += 1;
                                continue;
                            }
                            _ => panic!("Expected identifier, got {:?}", prop)
                        };

                        let bytes = call!(name!(format!("{}.{}", item, prop)));
                        wrapper.push(bytes);

                        i += 1;
                        continue;
                    },
                    _ => panic!("Expected Identifier or CallFunc, got {:?}", prop)
                };
            }

            _ => {
                println!("[Engine] Ignoring instruction: {:?}", ast[i]);
            }
        }

        i+=1;
    }
}

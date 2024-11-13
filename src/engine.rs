use rainbow_wrapper::generation::{generate_function, generate_scope, Arg};
use rainbow_wrapper::wrapper::Wrapper;
use rainbow_wrapper::types::*;
use rainbow_wrapper::*;

use std::collections::HashMap;

use crate::enums::*;

fn infer_type(expr: &Expr, state: &HashMap<String, String>) -> Type {
    match expr {
        Expr::Number(_) => Type::I64,
        Expr::String(_) => Type::STRUCT,
        Expr::Identifier(name) => {
            match state.get(name) {
                Some(t) => {
                    match t.as_str() {
                        "number" => Type::I64,
                        "string" => Type::STRUCT,
                        "struct" => Type::STRUCT,
                        "callback" => Type::NAME,
                        "null" => Type::VOID,
                        _ => Type::I64
                    }
                }
                None => {
                    panic!("Could not infer type of identifier: {}", name);
                }
            }
        }
        Expr::CallFunc(name, _) => {
            match *name.clone() {
                Expr::Identifier(name) => {
                    match state.get(&name) {
                        Some(t) => {
                            match t.as_str() {
                                "number" => Type::I64,
                                "string" => Type::STRUCT,
                                "struct" => Type::STRUCT,
                                "callback" => Type::NAME,
                                "null" => Type::VOID,
                                _ => Type::I64
                            }
                        }
                        None => {
                            panic!("Could not infer type of function: {}", name);
                        }
                    }
                }
                _ => {
                    panic!("Expected identifier, got {:?}", name);
                }
            }
        }
        Expr::Array(_) => Type::I64,
        _ => {
            panic!("Could not infer type of expression: {:?}", expr);
        }
    }
}

fn gen_cmp(op: Operator, left: Expr, right: Expr, wrapper: &mut Wrapper, state: &mut HashMap<String, String>) -> Vec<u8> {
    match op {
        Operator::Declare(ref typ) => {
            let name = match left {
                Expr::Identifier(ref i) => i,
                _ => todo!()
            };

            let typ = match typ {
                MType::Number => {
                    Value::TYPE(vec![Type::I64])
                }
                MType::Struct | MType::String => {
                    Value::TYPE(vec![Type::STRUCT])
                }
                MType::Undefined => {
                    let typ = infer_type(&right, state);
                    Value::TYPE(vec![typ])
                }
                _ => todo!("unhandled type: {:?}", typ)
            };

            wrapper.push(var!(
                typ.clone(),
                Value::NAME(name.clone().to_string())
            ));

        }
        _ => {}
    }

    let left_macro = match left {
        Expr::Number(n) => immediate!(SIGNED(n)),
        Expr::Identifier(ref i) => ident!(i),
        Expr::PropertyAccess(_, ref prop) => {
            match **prop {
                Expr::CallFunc(_, _) => {
                    eval(vec![left.clone()], wrapper, state);
                    ident!("temp_struct")
                }
                _ => {
                    eval(vec![left.clone()], wrapper, state);
                    ident!("temp2")
                }
            }
        }
        Expr::CallFunc(_, _) => {
            eval(vec![left.clone()], wrapper, state);
            wrapper.push(mov!(ident!("temp"), ident!("temp2".to_string())));
            ident!("temp2")
        }
        Expr::String(_) => {
            println!("[Engine] WARN! Left-side string comparison is not supported.");
            eval(vec![left.clone()], wrapper, state);
            wrapper.push(mov!(ident!("temp"), ident!("temp2".to_string())));
            ident!("temp2")
        }
        _ => {
            eval(vec![left.clone()], wrapper, state);
            wrapper.push(mov!(ident!("temp"), ident!("temp2".to_string())));
            ident!("temp2")
        }
    };

    let right_macro = match right {
        Expr::Number(n) => immediate!(SIGNED(n)),
        Expr::Identifier(ref i) => ident!(i),
        Expr::PropertyAccess(ref obj, ref prop) => {
            // NOTE: This means that every time a property is a call (such as `string.new()`), it
            // will attempt to store the result in a struct. This is because I am bad at rust.
            match **prop {
                Expr::CallFunc(ref name, _) => {
                    eval(vec![right.clone()], wrapper, state);
                    let name = match *name.clone() {
                        Expr::Identifier(name) => name.clone(),
                        _ => panic!("Expected identifier, got {:?}", name)
                    };
                    let obj_name = match *obj.clone() {
                        Expr::Identifier(name) => name.clone(),
                        _ => panic!("Expected identifier, got {:?}", obj)
                    };
                    let full_name = format!("{}.{}", obj_name, name);

                    let typ = match state.get(&full_name) {
                        Some(t) => t,
                        None => &MType::Undefined.stringify()
                    };
                    let bytes = match typ.as_str() {
                        "number" => {
                            ident!("temp")
                        }
                        "string" => {
                            ident!("temp_struct")
                        }
                        "struct" => {
                            ident!("temp_struct")
                        }
                        "callback" => {
                            panic!()
                        }
                        "null" => {
                            panic!()
                        }
                        _ => {
                            ident!("temp")
                        }
                    };
                    bytes
                }
                _ => {
                    eval(vec![right.clone()], wrapper, state);
                    ident!("temp")
                }
            }
        }
        Expr::CallFunc(ref name, _) => {
            eval(vec![right.clone()], wrapper, state);
            ident!("temp")
        }
        Expr::String(ref s) => {
            let new_expr = Expr::CallFunc(
                Box::new(
                    Expr::Identifier(
                        "string.new".to_string()
                    )
                ),
                Box::new(vec![Expr::String(s.to_string())])
            );
            eval(vec![new_expr], wrapper, state);
            ident!("temp_struct")
        }
        _ => {
            eval(vec![right.clone()], wrapper, state);
            ident!("temp")
        }
    };

    match op {
        Operator::Add => add!(left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::Subtract => sub!(left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::Multiply => mul!(left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::Divide => div!(left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::Lesser => cmp!(cond!(<), left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::Greater => {
            match right {
                Expr::CallFunc(_, _) => {
                    cmp!(cond!(>), left_macro.clone(), right_macro.clone(), ident!("temp"))
                }
                _ => {
                    cmp!(cond!(>), left_macro.clone(), right_macro.clone(), ident!("temp"))
                }
            }
        }
        Operator::Equality => cmp!(cond!(==), left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::GreaterEqual => cmp!(cond!(>=), left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::LesserEqual => cmp!(cond!(<=), left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::NotEqual => cmp!(cond!(!=), left_macro.clone(), right_macro.clone(), ident!("temp")),
        Operator::Assign => {
            mov!(right_macro.clone(), left_macro.clone())
        },
        Operator::Declare(typ) => {
            mov!(right_macro.clone(), left_macro.clone())
        }
    }
}

pub fn eval(ast: Vec<Expr>, wrapper: &mut Wrapper, state: &mut HashMap<String, String>) {
    let mut i = 0;
    while i < ast.len() {
        match &ast[i] {
            Expr::BinOp(op, left, right) => {
                let bytes = gen_cmp(op.clone(), *left.clone(), *right.clone(), wrapper, state);
                wrapper.push(bytes);
            },

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
                            eval(vec![items[i].clone()], wrapper, state);
                            
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
                eval(vec![*index.clone()], wrapper, state);

                let pointer = add!(
                    ident!(name),
                    ident!("temp"),
                    ident!("temp")
                );
                wrapper.push(pointer);

                let bytes = self::deref!(ident!("temp"), ident!("temp".to_string()));
                wrapper.push(bytes);
            }

            Expr::DecFunc(name, args, body, typ) => {
                let mut rb_args = vec![];
                let mut i = 0;
                while i < args.len() {
                    match &args[i] {
                        Expr::Parameter(name, typ) => {
                            let t = match typ {
                                MType::Number => Type::I64,
                                MType::String => Type::STRUCT,
                                MType::Struct => Type::STRUCT,
                                MType::Function => {
                                    state.insert(name.clone(), "callback".to_string());
                                    Type::NAME
                                },
                                MType::Nested(parent, child) => {
                                    let parent = match *parent.clone() {
                                        MType::Number => Type::I64,
                                        MType::String => Type::STRUCT,
                                        MType::Struct => Type::STRUCT,
                                        MType::Function => {
                                            state.insert(name.clone(), "callback".to_string());
                                            Type::NAME
                                        },
                                        _ => Type::I64
                                    };

                                    let child = match *child.clone() {
                                        MType::Number => {
                                            state.insert(name.clone(), "callback-number".to_string());
                                            Type::I64
                                        }
                                        MType::String => {
                                            state.insert(name.clone(), "callback-string".to_string());
                                            Type::STRUCT
                                        },
                                        MType::Struct => {
                                            state.insert(name.clone(), "callback-struct".to_string());
                                            Type::STRUCT
                                        },
                                        MType::Function => {
                                            state.insert(name.clone(), "callback-callback".to_string());
                                            Type::NAME
                                        },
                                        _ => {
                                            state.insert(name.clone(), "callback-number".to_string());
                                            Type::I64
                                        }
                                    };

                                    child
                                }
                                MType::Undefined => Type::VOID,
                                _ => Type::I64
                            };
                            rb_args.push(Arg {
                                name: name.to_string(),
                                typ: vec![t]
                            });
                        }
                        _ => {
                            panic!("Expected argument, got {:?}", args[i]);
                        }
                    }

                    i += 1;
                }

                let mut func_wrapper = Wrapper::new();
                let create_var_bytes = var!(
                    Value::TYPE(vec![Type::I64]),
                    Value::NAME("temp".to_string())
                );
                func_wrapper.push(create_var_bytes);

                eval(*body.clone(), &mut func_wrapper, state);
                let function_bytes = func_wrapper.bytes.clone();

                wrapper.merge_data(&func_wrapper); // Note: we do this because strings are not
                // stored in `bytes`, but are a seperate thing. @gromton12 please fix this.
               
                state.insert(name.clone(), typ.stringify());
                
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
                            wrapper.push(
                                push!(
                                    immediate!(
                                        NAME(name.clone())
                                    )
                                )
                            );
                        }
                        Expr::String(_s) => {
                            eval(vec![args[i].clone()], wrapper, state);
                        }
                        Expr::BinOp(_, _, _) => {
                            eval(vec![args[i].clone()], wrapper, state);
                            let bytes = push!(ident!("temp"));
                            wrapper.push(bytes);
                        }
                        Expr::DecFunc(name, _, _, typ) => {
                            eval(vec![args[i].clone()], wrapper, state);
                            let bytes = push!(name!(name.clone()));
                            wrapper.push(bytes);
                        }
                        _ => {
                            // arg is stored in `temp`
                            eval(vec![args[i].clone()], wrapper, state);
                            let bytes = push!(ident!("temp"));
                            wrapper.push(bytes);
                        }
                    }

                    i += 1;
                }

                let is_pointer = match state.get(&name) {
                    Some(t) => t.starts_with("callback"),
                    None => false
                };

                match is_pointer {
                    true => {
                        let bytes = call!(ident!(name));
                        wrapper.push(bytes);
                    }
                    false => {
                        let bytes = call!(name!(name));
                        wrapper.push(bytes);
                    }
                }

                let typ = match state.get(&name) {
                    Some(t) => t,
                    None => &MType::Undefined.stringify()
                };

                let bytes = match typ.as_str() {
                    "number" => {
                        pop!(ident!("temp"))
                    }
                    "string" => {
                        pop!(ident!("temp_struct"))
                    }
                    "struct" => {
                        pop!(ident!("temp_struct"))
                    }
                    "callback" => {
                        nop!()
                    }
                    "null" => {
                        nop!()
                    }
                    _ => {
                        // Assume number on function return types
                        pop!(ident!("temp"))
                    }
                };

                wrapper.push(bytes);
            }

            Expr::Number(n) => {
                let bytes = mov!(immediate!(SIGNED(*n)), ident!("temp"));
                wrapper.push(bytes);
            }

            Expr::String(s) => {
                wrapper.push_string(&s);
                wrapper.push(push!(ident!(Wrapper::get_string_name(s))));
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
                eval(vec![*cond.clone()], wrapper, state);
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
                eval(*body.clone(), &mut body_wrapper, state);
                let true_scope = generate_scope(&body_wrapper.bytes);

                // After finishing the `true` body, jump to the end of the `if` statement
                // This it to prevent the `false` body from being executed right after the `true` body.
                let jump_after_true = jmp!(
                    immediate!(SIGNED(4)) // what index to jump to
                );

                // Create a new scope, which stores the `false` body
                let mut else_body_wrapper = Wrapper::new();
                eval(*else_body.clone(), &mut else_body_wrapper, state);
                let false_scope = generate_scope(&else_body_wrapper.bytes);

                let merged_scopes = [jump, true_scope, jump_after_true, false_scope].concat();
                let wrap = generate_scope(&merged_scopes);

                wrapper.push(wrap);
            }

            Expr::While(cond, body) => {
                let mut body_wrapper = Wrapper::new();

                // Evaluate the condition
                let mut cond_wrapper = Wrapper::new();
                eval(vec![*cond.clone()], &mut cond_wrapper, state);


                // If `temp` is 0, jump to the end of the `while` loop
                let jump = jne!(
                    ident!("temp"),
                    immediate!(SIGNED(1)),
                    immediate!(SIGNED(4)) // what index to jump to
                );

                // Evaluate the body
                eval(*body.clone(), &mut body_wrapper, state);

                // Jump back to the condition
                let jump_back = jmp!(
                    immediate!(SIGNED(0))
                );

                let merged_scopes = [generate_scope(&cond_wrapper.bytes), jump, generate_scope(&body_wrapper.bytes), jump_back].concat();
                let wrap = generate_scope(&merged_scopes);

                wrapper.push(wrap);
            }

            Expr::Return(val) => {
                eval(vec![*val.clone()], wrapper, state);

                let return_bytes = ret!(
                    ident!("temp")
                );

                wrapper.push(return_bytes);
                
            }

            Expr::Pass => {
                wrapper.push(nop!());
            }

            // TODO: This is horribly nested. Rewrite this so it is recursive instead.
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
                    Expr::CallFunc(name, args) => {
                        let name = match *name.clone() {
                            Expr::Identifier(name) => name.clone(),
                            _ => panic!("Expected identifier, got {:?}", name)
                        };
                        let full_name = format!("{}.{}", item, name);
                        let mut j = 0;
                        while j < args.len() {
                            match &args[j] {
                                Expr::Number(n) => {
                                    wrapper.push(push!(immediate!(SIGNED(*n))));
                                }
                                Expr::Identifier(name) => {
                                    match full_name.as_str() {
                                        "io.println" | "io.print" => {
                                            let text = format!("{}.text", name);
                                            let length = format!("{}.length", name);

                                            wrapper.push(push!(ident!(text)));
                                            wrapper.push(push!(ident!(length)));
                                        }
                                        _ => {
                                            wrapper.push(push!(ident!(name.clone())));
                                        }
                                    }
                                }
                                Expr::String(s) => {
                                    eval(vec![args[j].clone()], wrapper, state);
                                }
                                Expr::PropertyAccess(obj, prop) => {
                                    let obj = match *obj.clone() {
                                        Expr::Identifier(name) => name.clone(),
                                        _ => panic!("Expected identifier, got {:?}", obj)
                                    };
                                    let prop = match *prop.clone() {
                                        Expr::Identifier(name) => name.clone(),
                                        Expr::ArrayAccess(_, _) => {
                                            eval(vec![args[j].clone()], wrapper, state);
                                            let bytes = push!(ident!("temp"));
                                            wrapper.push(bytes);

                                            j += 1;
                                            continue;
                                        }
                                        Expr::CallFunc(_, _) => {
                                            eval(vec![args[j].clone()], wrapper, state);
                                            let bytes = push!(ident!("temp_struct"));
                                            wrapper.push(bytes);
                                            j += 1;
                                            continue;
                                        }
                                        _ => panic!("Expected identifier, got {:?}", prop)
                                    };
                                    let full_name = format!("{}.{}", obj, prop);
                                    wrapper.push(push!(ident!(full_name)));
                                }
                                _ => {
                                    // arg is stored in `temp`
                                    eval(vec![args[j].clone()], wrapper, state);
                                    let bytes = push!(ident!("temp"));
                                    wrapper.push(bytes);
                                }
                            }

                            j += 1;
                        }

                        let bytes = call!(name!(full_name));
                        wrapper.push(bytes);

                        let typ = match state.get(&full_name) {
                            Some(t) => t,
                            None => &MType::Undefined.stringify()
                        };
                        let bytes = match typ.as_str() {
                            "number" => {
                                pop!(ident!("temp"))
                            }
                            "string" => {
                                pop!(ident!("temp_struct"))
                            }
                            "struct" => {
                                pop!(ident!("temp_struct"))
                            }
                            "callback" => {
                                nop!()
                            }
                            "null" => {
                                nop!()
                            }
                            _ => {
                                // Assume number on function return types
                                pop!(ident!("temp"))
                            }
                        };
                        wrapper.push(bytes);

                    }
                    Expr::ArrayAccess(name, index) => {
                        let index = match *index.clone() {
                            Expr::Number(n) => n,
                            _ => panic!("Expected number, got {:?}", index)
                        };

                        let full_name = format!("{}.{}", item, name);
                        let pointer = add!(
                            ident!(full_name),
                            immediate!(SIGNED(index)),
                            ident!("temp")
                        );
                        wrapper.push(pointer);

                        let bytes = self::deref!(ident!("temp"), ident!("temp".to_string()));
                        wrapper.push(bytes);
                    }
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

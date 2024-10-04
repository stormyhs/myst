use ::rainbow_wrapper::rainbow_wrapper::wrapper::Wrapper;
use ::rainbow_wrapper::rainbow_wrapper::types::*;
use ::rainbow_wrapper::ret;
use ::rainbow_wrapper::add;
use ::rainbow_wrapper::var;
use ::rainbow_wrapper::mov;

use crate::enums::*;

pub fn eval(ast: Vec<Expr>, wrapper: &mut Wrapper) {
    /*
    let mut i = 0;
    while i < ast.len() {
        match &ast[i] {
            Expr::BinOp(op, left, right) => {
                match (*left.clone(), *right.clone()) {
                    (Expr::Number(l), Expr::Number(r)) => {
                        match op {
                            Operator::Add => {
                                let addition = add!(
                                    "a".to_string(),
                                    "b".to_string(),
                                    "temp".to_string()
                                );

                                wrapper.push_bytes(addition);
                            },
                            _ => { todo!() }
                        }
                    },
                    _ => { todo!() }
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
                    Value::IDENT(val.clone())
                );

                wrapper.push_bytes(return_bytes);
                
            }
            _ => {
                println!("Ignoring instruction: {:?}", ast[i]);
            }
        }

        i+=1;
    }
    */
}

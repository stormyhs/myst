use std::env;
use std::fs;

mod enums;
mod tokenizer;
mod parser;
mod engine;

use ::rainbow_wrapper::rainbow_wrapper::wrapper::Wrapper;
use ::rainbow_wrapper::rainbow_wrapper::types::*;
use ::rainbow_wrapper::var;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut debug_mode = false;
    let mut running_tests = false;
    let mut source = String::new();

    for arg in &args {
        match arg.as_str() {
            "--debug" | "-d" => {
                debug_mode = true;
            },
            "--test" | "-t" => {
                running_tests = true;
            }

            _ => {
                if source == "" {
                    source = arg.to_string();
                }
            }
        }
    }

    if running_tests {
        todo!();
    }

    if debug_mode {
        let current_path = env::current_dir().unwrap();
        println!("Current directory: {}", current_path.display());
    }

    if source == "" {
        panic!("No source file provided.");
    }

    let source = std::fs::read_to_string(source).expect("Could not read source file");
    let tokens = tokenizer::tokenize(source);
    if debug_mode {
        println!("\n\nTokens: {:?}\n\n", tokens);
    }

    let mut parser = parser::Parser::new(tokens.clone());
    let ast = parser.parse();
    if debug_mode {
        println!("\n\nAST: {:#?}", ast);
    }

    /*
    let mut wrapper = Wrapper::new();
    let create_var_bytes = var!(
        Value::TYPE(vec![Type::I16]),
        Value::NAME("temp".to_string())
    );
    wrapper.push_bytes(create_var_bytes);
    engine::eval(ast, &mut wrapper);

    println!("Myst source code translated to Rainbow bytes:\n{:?}", wrapper.bytes);

    fs::write("output.rbb", wrapper.emit()).expect("Could not write bytecode to file");

    println!("Bytecode written to output.rbb");
    */
}

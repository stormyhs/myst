use std::env;
use std::fs;

mod enums;
mod tokenizer;
mod parser;

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
    let tokens = tokenizer::tokenize(source, debug_mode);
    if debug_mode {
        println!("\n\nTokens: {:?}\n\n", tokens);
    }

    let mut parser = parser::Parser::new(tokens.clone(), debug_mode);
    let ast = parser.parse();
    if debug_mode {
        println!("\n\nAST: {:#?}", ast);
    }

}

use std::fs;
use std::env;
use std::io::Write;
use std::collections::HashMap;

mod tokenizer;
mod parser;
mod tokens;
mod engine;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut repl_mode = false;
    let mut debug_mode = false;

    let mut source = String::new();

    for arg in &args {
        match arg.as_str() {
            "--repl" => {
                repl_mode = true;
            },
            "--debug" => {
                debug_mode = true;
            },

            _ => {
                if source == "" {
                    source = arg.to_string();
                }
            }
        }
    }

    let state: HashMap<String, i16> = HashMap::new();

    if repl_mode {
        loop {
            print!("> ");
            _ = std::io::stdout().flush();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "exit" {
                break;
            }

            let tokens = tokenizer::tokenize(input, debug_mode);
            let parsed = parser::parse(tokens);

            let result = engine::evaluate(parsed, &state);

            println!("# {}", result);
        }
    }
    else {
        if source == "" {
            println!("No source file path provided. Did you mean to use --repl?");
            return;
        }

        let source = fs::read_to_string(source)
            .expect("Unable to read source file");

        let tokens = tokenizer::tokenize(source, debug_mode);
        if debug_mode {
            println!("Tokens: {:?}", tokens);
        }

        let parsed = parser::parse(tokens);
        if debug_mode {
            println!("AST: {:?}", parsed);
        }

        let result = engine::evaluate(parsed, &state);

        println!("{}", result);
    }

}


use std::fs;
use std::env;
use std::io::Write;

use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

mod tokenizer;
mod parser;
mod tokens;
mod engine;

fn main() {
    let start_time = std::time::Instant::now();

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut debug_mode = false;
    let mut output_path = "out.rbb";
    let mut source = String::new();

    for arg in &args {
        match arg.as_str() {
            "--debug" | "-d" => {
                debug_mode = true;
            },

            "--output" | "-o" => {
                output_path = arg;
            },

            _ => {
                if source == "" {
                    source = arg.to_string();
                }
            }
        }
    }

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

    let parsed = parser::parse(tokens, debug_mode);
    if debug_mode {
        println!("AST: {:?}", parsed);
    }

    let result = engine::evaluate(parsed, &mut HashMap::new(), debug_mode);
    if debug_mode {
        println!("Program evaluated to: {:?}", result);
    }

    let elapsed = start_time.elapsed();
    let elapsed = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9;
    //if debug_mode {
        println!("Execution took: {}s", elapsed);
    //}
}


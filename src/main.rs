use std::fs;
use std::env;
use std::collections::HashMap;
use std::io::Write;

use colored::*;

mod tokenizer;
mod parser;
mod tokens;
mod engine;

fn run_file(path: &str, debug_mode: bool, is_testing: bool) -> Vec<tokens::Expr> {
    let start_time = std::time::Instant::now();

    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            if is_testing {
                println!(
                    "{} {} {}",
                    "Could not read".red(),
                    format!("{}.", path).red(),
                    "Are you in the tests/ directory?".red()
                );
            }
            panic!("Could not read file: {}", e);
        }
    };

    let tokens = tokenizer::tokenize(source, debug_mode);
    if debug_mode {
        println!("Tokens: {:?}", tokens);
    }

    let mut parsed = parser::parse(tokens, debug_mode);
    if debug_mode {
        println!("AST:");
        println!("{:#?}", parsed);
    }

    let result = engine::evaluate(&mut parsed, &mut HashMap::new(), debug_mode);
    if debug_mode {
        println!("Program evaluated to: {:?}", result);
        println!("Execution took {:.6}s", start_time.elapsed().as_secs_f32());
    }

    return result;
}

fn run_test(path: &str, expect: tokens::Expr) -> bool {
    print!("{}: ", path);
    std::io::stdout().flush().unwrap();

    let result = run_file(path, false, true);
    if result.len() != 1 || result[0] != expect {
        print!("{}  ", "failed".red());
        print!("{}: {:?}  ", "Expected".yellow(), expect);
        println!("{}: {:?}  ", "Got".yellow(), result);

        return false;
    } else {
        println!("{}", "passed".green());
        return true;
    }
}

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

    if debug_mode {
        let current_path = env::current_dir().unwrap();
        println!("Current directory: {}", current_path.display());
    }

    if running_tests {
        println!("\n\nRunning tests...\n");

        let mut tests: Vec<bool> = Vec::new();

        let expect = tokens::Expr::Number(10);

        tests.push(run_test("variables.myst", expect.clone()));
        tests.push(run_test("numbers.myst", expect.clone()));
        tests.push(run_test("loops.myst", expect.clone()));
        tests.push(run_test("functions.myst", expect.clone()));
        tests.push(run_test("conditions.myst", expect.clone()));
        tests.push(run_test("arrays.myst", expect.clone()));
        tests.push(run_test("imports.myst", expect.clone()));

        let mut passed = 0;
        for test in &tests {
            if *test {
                passed += 1;
            }
        }

        println!();
        let text = format!("{} out of {} passed", passed, tests.len());
        if passed == tests.len() {
            println!("{}", text.green());
        } else {
            println!("{}", text.red());
        }
        println!();

        return;
    } else {
        if source == "" {
            println!("No source file path provided. Did you mean to use --repl?");
            return;
        }

        run_file(&source, debug_mode, false);
    }
}


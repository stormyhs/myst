use std::env;
use std::fs;
use std::io::ErrorKind;
use std::process::Command;
use std::process;

mod enums;
mod tokenizer;
mod parser;
mod engine;

use rainbow_wrapper::wrapper::Wrapper;
use rainbow_wrapper::types::*;
use rainbow_wrapper::var;

use colored::*;

fn get_rb_path() -> String {
    return env::var("RAINBOW_PATH").unwrap_or_else(|_| "/home/stormy/code/Rainbow/target/debug/rainbow".to_string());
}

fn run_with_rb(path: String, debug: bool) -> i32 {
    let current_dir = String::from(env::current_dir().unwrap().to_str().unwrap());
    let mut output = Command::new(get_rb_path());
    output.arg(format!("{}/{}", current_dir, path));
    if debug {
        output.arg("--debug");
    }
    let output = match output.output() {
        Ok(o) => o,
        Err(e) => {
            println!("❌ Could not run Rainbow: {}", e);
            std::process::exit(1);
        }
    };

    if output.stdout.len() > 0 {
        print!("RB {}: {}", "stdout".red(), String::from_utf8_lossy(&output.stdout));
    }
    if output.stderr.len() > 0 {
        print!("RB {}: {}", "stderr".red(), String::from_utf8_lossy(&output.stderr));
    }

    return output.status.code().unwrap();
}

fn run_tests(debug: bool) {
    println!("\nRunning tests...\n");

    let test_files = vec![
        "tests/vars.myst",
        "tests/comments.myst",
        "tests/return.myst",
        "tests/imports.myst",
        "tests/functions.myst",
        "tests/conditions.myst",
        "tests/arrays.myst",
        "tests/loops.myst",
    ];

    let mut failed = 0;
    for file in test_files.clone() {
        let source = match std::fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => {
                        println!("❌ Could not find file '{}'", file);
                        std::process::exit(1);
                    },
                    _=> {
                        println!("❌ Could not read file '{}'", file);
                        println!("{e}");
                        std::process::exit(1);
                    }
                }
            }
        };

        let tokens = tokenizer::tokenize(source);
        let mut parser = parser::Parser::new(tokens.clone());
        let ast = parser.parse();

        let mut wrapper = Wrapper::new();
        let create_var_bytes = var!(
            Value::TYPE(vec![Type::I64]),
            Value::NAME("temp".to_string())
        );
        wrapper.push(create_var_bytes);
        let create_var_bytes = var!(
            Value::TYPE(vec![Type::I64]),
            Value::NAME("temp2".to_string())
        );
        wrapper.push(create_var_bytes);
        let create_var_bytes = var!(
            Value::TYPE(vec![Type::I64]),
            Value::NAME("temp3".to_string())
        );
        wrapper.push(create_var_bytes);

        engine::eval(ast, &mut wrapper);

        let output_path = "output.rbb";
        fs::write(output_path, wrapper.emit()).expect("Could not write bytecode to file");

        let ret = run_with_rb(output_path.to_string(), debug);
        let expected_output = 69;

        if ret == expected_output {
            println!("✔  Test passed: {}", file.green());
        } else {
            print!("❌ Test failed: {} - ", file.red());
            print!("Expected: {} - ", expected_output.to_string().green());
            println!("Got: {}", ret.to_string().red());

            failed += 1;
        }
    }

    if failed == 0 {
        println!("\n✔️ {}/{} tests passed\n", test_files.len(), test_files.len());
    } else {
        println!("\n❌ {}/{} tests passed\n", test_files.len() - failed, test_files.len());
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut debug_mode = false;
    let mut running_tests = false;
    let mut source = String::new();
    let mut output_path = String::from("out.rbb");
    let mut no_run = false;
    let mut help = false;

    let mut i = 0;
    while i < args.len() {
        let arg = args[i].clone();
        match arg.as_str() {
            "--" => {
                i += 1;
                continue;
            }
            "--debug" | "-d" => {
                debug_mode = true;
            },
            "--test" | "-t" => {
                running_tests = true;
            }
            "--output" | "-o" => {
                output_path = args[i + 1].clone();
                i += 1;
            },
            "--no-run" | "-n" => {
                no_run = true;
            },
            "--help" | "-h" => {
                help = true;
            },
            _ => {
                source = arg;
            }
        }

        i += 1;
    }

    if args.len() == 0 || help {
        println!("\nUsage: {} {} {}\n", "myst".blue(), "[options]".cyan(), "<source file>".green());
        println!("Options:");
        println!("  {} {}:          Enable debug mode", "--debug".cyan(), "-d".cyan());
        println!("  {} {}:           Run tests", "--test".cyan(), "-t".cyan());
        println!("  {} {} {}:  Specify output file", "--output".cyan(), "-o".cyan(), "<file>".green());
        println!("  {} {}:         Do not execute the output file", "--no-run".cyan(), "-n".cyan());
        println!("  {} {}:           Display this help message", "--help".cyan(), "-h".cyan());
        println!("\nExample:");
        println!("  {} -d -o {} {}", "myst".blue(), "build.rbb".green(), "source.myst".green());
        println!("  {} {} -d", "myst".blue(), "source.myst".green());
        println!();
        return;
    }

    if running_tests {
        run_tests(debug_mode);
        return;
    } else if source == "" {
        println!("❌ No source file provided.");
        std::process::exit(1);
    }

    let source = match std::fs::read_to_string(source.clone()) {
        Ok(s) => s,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    println!("❌ Could not find file '{}'", source);
                    std::process::exit(1);
                },
                _=> {
                    println!("❌ Could not read file '{}'", source);
                    println!("{e}");
                    std::process::exit(1);
                }
            }
        }
    };

    let tokens = tokenizer::tokenize(source);
    if debug_mode {
        println!("\n\nTokens: {:?}", tokens);
    }

    let mut parser = parser::Parser::new(tokens.clone());
    let ast = parser.parse();
    if debug_mode {
        println!("\n\nAST: {:#?}", ast);
    }

    let mut wrapper = Wrapper::new();

    // TODO: Use the stack instead. This is just a PoC
    let create_var_bytes = var!(
        Value::TYPE(vec![Type::I64]),
        Value::NAME("temp".to_string())
    );
    wrapper.push(create_var_bytes);

    let create_var_bytes = var!(
        Value::TYPE(vec![Type::I64]),
        Value::NAME("temp2".to_string())
    );
    wrapper.push(create_var_bytes);

    let create_var_bytes = var!(
        Value::TYPE(vec![Type::I64]),
        Value::NAME("temp3".to_string())
    );
    wrapper.push(create_var_bytes);

    engine::eval(ast, &mut wrapper);

    if debug_mode {
        println!("\n\nMyst source code translated to Rainbow bytes:\n{:?}\n\n", wrapper.bytes);
    }

    fs::write(output_path.clone(), wrapper.emit()).expect("Could not write bytecode to file");

    println!("✔  Compiled to {}", output_path.green());

    if !no_run {
        let ret = run_with_rb(output_path, debug_mode);
        println!("✔️ Rainbow exited with code {}", ret);
    }

}

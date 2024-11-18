use std::env;
use std::fs;
use std::io::ErrorKind;
use std::process::Command;
use std::collections::HashMap;

mod enums;
mod tokenizer;
mod parser;
mod engine;

use crate::enums::Expr;

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

    output.arg("-l");
    output.arg("/home/stormy/code/Rainbow/core");

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
        print!("{}", String::from_utf8_lossy(&output.stdout));
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
        "tests/types.myst",
        "tests/fnargs.myst",
        "tests/lambda.myst",
        "tests/ascii.myst",
        "tests/files.myst",
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
        wrapper.push(var!(
            Value::TYPE(vec![Type::I64]),
            Value::NAME("temp".to_string())
        ));
        wrapper.push(var!(
            Value::TYPE(vec![Type::I64]),
            Value::NAME("temp2".to_string())
        ));
        wrapper.push(var!(
            Value::TYPE(vec![Type::I64]),
            Value::NAME("temp3".to_string())
        ));
        wrapper.push(var!(
            Value::TYPE(vec![Type::STRUCT("_".to_string())]),
            Value::NAME("temp_struct".to_string())
        ));

        // Poor man's function signatures. @gromton12 kindly fix this.
        let mut state: HashMap<String, String> = HashMap::new();
        state.insert("string.ntos".to_string(), "struct".to_string());
        state.insert("string.ston".to_string(), "number".to_string());
        state.insert("string.new".to_string(), "struct".to_string());
        state.insert("io.println".to_string(), "null".to_string());
        state.insert("io.print".to_string(), "null".to_string());
        state.insert("fs.clear".to_string(), "null".to_string());
        state.insert("fs.readString".to_string(), "struct".to_string());
        state.insert("fs.close".to_string(), "null".to_string());
        state.insert("fs.open".to_string(), "number".to_string());

        let implicit_imports = vec![
            Expr::Import("io.rbb".to_string()),
            Expr::Import("string.rbb".to_string()),
            Expr::Import("fs.rbb".to_string()),
        ];

        let mut new_ast = implicit_imports.clone();
        new_ast.extend(ast.clone());
        let ast = new_ast;

        engine::eval(ast, &mut wrapper, &mut state);

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
    let mut user_overwrote_output_path = false;
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
                user_overwrote_output_path = true;
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
                if !user_overwrote_output_path {
                    let mut parts: Vec<&str> = source.split(".").collect();
                    parts.pop();
                    output_path = format!("{}.rbb", parts.join("."));
                }
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
    wrapper.push(var!(
        Value::TYPE(vec![Type::I64]),
        Value::NAME("temp".to_string())
    ));
    wrapper.push(var!(
        Value::TYPE(vec![Type::I64]),
        Value::NAME("temp2".to_string())
    ));
    wrapper.push(var!(
        Value::TYPE(vec![Type::I64]),
        Value::NAME("temp3".to_string())
    ));
    wrapper.push(var!(
        Value::TYPE(vec![Type::STRUCT("_".to_string())]),
        Value::NAME("temp_struct".to_string())
    ));

    let mut state: HashMap<String, String> = HashMap::new();
    // Poor man's function signatures. @gromton12 kindly fix this.
    state.insert("string.ntos".to_string(), "struct".to_string());
    state.insert("string.ston".to_string(), "number".to_string());
    state.insert("string.new".to_string(), "struct".to_string());
    state.insert("io.println".to_string(), "null".to_string());
    state.insert("io.print".to_string(), "null".to_string());
    state.insert("fs.clear".to_string(), "null".to_string());
    state.insert("fs.readString".to_string(), "struct".to_string());
    state.insert("fs.close".to_string(), "null".to_string());
    state.insert("fs.open".to_string(), "number".to_string());

    let implicit_imports = vec![
        Expr::Import("io.rbb".to_string()),
        Expr::Import("string.rbb".to_string()),
        Expr::Import("fs.rbb".to_string()),
    ];

    let mut new_ast = implicit_imports.clone();
    new_ast.extend(ast.clone());
    let ast = new_ast;

    engine::eval(ast.clone(), &mut wrapper, &mut state);
    fs::write(output_path.clone(), wrapper.emit()).expect("Could not write bytecode to file");

    if debug_mode {
        println!("✔  Compiled to {}", output_path.green());
    }

    if !no_run {
        let ret = run_with_rb(output_path, debug_mode);
        if debug_mode {
            println!("✔️ Rainbow exited with code {}", ret);
        }
    }
}

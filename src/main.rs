use std::env;
use std::fs;
use std::process;

use dinoe_compiler::lexer::Lexer;
use dinoe_compiler::parser::Parser;
use dinoe_compiler::typechecker::TypeChecker;
use dinoe_compiler::interpreter::Interpreter;

fn main() {
    // 1. Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: dinoe <file.dinoe> [--dump]");
        process::exit(1);
    }

    let filename = &args[1];
    let dump = args.contains(&"--dump".to_string());

    // 2. Read the source file
    let source = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        process::exit(1);
    });

    // 3. Run the pipeline
    match run_pipeline(&source, dump) {
        Ok(value) => println!("Result: {:?}", value),
        Err(err) => {
            eprintln!("Compiler Error: {}", err);
            process::exit(1);
        }
    }
}

fn run_pipeline(source: &str, dump: bool) -> Result<dinoe_compiler::interpreter::Value, String> {
    // Stage 1: Lexer
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex_all();
    if dump {
        println!("--- TOKENS ---");
        println!("{:#?}", tokens);
    }

    // Stage 2: Parser
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;
    if dump {
        println!("\n--- AST ---");
        println!("{:#?}", program);
        println!("\n--- EXECUTION ---");
    }

    // Stage 3: Type Checker
    let mut checker = TypeChecker::new();
    checker.check_program(&program)?;

    // Stage 4: Interpreter
    let mut interpreter = Interpreter::new();
    interpreter.execute_program(program)
}

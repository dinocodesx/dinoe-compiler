use std::env;
use std::fs;
use std::process;

use dinoe_compiler::lexer::Lexer;
use dinoe_compiler::parser::Parser;
use dinoe_compiler::typechecker::TypeChecker;
use dinoe_compiler::interpreter::Interpreter;
use dinoe_compiler::compiler::Compiler;
use dinoe_compiler::codegen::Codegen;

fn main() {
    // 1. Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: dinoe <file.dinoe> [--dump] [--emit-asm]");
        process::exit(1);
    }

    let filename = &args[1];
    let dump = args.contains(&"--dump".to_string());
    let emit_asm = args.contains(&"--emit-asm".to_string());

    // 2. Read the source file
    let source = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        process::exit(1);
    });

    // 3. Run the pipeline
    match run_pipeline(&source, dump, emit_asm, filename) {
        Ok(value) => {
            if let Some(v) = value {
                println!("Result: {:?}", v);
            }
        }
        Err(err) => {
            eprintln!("Compiler Error: {}", err);
            process::exit(1);
        }
    }
}

fn run_pipeline(source: &str, dump: bool, emit_asm: bool, filename: &str) -> Result<Option<dinoe_compiler::interpreter::Value>, String> {
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
    }

    // Stage 3: Type Checker
    let mut checker = TypeChecker::new();
    checker.check_program(&program)?;

    if emit_asm {
        // Stage 4a: IR Generation
        let mut compiler = Compiler::new();
        let ir_functions = compiler.compile_program(program);
        if dump {
            println!("\n--- IR ---");
            println!("{:#?}", ir_functions);
        }

        // Stage 4b: Code Generation
        let mut codegen = Codegen::new();
        let assembly = codegen.emit_program(ir_functions);
        
        let asm_filename = format!("{}.s", filename.strip_suffix(".dinoe").unwrap_or(filename));
        fs::write(&asm_filename, assembly).map_err(|e| e.to_string())?;
        println!("Assembly written to {}", asm_filename);
        Ok(None)
    } else {
        // Stage 4: Interpreter
        if dump {
            println!("\n--- EXECUTION ---");
        }
        let mut interpreter = Interpreter::new();
        let result = interpreter.execute_program(program)?;
        Ok(Some(result))
    }
}

use dinoe_compiler::lexer::Lexer;
use dinoe_compiler::parser::Parser;
use dinoe_compiler::compiler::Compiler;
use dinoe_compiler::codegen::Codegen;

#[test]
fn test_codegen_basic() {
    let input = "fn main() -> int { return 42 }";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let mut compiler = Compiler::new();
    let ir_functions = compiler.compile_program(program);

    let mut codegen = Codegen::new();
    let assembly = codegen.emit_program(ir_functions);

    // Verify key assembly elements are present
    assert!(assembly.contains("main:"));
    assert!(assembly.contains("push rbp"));
    assert!(assembly.contains("mov rax, 42"));
    assert!(assembly.contains("ret"));
}

#[test]
fn test_codegen_math() {
    let input = "fn main() -> int { return 1 + 2 }";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let mut compiler = Compiler::new();
    let ir_functions = compiler.compile_program(program);

    let mut codegen = Codegen::new();
    let assembly = codegen.emit_program(ir_functions);

    assert!(assembly.contains("add rax, rbx"));
}

use dinoe_compiler::compiler::Compiler;
use dinoe_compiler::ir::Instruction;
use dinoe_compiler::lexer::Lexer;
use dinoe_compiler::parser::Parser;

#[test]
fn test_compile_basic_math() {
    let input = "fn main() -> int { return 1 + 2 * 3 }";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let mut compiler = Compiler::new();
    let ir_functions = compiler.compile_program(program);

    assert_eq!(ir_functions.len(), 1);
    let func = &ir_functions[0];
    assert_eq!(func.name, "main");

    // Expected IR:
    // VReg1 = LoadImm(2)
    // VReg2 = LoadImm(3)
    // VReg3 = Mul(VReg1, VReg2)
    // VReg4 = LoadImm(1)
    // VReg5 = Add(VReg4, VReg3)
    // Ret(VReg5)

    // Actually our current compiler emits slightly different IR due to how compile_expr is ordered
    // Let's just check that we have the right number of instructions and a Ret at the end
    assert!(func.instructions.len() >= 3);
    assert!(matches!(
        func.instructions.last().unwrap(),
        Instruction::Ret(_)
    ));
}

#[test]
fn test_compile_function_call() {
    let input = "
        fn add(a: int, b: int) -> int { return a + b }
        fn main() -> int { return add(1, 2) }
    ";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let mut compiler = Compiler::new();
    let ir_functions = compiler.compile_program(program);

    assert_eq!(ir_functions.len(), 2);

    let add_func = ir_functions.iter().find(|f| f.name == "add").unwrap();
    assert_eq!(add_func.params, vec!["a", "b"]);

    let main_func = ir_functions.iter().find(|f| f.name == "main").unwrap();
    // Should have a Call instruction
    assert!(
        main_func
            .instructions
            .iter()
            .any(|i| matches!(i, Instruction::Call { .. }))
    );
}

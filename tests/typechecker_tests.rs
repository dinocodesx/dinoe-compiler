use dinoe_compiler::lexer::Lexer;
use dinoe_compiler::parser::Parser;
use dinoe_compiler::typechecker::TypeChecker;

#[test]
fn test_typecheck_valid_program() {
    let input = "
        fn add(a: int, b: int) -> int {
            return a + b
        }
    ";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let mut checker = TypeChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_ok());
}

#[test]
fn test_typecheck_arithmetic_error() {
    let input = "
        fn fail() -> int {
            return 1 + true
        }
    ";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let mut checker = TypeChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Arithmetic operators require integer operands"));
}

#[test]
fn test_typecheck_undefined_variable() {
    let input = "
        fn fail() -> int {
            return x + 1
        }
    ";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let mut checker = TypeChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Undefined variable: x"));
}

#[test]
fn test_typecheck_return_type_mismatch() {
    let input = "
        fn fail() -> bool {
            return 42
        }
    ";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let mut checker = TypeChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Type mismatch: expected return type Bool, but got Int"));
}

#[test]
fn test_typecheck_function_call_error() {
    let input = "
        fn add(a: int, b: int) -> int {
            return a + b
        }
        fn main() -> int {
            return add(1, true)
        }
    ";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    let mut checker = TypeChecker::new();
    let result = checker.check_program(&program);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Argument type mismatch for function 'add'"));
}

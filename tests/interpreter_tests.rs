use dinoe_compiler::lexer::Lexer;
use dinoe_compiler::parser::Parser;
use dinoe_compiler::typechecker::TypeChecker;
use dinoe_compiler::interpreter::{Interpreter, Value};

fn run_dinoe(input: &str) -> Result<Value, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex_all();
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program()?;

    let mut checker = TypeChecker::new();
    checker.check_program(&program)?;

    let mut interpreter = Interpreter::new();
    interpreter.execute_program(program)
}

#[test]
fn test_interpreter_basic_math() {
    let input = "
        fn main() -> int {
            return 1 + 2 * 3
        }
    ";
    let result = run_dinoe(input).unwrap();
    assert_eq!(result, Value::Int(7));
}

#[test]
fn test_interpreter_function_call() {
    let input = "
        fn add(a: int, b: int) -> int {
            return a + b
        }

        fn main() -> int {
            return add(10, 20)
        }
    ";
    let result = run_dinoe(input).unwrap();
    assert_eq!(result, Value::Int(30));
}

#[test]
fn test_interpreter_nested_calls() {
    let input = "
        fn square(n: int) -> int {
            return n * n
        }

        fn sum_squares(a: int, b: int) -> int {
            return square(a) + square(b)
        }

        fn main() -> int {
            return sum_squares(3, 4)
        }
    ";
    let result = run_dinoe(input).unwrap();
    assert_eq!(result, Value::Int(25));
}

#[test]
fn test_interpreter_division_by_zero() {
    let input = "
        fn main() -> int {
            return 10 / 0
        }
    ";
    let result = run_dinoe(input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Division by zero");
}

#[test]
fn test_interpreter_boolean_logic() {
    let input = "
        fn is_equal(a: bool, b: bool) -> bool {
            return a
        }

        fn main() -> bool {
            return is_equal(true, false)
        }
    ";
    let result = run_dinoe(input).unwrap();
    assert_eq!(result, Value::Bool(true));
}

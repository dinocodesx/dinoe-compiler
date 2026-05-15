use dinoe_compiler::lexer::Lexer;
use dinoe_compiler::parser::Parser;
use dinoe_compiler::ast::{Expr, Op};

#[test]
fn test_parse_number() {
    let mut lexer = Lexer::new("42");
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expr().unwrap();
    assert_eq!(ast, Expr::Number(42));
}

#[test]
fn test_parse_precedence() {
    // 1 + 2 * 3 should be 1 + (2 * 3)
    let mut lexer = Lexer::new("1 + 2 * 3");
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expr().unwrap();

    let expected = Expr::BinOp {
        op: Op::Add,
        lhs: Box::new(Expr::Number(1)),
        rhs: Box::new(Expr::BinOp {
            op: Op::Mul,
            lhs: Box::new(Expr::Number(2)),
            rhs: Box::new(Expr::Number(3)),
        }),
    };
    assert_eq!(ast, expected);
}

#[test]
fn test_parse_parentheses() {
    // (1 + 2) * 3 should respect parentheses
    let mut lexer = Lexer::new("(1 + 2) * 3");
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expr().unwrap();

    let expected = Expr::BinOp {
        op: Op::Mul,
        lhs: Box::new(Expr::BinOp {
            op: Op::Add,
            lhs: Box::new(Expr::Number(1)),
            rhs: Box::new(Expr::Number(2)),
        }),
        rhs: Box::new(Expr::Number(3)),
    };
    assert_eq!(ast, expected);
}

#[test]
fn test_parse_error() {
    let mut lexer = Lexer::new("1 + (2 * 3"); // Missing ')'
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let result = parser.parse_expr();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Expected ')' after expression");
}

#[test]
fn test_parse_full_function() {
    let input = "
        fn add(a: int, b: int) -> int {
            return a + b
        }
    ";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex_all();
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().unwrap();

    assert_eq!(program.len(), 1);
    
    match &program[0] {
        dinoe_compiler::ast::Stmt::FnDef { name, params, return_type, body } => {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
            assert_eq!(params[0].0, "a");
            assert_eq!(params[1].0, "b");
            assert_eq!(return_type, &dinoe_compiler::ast::Type::Int);
            assert_eq!(body.len(), 1);
        },
        _ => panic!("Expected function definition"),
    }
}

use dinoe_compiler::ast::{Expr, Op};

#[test]
fn test_ast_construction() {
    // Manually build an AST for: a + 5
    let ast = Expr::BinOp {
        op: Op::Add,
        lhs: Box::new(Expr::Ident("a".to_string())),
        rhs: Box::new(Expr::Number(5)),
    };

    if let Expr::BinOp { op, .. } = ast {
        assert_eq!(op, Op::Add);
    } else {
        panic!("Expected BinOp");
    }
}

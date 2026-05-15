/// Represents the available binary operators in Dinoe.
#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

/// Represents the primitive data types in Dinoe.
#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Bool,
}

/// Expressions are pieces of code that evaluate to a value.
/// For example: `5`, `a + b`, or `add(1, 2)`.
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /// A literal integer number.
    Number(i64),
    /// A literal boolean value.
    Bool(bool),
    /// A variable name (identifier).
    Ident(String),
    /// A binary operation like `a + b`.
    /// We use `Box` because the type is recursive (an Expr contains other Exprs).
    BinOp {
        op: Op,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    /// A function call like `print(x)`.
    Call { name: String, args: Vec<Expr> },
}

/// Statements are instructions that perform actions but don't necessarily return a value.
/// In Dinoe, a function body consists of a list of statements.
#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    /// A function definition: `fn name(params) -> return_type { body }`.
    FnDef {
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Stmt>,
    },
    /// A return statement: `return x + 1`.
    Return(Expr),
    /// An expression used as a statement (e.g., a function call on its own line).
    Expr(Expr),
}

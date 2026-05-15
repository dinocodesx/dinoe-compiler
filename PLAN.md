# Dinoe Compiler — Build Plan

A compiler for the `.dinoe` custom programming language, written in Rust.

---

## Language Overview

Programs are written in `.dinoe` files with the following features:

- **Functions** declared with `fn`
- **Data types:** `int` and `bool`
- **Operators:** `+`, `-`, `×`, `÷`, and grouping with `(` `)`

### Example Program

```
fn add(a: int, b: int) -> int {
    a + b
}

fn main() -> int {
    add(3, 5)
}
```

---

## Project Structure

```
dinoe/
├── src/
│   ├── main.rs          ← reads .dinoe file, runs the full pipeline
│   ├── lexer.rs         ← Stage 1: characters → tokens
│   ├── ast.rs           ← AST node type definitions
│   ├── parser.rs        ← Stage 2: tokens → AST
│   ├── typechecker.rs   ← Stage 3: validate types across AST
│   └── interpreter.rs   ← Stage 4: walk AST and evaluate
└── Cargo.toml
```

---

## Pipeline

```
source.dinoe  →  Lexer  →  Parser  →  Type Checker  →  Interpreter
```

---

## Stage 1 — Lexer

**Goal:** Turn raw source text into a flat list of tokens.

**What to build:**
- A `Token` enum with variants for every language element
- A `Lexer` struct that walks the source character by character
- A method that produces a `Vec<Token>`

**Token variants to define:**

```rust
pub enum Token {
    Fn,
    Ident(String),
    IntType,
    BoolType,
    Number(i64),
    True,
    False,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Comma,
    Arrow,   // ->
    Eof,
}
```

**Rust skills practised:** iterators, `char` methods (`is_alphabetic`, `is_numeric`), `String` building, `match`.

---

## Stage 2 — Parser

**Goal:** Consume the token stream and produce an Abstract Syntax Tree (AST).

**What to build:**
- `Expr` and `Stmt` enums in `ast.rs`
- A recursive descent parser in `parser.rs`
- Functions like `parse_expr()`, `parse_term()`, `parse_factor()` to handle operator precedence

**AST nodes to define:**

```rust
pub enum Expr {
    Number(i64),
    Bool(bool),
    Ident(String),
    BinOp { op: Op, lhs: Box<Expr>, rhs: Box<Expr> },
    Call { name: String, args: Vec<Expr> },
}

pub enum Stmt {
    FnDef {
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Stmt>,
    },
    Return(Expr),
    Expr(Expr),
}

pub enum Op { Add, Sub, Mul, Div }
pub enum Type { Int, Bool }
```

**Rust skills practised:** `Box<T>` for recursive types, `Vec`, `Result<T, E>` for parse errors, recursive functions.

---

## Stage 3 — Type Checker

**Goal:** Walk the AST and verify type consistency before evaluation.

**What to build:**
- A `TypeChecker` struct with a symbol table (function signatures, variable types)
- A `check_expr()` method that returns the inferred type or an error
- A `check_fn()` method that validates each function body

**Errors to catch:**
- Adding a `bool` and an `int` (type mismatch)
- Calling a function with wrong argument types
- Returning the wrong type from a function
- Using an undefined variable or function

**Rust skills practised:** `HashMap` for symbol tables, returning `Result<Type, String>`, tree traversal with `match`.

---

## Stage 4 — Interpreter

**Goal:** Walk the type-checked AST and evaluate it to a result.

**What to build:**
- A `Value` enum: `Value::Int(i64)` and `Value::Bool(bool)`
- An `Interpreter` struct that holds a call stack / environment
- An `eval_expr()` method and an `eval_fn()` method
- Entry point: find and call `main()`

**Rust skills practised:** `HashMap` for variable environments, `Box<dyn Error>`, cloning values, recursive evaluation.

---

## Error Handling Strategy

Use `Result<T, String>` (or a custom `DinoError` enum) throughout every stage. Each stage should produce clear, human-readable error messages with enough context to help the programmer fix their `.dinoe` file.

```rust
pub enum DinoError {
    LexError(String),
    ParseError(String),
    TypeError(String),
    RuntimeError(String),
}
```

---

## Suggested Build Order

1. `Token` enum + basic lexer (just keywords and numbers)
2. Parser for simple expressions (`1 + 2 * 3`)
3. Expand to full function definitions
4. Type checker for expressions
5. Interpreter for expressions
6. Wire everything together in `main.rs`
7. Add meaningful error messages throughout

---

## Rust Concepts You Will Practice

| Concept | Where it appears |
|---|---|
| `enum` with data | Tokens, AST nodes, values |
| Recursive types with `Box<T>` | `Expr::BinOp`, `Expr::Call` |
| `Result<T, E>` propagation | Every stage |
| Pattern matching (`match`) | Lexer, parser, interpreter |
| `HashMap` | Symbol table, environments |
| Iterators | Lexer character scanning |
| Structs and `impl` blocks | Lexer, Parser, TypeChecker, Interpreter |
| Recursive functions | Parser and interpreter |
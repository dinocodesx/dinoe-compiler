# Dinoe Compiler

A educational, multi-stage compiler for the `.dinoe` programming language, built from scratch in Rust.

Dinoe is a statically typed language that supports functions, basic arithmetic, and boolean logic. This project demonstrates the full lifecycle of a compiler, from raw source text to an executable x86_64 assembly backend.

## 🚀 Features

- **Lexical Analysis**: Custom hand-written Lexer with keyword and symbol recognition.
- **Recursive Descent Parser**: Transforms tokens into an Abstract Syntax Tree (AST) while respecting operator precedence.
- **Static Type Checking**: Verifies type consistency (integers and booleans) before execution.
- **Dual Execution Modes**:
    - **Interpreter**: Directly executes the AST for fast development.
    - **Assembly Backend**: Compiles the AST into an Intermediate Representation (IR) and then into x86_64 Assembly (System V ABI).
- **CLI Tooling**: Visualise internal compiler states with the `--dump` flag.

## 🏗️ Architecture

The compiler is organized into a modular pipeline:

1.  **Lexer (`lexer.rs`)**: Reads `.dinoe` source code and produces a stream of `Token`s.
2.  **Parser (`parser.rs`)**: Consumes tokens to build an **Abstract Syntax Tree (AST)** defined in `ast.rs`.
3.  **Type Checker (`typechecker.rs`)**: Walks the AST to ensure variables are defined and types match (e.g., you can't add a `bool` to an `int`).
4.  **Backend**:
    - **Interpreter (`interpreter.rs`)**: A recursive walker that evaluates the AST in real-time.
    - **Compiler (`compiler.rs` & `codegen.rs`)**: Flattens the AST into **Three-Address Code (IR)** and emits **x86_64 Assembly**.

## 💻 Language Syntax

```rust
fn add(a: int, b: int) -> int {
    return a + b
}

fn main() -> int {
    return add(3, 5) * 10
}
```

## 🛠️ Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- (Optional) `gcc` or `nasm` for assembling generated `.s` files.

### Installation
```bash
git clone <your-repo-url>
cd dinoe-compiler
cargo build --release
```

### Running the Compiler

**Interpret a file:**
```bash
cargo run -- example.dinoe
```

**Generate x86_64 Assembly:**
```bash
cargo run -- example.dinoe --emit-asm
```

**Debug Mode (See Tokens, AST, and IR):**
```bash
cargo run -- example.dinoe --dump
```

## 🧪 Testing

The project includes a robust suite of 23 integration tests covering every stage of the pipeline.

```bash
cargo test
```

## 📚 What I Learned Building This
- **Rust Fundamentals**: Ownership, pattern matching, recursive types with `Box<T>`, and modular design.
- **Language Grammar**: Implementing operator precedence (PEMDAS) using recursive descent.
- **Low-Level Systems**: Understanding CPU registers, the stack frame (prologue/epilogue), and the System V calling convention for function calls.
- **Compiler Design**: Learning how professional compilers like Rust and Clang use Intermediate Representations (IR) to bridge high-level logic and hardware.

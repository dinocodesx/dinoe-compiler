use crate::ast::{Expr, Op, Stmt};
use crate::ir::{Instruction, IRFunction, Operand};

/// The Compiler is responsible for flattening the recursive AST structure
/// into a linear Intermediate Representation (IR) called Three-Address Code (TAC).
pub struct Compiler {
    /// Counter to generate unique virtual register identifiers.
    reg_count: usize,
    /// Accumulates instructions for the current function being compiled.
    instructions: Vec<Instruction>,
}

impl Compiler {
    /// Creates a new Compiler instance.
    pub fn new() -> Self {
        Self {
            reg_count: 0,
            instructions: Vec::new(),
        }
    }

    /// Generates the next available virtual register index.
    fn next_reg(&mut self) -> usize {
        self.reg_count += 1;
        self.reg_count
    }

    /// Entry point: Compiles a list of AST statements (function definitions) into IR functions.
    pub fn compile_program(&mut self, program: Vec<Stmt>) -> Vec<IRFunction> {
        let mut ir_functions = Vec::new();
        for stmt in program {
            if let Stmt::FnDef { name, params, body, .. } = stmt {
                ir_functions.push(self.compile_function(name, params, body));
            }
        }
        ir_functions
    }

    /// Compiles a single function's AST body into a linear list of IR instructions.
    fn compile_function(&mut self, name: String, params: Vec<(String, crate::ast::Type)>, body: Vec<Stmt>) -> IRFunction {
        // Reset state for each new function
        self.reg_count = 0;
        self.instructions = Vec::new();

        let param_names = params.into_iter().map(|(n, _)| n).collect();

        for stmt in body {
            self.compile_stmt(stmt);
        }

        IRFunction {
            name,
            params: param_names,
            instructions: self.instructions.clone(),
        }
    }

    /// Translates an AST statement into one or more IR instructions.
    fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Return(expr) => {
                // Evaluate the expression and then emit a Ret instruction
                let op = self.compile_expr(expr);
                self.instructions.push(Instruction::Ret(op));
            }
            Stmt::Expr(expr) => {
                // Standalone expressions are compiled but their results may be ignored
                self.compile_expr(expr);
            }
            Stmt::FnDef { .. } => panic!("Nested functions not supported in IR generation"),
        }
    }

    /// Recursively flattens an AST expression into a series of IR instructions.
    /// Returns the Operand (VReg, Imm, or Var) representing the result of the expression.
    fn compile_expr(&mut self, expr: Expr) -> Operand {
        match expr {
            Expr::Number(n) => Operand::Imm(n),
            Expr::Bool(b) => Operand::Imm(if b { 1 } else { 0 }),
            Expr::Ident(s) => Operand::Var(s),
            Expr::BinOp { op, lhs, rhs } => {
                // 1. Compile both sides
                let l = self.compile_expr(*lhs);
                let r = self.compile_expr(*rhs);
                
                // 2. Allocate a virtual register for the result
                let dest = self.next_reg();
                
                // 3. Emit the linear instruction
                let instr = match op {
                    Op::Add => Instruction::Add { dest, lhs: l, rhs: r },
                    Op::Sub => Instruction::Sub { dest, lhs: l, rhs: r },
                    Op::Mul => Instruction::Mul { dest, lhs: l, rhs: r },
                    Op::Div => Instruction::Div { dest, lhs: l, rhs: r },
                };
                
                self.instructions.push(instr);
                Operand::Reg(dest)
            }
            Expr::Call { name, args } => {
                // 1. Compile all arguments into operands
                let mut arg_ops = Vec::new();
                for arg in args {
                    arg_ops.push(self.compile_expr(arg));
                }
                
                // 2. Allocate a virtual register for the function return value
                let dest = self.next_reg();
                self.instructions.push(Instruction::Call { dest, name, args: arg_ops });
                Operand::Reg(dest)
            }
        }
    }
}

use crate::ir::{Instruction, IRFunction, Operand};
use std::collections::HashMap;

/// The Codegen (Code Generator) is responsible for translating the linear IR
/// into platform-specific x86_64 assembly code.
pub struct Codegen {
    /// The accumulated assembly code as a string.
    output: String,
}

impl Codegen {
    /// Creates a new Codegen instance.
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    /// Entry point: Translates a list of IR functions into a single assembly string.
    pub fn emit_program(&mut self, functions: Vec<IRFunction>) -> String {
        // macOS requires an underscore prefix for global symbols (e.g., _main)
        let prefix = if cfg!(target_os = "macos") { "_" } else { "" };

        for func in functions {
            self.emit_function(func, prefix);
        }
        self.output.clone()
    }

    /// Generates assembly for a single function.
    fn emit_function(&mut self, func: IRFunction, prefix: &str) {
        // 1. Declare the symbol as global and emit the label
        self.output.push_str(&format!(".global {}{}\n", prefix, func.name));
        self.output.push_str(&format!("{}{}:\n", prefix, func.name));

        // 2. Function Prologue: Set up the stack frame
        // Save the caller's base pointer and set our base pointer to the current stack pointer
        self.output.push_str("    push rbp\n");
        self.output.push_str("    mov rbp, rsp\n");
        
        // Reserve 256 bytes on the stack for virtual registers and local variables
        // This is a simple "fixed-size" stack allocation strategy.
        self.output.push_str("    sub rsp, 256\n");

        // 3. Map parameter names to stack offsets and load them from registers
        // Following the System V ABI calling convention (Linux/macOS)
        let arg_regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
        let mut var_offsets = HashMap::new();

        for (i, param) in func.params.iter().enumerate() {
            let offset = (i + 1) * 8; // Each parameter takes 8 bytes (64-bit)
            var_offsets.insert(param.clone(), offset);
            if i < arg_regs.len() {
                // Move the argument from the ABI register to our local stack space
                self.output.push_str(&format!("    mov [rbp - {}], {}\n", offset, arg_regs[i]));
            }
        }

        // 4. Emit assembly for each linear IR instruction
        for instr in func.instructions {
            self.emit_instruction(instr, &var_offsets);
        }

        // 5. Function Epilogue: Clean up the stack frame
        self.output.push_str("    mov rsp, rbp\n");
        self.output.push_str("    pop rbp\n");
        self.output.push_str("    ret\n\n");
    }

    /// Translates a single IR instruction into one or more x86_64 assembly instructions.
    fn emit_instruction(&mut self, instr: Instruction, var_offsets: &HashMap<String, usize>) {
        match instr {
            Instruction::Load { dest, src } => {
                self.load_operand("rax", src, var_offsets);
                self.output.push_str(&format!("    mov [rbp - {}], rax\n", dest * 8 + 64));
            }
            Instruction::Add { dest, lhs, rhs } => {
                self.load_operand("rax", lhs, var_offsets);
                self.load_operand("rbx", rhs, var_offsets);
                self.output.push_str("    add rax, rbx\n");
                self.output.push_str(&format!("    mov [rbp - {}], rax\n", dest * 8 + 64));
            }
            Instruction::Sub { dest, lhs, rhs } => {
                self.load_operand("rax", lhs, var_offsets);
                self.load_operand("rbx", rhs, var_offsets);
                self.output.push_str("    sub rax, rbx\n");
                self.output.push_str(&format!("    mov [rbp - {}], rax\n", dest * 8 + 64));
            }
            Instruction::Mul { dest, lhs, rhs } => {
                self.load_operand("rax", lhs, var_offsets);
                self.load_operand("rbx", rhs, var_offsets);
                self.output.push_str("    imul rax, rbx\n");
                self.output.push_str(&format!("    mov [rbp - {}], rax\n", dest * 8 + 64));
            }
            Instruction::Div { dest, lhs, rhs } => {
                self.load_operand("rax", lhs, var_offsets);
                self.load_operand("rbx", rhs, var_offsets);
                self.output.push_str("    cqo\n"); // Sign-extend RAX into RDX:RAX for division
                self.output.push_str("    idiv rbx\n");
                self.output.push_str(&format!("    mov [rbp - {}], rax\n", dest * 8 + 64));
            }
            Instruction::Call { dest, name, args } => {
                // Prepare arguments in registers according to System V ABI
                let arg_regs = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
                for (i, arg) in args.iter().enumerate() {
                    if i < arg_regs.len() {
                        self.load_operand(arg_regs[i], arg.clone(), var_offsets);
                    }
                }
                let prefix = if cfg!(target_os = "macos") { "_" } else { "" };
                self.output.push_str(&format!("    call {}{}\n", prefix, name));
                // Move result from RAX to the destination virtual register
                self.output.push_str(&format!("    mov [rbp - {}], rax\n", dest * 8 + 64));
            }
            Instruction::Ret(op) => {
                // Move the return value into RAX as per standard calling convention
                self.load_operand("rax", op, var_offsets);
            }
        }
    }

    /// Helper to load an IR operand (Immediate, Virtual Register, or Variable) into a physical CPU register.
    fn load_operand(&mut self, reg: &str, op: Operand, var_offsets: &HashMap<String, usize>) {
        match op {
            Operand::Imm(n) => {
                self.output.push_str(&format!("    mov {}, {}\n", reg, n));
            }
            Operand::Reg(n) => {
                // Virtual registers are mapped to stack offsets starting after parameters (at 64 bytes)
                self.output.push_str(&format!("    mov {}, [rbp - {}]\n", reg, n * 8 + 64));
            }
            Operand::Var(s) => {
                // Variables (parameters) are mapped to stack offsets at the beginning of the frame
                let offset = var_offsets.get(&s).expect("Undefined variable in codegen");
                self.output.push_str(&format!("    mov {}, [rbp - {}]\n", reg, offset));
            }
        }
    }
}

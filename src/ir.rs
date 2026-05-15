/// Operands for IR instructions.
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    /// A virtual register identifier (e.g., %1, %2).
    Reg(usize),
    /// A constant integer value.
    Imm(i64),
    /// A variable name (identifier).
    Var(String),
}

/// A single instruction in our Three-Address Code (TAC) IR.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// Load an operand into a virtual register.
    Load { dest: usize, src: Operand },
    /// Arithmetic addition: dest = lhs + rhs.
    Add { dest: usize, lhs: Operand, rhs: Operand },
    /// Arithmetic subtraction: dest = lhs - rhs.
    Sub { dest: usize, lhs: Operand, rhs: Operand },
    /// Arithmetic multiplication: dest = lhs * rhs.
    Mul { dest: usize, lhs: Operand, rhs: Operand },
    /// Arithmetic division: dest = lhs / rhs.
    Div { dest: usize, lhs: Operand, rhs: Operand },
    /// Function call: dest = name(args).
    Call { dest: usize, name: String, args: Vec<Operand> },
    /// Return a value from a function.
    Ret(Operand),
}

/// A function in IR form, containing its name and a linear list of instructions.
#[derive(Debug, Clone, PartialEq)]
pub struct IRFunction {
    pub name: String,
    pub params: Vec<String>,
    pub instructions: Vec<Instruction>,
}

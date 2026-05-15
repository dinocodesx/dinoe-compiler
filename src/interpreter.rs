use std::collections::HashMap;
use crate::ast::{Expr, Op, Stmt};

/// Represents the values that can exist at runtime in Dinoe.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
}

/// The Interpreter walks the AST and executes the program.
pub struct Interpreter {
    /// Stores function definitions by name.
    functions: HashMap<String, Stmt>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    /// Entry point: executes the 'main' function of the program.
    pub fn execute_program(&mut self, program: Vec<Stmt>) -> Result<Value, String> {
        // Register all functions.
        for stmt in program {
            if let Stmt::FnDef { ref name, .. } = stmt {
                self.functions.insert(name.clone(), stmt);
            }
        }

        // Call the main function.
        self.call_function("main", vec![])
    }

    /// Handles a function call, creating a new scope for local variables.
    fn call_function(&self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        let func = self.functions.get(name).ok_or_else(|| format!("Function '{}' not found", name))?;

        if let Stmt::FnDef { params, body, .. } = func {
            if params.len() != args.len() {
                return Err(format!("Function '{}' expected {} arguments, got {}", name, params.len(), args.len()));
            }

            // Create a local environment for this function call.
            let mut env = HashMap::new();
            for (i, (param_name, _)) in params.iter().enumerate() {
                env.insert(param_name.clone(), args[i].clone());
            }

            // Execute statements in the body.
            for stmt in body {
                match stmt {
                    Stmt::Return(expr) => {
                        return self.eval_expr(expr, &env);
                    }
                    Stmt::Expr(expr) => {
                        self.eval_expr(expr, &env)?;
                    }
                    _ => return Err("Unexpected statement in function body".to_string()),
                }
            }

            Err(format!("Function '{}' ended without a return statement", name))
        } else {
            Err("Expected function definition".to_string())
        }
    }

    /// Evaluates an expression to a Value.
    fn eval_expr(&self, expr: &Expr, env: &HashMap<String, Value>) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Int(*n)),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Ident(name) => env
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable: {}", name)),
            Expr::BinOp { op, lhs, rhs } => {
                let left = self.eval_expr(lhs, env)?;
                let right = self.eval_expr(rhs, env)?;

                match (left, right) {
                    (Value::Int(a), Value::Int(b)) => match op {
                        Op::Add => Ok(Value::Int(a + b)),
                        Op::Sub => Ok(Value::Int(a - b)),
                        Op::Mul => Ok(Value::Int(a * b)),
                        Op::Div => {
                            if b == 0 {
                                Err("Division by zero".to_string())
                            } else {
                                Ok(Value::Int(a / b))
                            }
                        }
                    },
                    _ => Err("Invalid operands for binary operator".to_string()),
                }
            }
            Expr::Call { name, args } => {
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.eval_expr(arg, env)?);
                }
                self.call_function(name, arg_values)
            }
        }
    }
}

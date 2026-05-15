use std::collections::HashMap;
use crate::ast::{Expr, Op, Stmt, Type};

/// The TypeChecker ensures that the program is logically sound regarding types.
pub struct TypeChecker {
    /// Maps function names to their (parameter types, return type).
    functions: HashMap<String, (Vec<Type>, Type)>,
    /// Maps variable names to their types within the current function scope.
    variables: HashMap<String, Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    /// Entry point: checks a list of function definitions.
    pub fn check_program(&mut self, program: &[Stmt]) -> Result<(), String> {
        // First pass: Register all function signatures so they can call each other.
        for stmt in program {
            if let Stmt::FnDef { name, params, return_type, .. } = stmt {
                let param_types = params.iter().map(|(_, t)| t.clone()).collect();
                if self.functions.insert(name.clone(), (param_types, return_type.clone())).is_some() {
                    return Err(format!("Duplicate function definition: {}", name));
                }
            }
        }

        // Second pass: Check each function body.
        for stmt in program {
            if let Stmt::FnDef { name, params, return_type, body } = stmt {
                self.check_fn(name, params, return_type, body)?;
            }
        }

        Ok(())
    }

    /// Checks a single function definition.
    fn check_fn(
        &mut self,
        _name: &str,
        params: &[(String, Type)],
        expected_return: &Type,
        body: &[Stmt],
    ) -> Result<(), String> {
        // Clear local variables and add parameters to scope.
        self.variables.clear();
        for (param_name, param_type) in params {
            self.variables.insert(param_name.clone(), param_type.clone());
        }

        let mut found_return = false;
        for stmt in body {
            if self.check_stmt(stmt, expected_return)? {
                found_return = true;
            }
        }

        if !found_return {
            return Err(format!("Function '{}' must end with a return statement", _name));
        }

        Ok(())
    }

    /// Checks a statement. Returns true if it's a return statement.
    fn check_stmt(&mut self, stmt: &Stmt, expected_return: &Type) -> Result<bool, String> {
        match stmt {
            Stmt::Return(expr) => {
                let actual_type = self.check_expr(expr)?;
                if &actual_type != expected_return {
                    return Err(format!(
                        "Type mismatch: expected return type {:?}, but got {:?}",
                        expected_return, actual_type
                    ));
                }
                Ok(true)
            }
            Stmt::Expr(expr) => {
                self.check_expr(expr)?;
                Ok(false)
            }
            Stmt::FnDef { .. } => Err("Nested function definitions are not supported".to_string()),
        }
    }

    /// Infers and validates the type of an expression.
    pub fn check_expr(&self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::Number(_) => Ok(Type::Int),
            Expr::Bool(_) => Ok(Type::Bool),
            Expr::Ident(name) => self
                .variables
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Undefined variable: {}", name)),
            Expr::BinOp { op, lhs, rhs } => {
                let left_type = self.check_expr(lhs)?;
                let right_type = self.check_expr(rhs)?;

                match op {
                    Op::Add | Op::Sub | Op::Mul | Op::Div => {
                        if left_type == Type::Int && right_type == Type::Int {
                            Ok(Type::Int)
                        } else {
                            Err("Arithmetic operators require integer operands".to_string())
                        }
                    }
                }
            }
            Expr::Call { name, args } => {
                let (param_types, return_type) = self
                    .functions
                    .get(name)
                    .cloned()
                    .ok_or_else(|| format!("Undefined function: {}", name))?;

                if args.len() != param_types.len() {
                    return Err(format!(
                        "Function '{}' expected {} arguments, but got {}",
                        name,
                        param_types.len(),
                        args.len()
                    ));
                }

                for (arg, expected_type) in args.iter().zip(param_types.iter()) {
                    let actual_type = self.check_expr(arg)?;
                    if &actual_type != expected_type {
                        return Err(format!(
                            "Argument type mismatch for function '{}': expected {:?}, got {:?}",
                            name, expected_type, actual_type
                        ));
                    }
                }

                Ok(return_type)
            }
        }
    }
}

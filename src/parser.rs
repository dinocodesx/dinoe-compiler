use crate::lexer::Token;
use crate::ast::{Expr, Op, Stmt, Type};

/// The Parser takes a stream of Tokens and builds an Abstract Syntax Tree (AST).
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Returns the current token without advancing.
    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    /// Advances the position and returns the previous token.
    fn advance(&mut self) -> &Token {
        let tok = self.peek();
        if tok != &Token::Eof {
            self.pos += 1;
        }
        self.tokens.get(self.pos - 1).unwrap_or(&Token::Eof)
    }

    /// Checks if the current token matches the expected type.
    fn check(&self, expected: &Token) -> bool {
        // We only check variant equality for symbols, not content for Ident/Number
        std::mem::discriminant(self.peek()) == std::mem::discriminant(expected)
    }

    /// If the current token matches, advance and return true.
    fn match_token(&mut self, expected: &Token) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Consumes the expected token or returns an error.
    fn consume(&mut self, expected: Token, message: &str) -> Result<Token, String> {
        if self.check(&expected) {
            Ok(self.advance().clone())
        } else {
            Err(format!("{}: Expected {:?}, found {:?}", message, expected, self.peek()))
        }
    }

    /// Parses a sequence of function definitions until EOF.
    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut program = Vec::new();
        while !self.check(&Token::Eof) {
            program.push(self.parse_fn_def()?);
        }
        Ok(program)
    }

    /// Parses a function definition: fn name(params) -> type { body }
    fn parse_fn_def(&mut self) -> Result<Stmt, String> {
        self.consume(Token::Fn, "Expect 'fn' at start of function definition")?;
        
        let name = if let Token::Ident(name) = self.advance().clone() {
            name
        } else {
            return Err("Expect function name".to_string());
        };

        self.consume(Token::LParen, "Expect '(' after function name")?;
        
        let mut params = Vec::new();
        if !self.check(&Token::RParen) {
            loop {
                let param_name = if let Token::Ident(name) = self.advance().clone() {
                    name
                } else {
                    return Err("Expect parameter name".to_string());
                };

                self.consume(Token::Colon, "Expect ':' after parameter name")?;
                let param_type = self.parse_type()?;
                params.push((param_name, param_type));

                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
        }
        
        self.consume(Token::RParen, "Expect ')' after parameters")?;
        self.consume(Token::Arrow, "Expect '->' before return type")?;
        
        let return_type = self.parse_type()?;
        
        self.consume(Token::LBrace, "Expect '{' before function body")?;
        
        let mut body = Vec::new();
        while !self.check(&Token::RBrace) && !self.check(&Token::Eof) {
            body.push(self.parse_stmt()?);
        }
        
        self.consume(Token::RBrace, "Expect '}' after function body")?;
        
        Ok(Stmt::FnDef {
            name,
            params,
            return_type,
            body,
        })
    }

    /// Parses a single statement.
    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        // For simplicity, we assume an identifier starting a line is an expression
        // unless it's a specific keyword like 'return'.
        if let Token::Ident(s) = self.peek() {
            if s == "return" {
                self.advance(); // consume "return"
                let expr = self.parse_expr()?;
                return Ok(Stmt::Return(expr));
            }
        }

        let expr = self.parse_expr()?;
        Ok(Stmt::Expr(expr))
    }

    /// Parses a data type (int or bool).
    fn parse_type(&mut self) -> Result<Type, String> {
        match self.advance() {
            Token::IntType => Ok(Type::Int),
            Token::BoolType => Ok(Type::Bool),
            other => Err(format!("Expected type (int or bool), found {:?}", other)),
        }
    }

    /// Main entry point for parsing an expression.
    /// Handles addition and subtraction (lowest precedence).
    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;

        while self.check(&Token::Plus) || self.check(&Token::Minus) {
            let op = match self.advance() {
                Token::Plus => Op::Add,
                Token::Minus => Op::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            left = Expr::BinOp {
                op,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Handles multiplication and division (medium precedence).
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        while self.check(&Token::Star) || self.check(&Token::Slash) {
            let op = match self.advance() {
                Token::Star => Op::Mul,
                Token::Slash => Op::Div,
                _ => unreachable!(),
            };
            let right = self.parse_primary()?;
            left = Expr::BinOp {
                op,
                lhs: Box::new(left),
                rhs: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Handles highest precedence: numbers, booleans, and parenthesized expressions.
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance().clone() {
            Token::Number(n) => Ok(Expr::Number(n)),
            Token::True => Ok(Expr::Bool(true)),
            Token::False => Ok(Expr::Bool(false)),
            Token::Ident(s) => {
                // Check if this is a function call
                if self.match_token(&Token::LParen) {
                    let mut args = Vec::new();
                    if !self.check(&Token::RParen) {
                        loop {
                            args.push(self.parse_expr()?);
                            if !self.match_token(&Token::Comma) {
                                break;
                            }
                        }
                    }
                    self.consume(Token::RParen, "Expect ')' after arguments")?;
                    Ok(Expr::Call { name: s, args })
                } else {
                    Ok(Expr::Ident(s))
                }
            }
            Token::LParen => {
                let expr = self.parse_expr()?;
                if !self.match_token(&Token::RParen) {
                    return Err("Expected ')' after expression".to_string());
                }
                Ok(expr)
            }
            other => Err(format!("Unexpected token: {:?}", other)),
        }
    }
}

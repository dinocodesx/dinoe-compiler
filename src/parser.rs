use crate::lexer::Token;
use crate::ast::{Expr, Op};

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
            Token::Ident(s) => Ok(Expr::Ident(s)),
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

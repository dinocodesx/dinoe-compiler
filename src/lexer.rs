/// Represents the basic units of the .dinoe language.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Fn,            // 'fn' keyword
    Ident(String), // e.g., 'add', 'main', 'x'
    IntType,       // 'int' keyword
    BoolType,      // 'bool' keyword
    Number(i64),   // e.g., 42
    True,          // 'true' keyword
    False,         // 'false' keyword
    Plus,          // '+'
    Minus,         // '-'
    Star,          // '*'
    Slash,         // '/'
    LParen,        // '('
    RParen,        // ')'
    LBrace,        // '{'
    RBrace,        // '}'
    Colon,         // ':'
    Comma,         // ','
    Arrow,         // '->'
    Eof,           // End of file
}

/// The Lexer transforms a string of source code into a sequence of Tokens.
pub struct Lexer {
    source: Vec<char>,
    pos: usize,
}

impl Lexer {
    /// Creates a new Lexer from a source string.
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            pos: 0,
        }
    }

    /// Returns the character at the current position without advancing.
    fn peek(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

    /// Returns the character at the current position and advances to the next.
    fn advance(&mut self) -> Option<char> {
        let res = self.peek();
        if res.is_some() {
            self.pos += 1;
        }
        res
    }

    /// Skips over any whitespace characters (spaces, tabs, newlines).
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Primary entry point: finds and returns the next Token in the stream.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let c = match self.advance() {
            Some(c) => c,
            None => return Token::Eof,
        };

        match c {
            '+' => Token::Plus,
            '-' => {
                // Check if this is the start of an arrow '->'
                if self.peek() == Some('>') {
                    self.advance(); // consume '>'
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            '*' => Token::Star,
            '/' => Token::Slash,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ':' => Token::Colon,
            ',' => Token::Comma,
            // Handle multi-character sequences
            _ if c.is_ascii_digit() => self.lex_number(c),
            _ if c.is_alphabetic() => self.lex_identifier(c),
            _ => panic!("Unexpected character: {}", c),
        }
    }

    /// Collects consecutive digits into a Number token.
    fn lex_number(&mut self, first_digit: char) -> Token {
        let mut num_str = String::from(first_digit);
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                num_str.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        Token::Number(num_str.parse().unwrap())
    }

    /// Collects alphanumeric characters into an Identifier or Keyword token.
    fn lex_identifier(&mut self, first_char: char) -> Token {
        let mut ident = String::from(first_char);
        while let Some(c) = self.peek() {
            // Identifiers can contain letters, numbers, and underscores
            if c.is_alphanumeric() || c == '_' {
                ident.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        // Match against keywords, otherwise it's a plain identifier
        match ident.as_str() {
            "fn" => Token::Fn,
            "int" => Token::IntType,
            "bool" => Token::BoolType,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(ident),
        }
    }

    /// Utility to collect all tokens until EOF into a Vector.
    pub fn lex_all(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let is_eof = tok == Token::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        tokens
    }
}

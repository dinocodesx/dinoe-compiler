use dinoe_compiler::lexer::{Lexer, Token};

#[test]
fn test_basic_symbols() {
    let mut lexer = Lexer::new("+ - * / ( ) { } : , ->");
    let tokens = lexer.lex_all();
    assert_eq!(tokens, vec![
        Token::Plus, Token::Minus, Token::Star, Token::Slash,
        Token::LParen, Token::RParen, Token::LBrace, Token::RBrace,
        Token::Colon, Token::Comma, Token::Arrow, Token::Eof
    ]);
}

#[test]
fn test_keywords_and_numbers() {
    let mut lexer = Lexer::new("fn main 123 int bool true false");
    let tokens = lexer.lex_all();
    assert_eq!(tokens, vec![
        Token::Fn,
        Token::Ident("main".to_string()),
        Token::Number(123),
        Token::IntType,
        Token::BoolType,
        Token::True,
        Token::False,
        Token::Eof
    ]);
}

#[test]
fn test_example_program() {
    let input = "
        fn add(a: int, b: int) -> int {
            a + b
        }
    ";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex_all();
    assert_eq!(tokens, vec![
        Token::Fn, Token::Ident("add".to_string()), Token::LParen,
        Token::Ident("a".to_string()), Token::Colon, Token::IntType, Token::Comma,
        Token::Ident("b".to_string()), Token::Colon, Token::IntType, Token::RParen,
        Token::Arrow, Token::IntType, Token::LBrace,
        Token::Ident("a".to_string()), Token::Plus, Token::Ident("b".to_string()),
        Token::RBrace, Token::Eof
    ]);
}

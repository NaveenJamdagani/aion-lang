use aion::lexer::{Lexer, Token};

#[test]
fn test_next_token() {
    let input = "let x = 10;";

    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token::Let);
    assert_eq!(lexer.next_token(), Token::Ident("x".to_string()));
    assert_eq!(lexer.next_token(), Token::Assign);
    assert_eq!(lexer.next_token(), Token::Number("10".to_string()));
    assert_eq!(lexer.next_token(), Token::Semicolon);
}

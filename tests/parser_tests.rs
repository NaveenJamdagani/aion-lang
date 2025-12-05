use aion::lexer::Lexer;
use aion::parser::Parser;
use aion::ast::{Statement};

#[test]
fn test_let_statements() {
    let input = "
        let x = 10;
        let y = 20;
    ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    assert_eq!(program.len(), 2);

    match &program[0] {
        Statement::LetStatement { name, .. } => assert_eq!(name, "x"),
        _ => panic!("expected let statement"),
    }

    match &program[1] {
        Statement::LetStatement { name, .. } => assert_eq!(name, "y"),
        _ => panic!("expected let statement"),
    }
}

use aion::lexer::Lexer;
use aion::parser::Parser;
use aion::ast::{Statement, Expression};

#[test]
fn test_infix_expression() {
    let input = "let x = 5 + 3 * 2;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    match &program[0] {
        Statement::LetStatement { name, value } => {
            assert_eq!(name, "x");
            println!("{:#?}", value);
        }
        _ => panic!("expected let statement"),
    };
}

use aion::lexer::Lexer;
use aion::parser::Parser;
use aion::interpreter::{eval_program, Environment, Object};

#[test]
fn test_let_and_ident() {
    let input = "
        let a = 5;
        let b = a + 5;
        b;
    ";

    let mut parser = Parser::new(Lexer::new(input));
    let program = parser.parse_program();

    let mut env = Environment::new();
    let result = eval_program(program, &mut env);

    assert_eq!(result, Object::Integer(10));
}

#[test]
fn test_arithmetic_and_comparison() {
    let cases = vec![
        ("5 + 5;", 10),
        ("2 * 3 + 1;", 7),
        ("5 + 3 * 2;", 11),
        ("5 < 10;", 1),   // we'll interpret boolean true as Object::Boolean(true)
    ];

    for (input, expected_int) in cases {
        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse_program();
        let mut env = Environment::new();
        let result = eval_program(program, &mut env);

        match result {
            Object::Integer(i) => assert_eq!(i, expected_int),
            Object::Boolean(b) => {
                if expected_int == 1 {
                    assert!(b);
                } else {
                    panic!("expected integer, found boolean");
                }
            }
            _ => panic!("unexpected object"),
        }
    }
}

// Note: For the boolean case 5 < 10; we check Object::Boolean(true); the test uses expected_int==1 as a simple guard.
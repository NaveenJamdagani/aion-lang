use aion::lexer::Lexer;
use aion::parser::Parser;
use aion::interpreter::{eval_program, Environment};
use std::io::{self, Write};

fn main() {
    let mut env = Environment::new();
    let mut input = String::new();

    println!("AION REPL — Type Ctrl+C to exit");

    loop {
        print!("aion> ");
        io::stdout().flush().unwrap();

        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input");
            continue;
        }

        if input.trim().is_empty() {
            continue;
        }

        let mut parser = Parser::new(Lexer::new(&input));
        let program = parser.parse_program();

        let result = eval_program(program, &mut env);
        println!("{}", result);
    }
}

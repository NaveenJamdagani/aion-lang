use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    LetStatement {
        name: String,
        value: Expression,
    },
    ReturnStatement {
        value: Expression,
    },
    ExpressionStatement(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    NumberLiteral(i64),

    Prefix {
        operator: String,
        right: Box<Expression>,
    },

    Infix {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
}

impl Expression {
    pub fn print(&self, indent: usize) {
        let pad = " ".repeat(indent);

        match self {
            Expression::Identifier(name) => {
                println!("{}{{ \"type\": \"Identifier\", \"name\": \"{}\" }}", pad, name);
            }

            Expression::NumberLiteral(value) => {
                println!("{}{{ \"type\": \"NumberLiteral\", \"value\": {} }}", pad, value);
            }

            Expression::Prefix { operator, right } => {
                println!("{}{{ \"type\": \"Prefix\", \"operator\": \"{}\",", pad, operator);
                println!("{}  \"right\":", pad);
                right.print(indent + 4);
                println!("{}}}", pad);
            }

            Expression::Infix { left, operator, right } => {
                println!("{}{{ \"type\": \"Infix\", \"operator\": \"{}\",", pad, operator);
                println!("{}  \"left\":", pad);
                left.print(indent + 4);
                println!("{}  \"right\":", pad);
                right.print(indent + 4);
                println!("{}}}", pad);
            }
        }
    }
}

// AION JSON AST Printer

// 1. Every Expression node has a print(indent) method.
// 2. It prints a JSON-like structure with visual indentation.
// 3. Helps debug:
//    - precedence issues
//    - prefix/infix structure
//    - nested expressions
// 4. Very important for interpreter development.

impl Statement {
    pub fn print(&self) {
        match self {
            Statement::LetStatement { name, value } => {
                println!("{{");
                println!("  \"type\": \"LetStatement\",");
                println!("  \"name\": \"{}\",", name);
                println!("  \"value\":");
                value.print(4);
                println!("}}");
            }

            Statement::ReturnStatement { value } => {
                println!("{{");
                println!("  \"type\": \"ReturnStatement\",");
                println!("  \"value\":");
                value.print(4);
                println!("}}");
            }

            Statement::ExpressionStatement(expr) => {
                println!("{{");
                println!("  \"type\": \"ExpressionStatement\",");
                println!("  \"value\":");
                expr.print(4);
                println!("}}");
            }
        }
    }
}




// ✔ Prefix: -x, !true
// ✔ Infix: x + y, a * b

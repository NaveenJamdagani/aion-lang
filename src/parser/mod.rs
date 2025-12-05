use crate::lexer::{Lexer, Token};
use crate::ast::{Statement, Expression};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    next_token: Token,
}

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    LOWEST,
    EQUALS,      // == 
    LESSGREATER, // < or >
    SUM,         // + or -
    PRODUCT,     // * or /
    PREFIX,      // -X or !X
    CALL,        // function calls: add(x, y)
}


impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        let next = lexer.next_token();

        Parser {
            lexer,
            current_token: current,
            next_token: next,
        }
    }

    fn advance_tokens(&mut self) {
        self.current_token = std::mem::replace(&mut self.next_token, self.lexer.next_token());
    }

    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut statements = vec![];

        while self.current_token != Token::EOF {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.advance_tokens();
        }

        statements
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }


    fn parse_let_statement(&mut self) -> Option<Statement> {
        // expect identifier next
        let name = if let Token::Ident(ref ident) = self.next_token {
            ident.clone()
        } else {
            return None;
        };

        self.advance_tokens(); // move to identifier
        self.advance_tokens(); // move to '='

        // Now parse the expression after '='
        self.advance_tokens(); // move to first token of expression
        let value = self.parse_expression(Precedence::LOWEST)?;

        // optional semicolon
        if self.next_token == Token::Semicolon {
            self.advance_tokens();
        }

        Some(Statement::LetStatement { name, value })
    }


    fn get_precedence(token: &Token) -> Precedence {
        match token {
            Token::Equal | Token::NotEqual => Precedence::EQUALS,
            Token::LessThan | Token::GreaterThan => Precedence::LESSGREATER,
            Token::Plus | Token::Minus => Precedence::SUM,
            Token::Asterisk | Token::Slash => Precedence::PRODUCT,
            _ => Precedence::LOWEST,
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left_exp = match self.current_token.clone() {
            Token::Ident(ident) => {
                Some(Expression::Identifier(ident))
            }
            Token::Number(num) => {
                Some(Expression::NumberLiteral(num.parse::<i64>().unwrap()))
            }
            Token::Bang | Token::Minus => {
                self.parse_prefix_expression()
            }
            Token::LParen => {
                self.advance_tokens(); // skip '('
                let expr = self.parse_expression(Precedence::LOWEST);
                self.expect_rparen();
                expr
            }
            _ => return None,
        };

        while self.next_token != Token::Semicolon 
            && precedence < Self::get_precedence(&self.next_token)
        {
            match self.next_token.clone() {
                Token::Plus | Token::Minus | Token::Asterisk | Token::Slash | Token::LessThan | Token::GreaterThan => {
                    self.advance_tokens();
                    left_exp = Some(self.parse_infix_expression(left_exp.unwrap()));
                }
                _ => return left_exp,
            }
        }

        left_exp
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = match &self.current_token {
            Token::Bang => "!",
            Token::Minus => "-",
            _ => return None,
        }.to_string();

        self.advance_tokens();

        let right = self.parse_expression(Precedence::PREFIX)?;

        Some(Expression::Prefix {
            operator,
            right: Box::new(right),
        })
    }
    fn parse_infix_expression(&mut self, left: Expression) -> Expression {
        let operator = match &self.current_token {
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Asterisk => "*",
            Token::Slash => "/",
            Token::LessThan => "<",
            Token::GreaterThan => ">",
            _ => "",
        }.to_string();

        let precedence = Self::get_precedence(&self.current_token);

        self.advance_tokens();

        let right = self.parse_expression(precedence).unwrap();

        Expression::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
    fn expect_rparen(&mut self) {
        if self.next_token == Token::RParen {
            self.advance_tokens();
        } else {
            panic!("Expected ')' but found {:?}", self.next_token);
        }
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression(Precedence::LOWEST)?;

        // If next token is a semicolon, consume it
        if self.next_token == Token::Semicolon {
            self.advance_tokens();
        }

        Some(Statement::ExpressionStatement(expr))
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.advance_tokens(); // move to the expression

        let value = self.parse_expression(Precedence::LOWEST)?;

        if self.next_token == Token::Semicolon {
            self.advance_tokens();
        }

        Some(Statement::ReturnStatement { value })
    }

}

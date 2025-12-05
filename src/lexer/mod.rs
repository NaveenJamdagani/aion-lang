#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    EOF,

    // Identifiers + literals
    Ident(String),
    Number(String),

    // Keywords
    Let,
    Fn,
    Return,
    True,
    False,
    If,
    Else,

    // Operators
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,
    LessThan,
    GreaterThan,
    Equal,
    NotEqual,


    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,      
    read_position: usize, 
    ch: char,             
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();  
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }
        self.input[start..self.position].iter().collect()
    }

    fn read_number(&mut self) -> String {
        let start = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        self.input[start..self.position].iter().collect()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    Token::Equal
                } else {
                    self.read_char();
                    Token::Assign
                }
            }
            '+' => { self.read_char(); Token::Plus }
            '-' => { self.read_char(); Token::Minus }
            '*' => { self.read_char(); Token::Asterisk }
            '/' => { self.read_char(); Token::Slash }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    self.read_char();
                    Token::NotEqual
                } else {
                    self.read_char();
                    Token::Bang
                }
            }
            '<' => { self.read_char(); Token::LessThan }
            '>' => { self.read_char(); Token::GreaterThan }
            '(' => { self.read_char(); Token::LParen }
            ')' => { self.read_char(); Token::RParen }
            '{' => { self.read_char(); Token::LBrace }
            '}' => { self.read_char(); Token::RBrace }
            ',' => { self.read_char(); Token::Comma }
            ';' => { self.read_char(); Token::Semicolon }
            '\0' => Token::EOF,

            _ => {
                if self.ch.is_alphabetic() {
                    let ident = self.read_identifier();
                    return match ident.as_str() {
                        "let" => Token::Let,
                        "fn" => Token::Fn,
                        "return" => Token::Return,
                        "true" => Token::True,
                        "false" => Token::False,
                        "if" => Token::If,
                        "else" => Token::Else,
                        _ => Token::Ident(ident),
                    };
                } else if self.ch.is_numeric() {
                    return Token::Number(self.read_number());
                } else {
                    Token::Illegal
                }
            }
        };

        tok
    }
    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

}

use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Integer(i64),
    Float(f64),
    Identifier(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Let,
    Function,
    Return,
    If,
    Else,
    While,
    EOF
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> TokenType {
        self.skip_whitespace();

        match self.input.next() {
            Some(ch) => {
                self.position += 1;
                match ch {
                    '0'..='9' => self.read_number(ch),
                    'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(ch),
                    '+' => TokenType::Plus,
                    '-' => TokenType::Minus,
                    '*' => TokenType::Multiply,
                    '/' => TokenType::Divide,
                    '=' => TokenType::Equal,
                    '(' => TokenType::LeftParen,
                    ')' => TokenType::RightParen,
                    '{' => TokenType::LeftBrace,
                    '}' => TokenType::RightBrace,
                    ';' => TokenType::Semicolon,
                    _ => self.read_next(),
                }
            }
            None => TokenType::EOF,
        }
    }

    fn read_number(&mut self, first_digit: char) -> TokenType {
        let mut number = first_digit.to_string();
        let mut is_float = false;

        while let Some(&ch) = self.input.peek() {
            if ch.is_digit(10) || (!is_float && ch == '.') {
                if ch == '.' {
                    is_float = true;
                }
                number.push(ch);
                self.input.next();
                self.position += 1;
            } else {
                break;
            }
        }

        if is_float {
            TokenType::Float(number.parse().unwrap())
        } else {
            TokenType::Integer(number.parse().unwrap())
        }
    }

    fn read_identifier(&mut self, first_char: char) -> TokenType {
        let mut identifier = first_char.to_string();

        while let Some(&ch) = self.input.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.input.next();
                self.position += 1;
            } else {
                break;
            }
        }

        match identifier.as_str() {
            "let" => TokenType::Let,
            "fn" => TokenType::Function,
            "return" => TokenType::Return,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            _ => TokenType::Identifier(identifier),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.input.next();
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn read_next(&mut self) -> TokenType {
        TokenType::EOF
    }
}

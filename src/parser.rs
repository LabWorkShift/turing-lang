use crate::lexer::{Lexer, TokenType};

#[derive(Debug)]
pub enum Node {
    Program(Vec<Node>),
    LetStatement {
        name: String,
        value: Box<Node>,
    },
    FunctionDeclaration {
        name: String,
        parameters: Vec<String>,
        body: Box<Node>,
    },
    Block(Vec<Node>),
    ReturnStatement(Box<Node>),
    IfStatement {
        condition: Box<Node>,
        consequence: Box<Node>,
        alternative: Option<Box<Node>>,
    },
    WhileStatement {
        condition: Box<Node>,
        body: Box<Node>,
    },
    Identifier(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    BinaryOperation {
        left: Box<Node>,
        operator: String,
        right: Box<Node>,
    },
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: TokenType,
    peek_token: TokenType,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: TokenType::EOF,
            peek_token: TokenType::EOF,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Node {
        let mut statements = Vec::new();

        while self.current_token != TokenType::EOF {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.next_token();
        }

        Node::Program(statements)
    }

    fn parse_statement(&mut self) -> Option<Node> {
        match self.current_token {
            TokenType::Let => Some(self.parse_let_statement()),
            TokenType::Function => Some(self.parse_function_declaration()),
            TokenType::Return => Some(self.parse_return_statement()),
            TokenType::If => Some(self.parse_if_statement()),
            TokenType::While => Some(self.parse_while_statement()),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Node {
        self.next_token();

        let name = match &self.current_token {
            TokenType::Identifier(name) => name.clone(),
            _ => panic!("Expected identifier after 'let'"),
        };

        self.next_token();
        if self.current_token != TokenType::Equal {
            panic!("Expected '=' after identifier in let statement");
        }

        self.next_token();
        let value = self.parse_expression();

        Node::LetStatement {
            name,
            value: Box::new(value),
        }
    }

    fn parse_function_declaration(&mut self) -> Node {
        self.next_token();

        let name = match &self.current_token {
            TokenType::Identifier(name) => name.clone(),
            _ => panic!("Expected function name"),
        };

        self.next_token();
        if self.current_token != TokenType::LeftParen {
            panic!("Expected '(' after function name");
        }

        let parameters = self.parse_function_parameters();

        if self.current_token != TokenType::LeftBrace {
            panic!("Expected '{{' to start function body");
        }

        let body = self.parse_block_statement();

        Node::FunctionDeclaration {
            name,
            parameters,
            body: Box::new(body),
        }
    }

    fn parse_function_parameters(&mut self) -> Vec<String> {
        let mut parameters = Vec::new();

        self.next_token();
        while self.current_token != TokenType::RightParen {
            if let TokenType::Identifier(name) = &self.current_token {
                parameters.push(name.clone());
            }

            self.next_token();
            if self.current_token == TokenType::RightParen {
                break;
            }
        }

        parameters
    }

    fn parse_block_statement(&mut self) -> Node {
        self.next_token();
        let mut statements = Vec::new();

        while self.current_token != TokenType::RightBrace && self.current_token != TokenType::EOF {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.next_token();
        }

        Node::Block(statements)
    }

    fn parse_return_statement(&mut self) -> Node {
        self.next_token();
        let value = self.parse_expression();
        Node::ReturnStatement(Box::new(value))
    }

    fn parse_if_statement(&mut self) -> Node {
        self.next_token();
        let condition = self.parse_expression();

        if self.peek_token != TokenType::LeftBrace {
            panic!("Expected '{{' after if condition");
        }

        self.next_token();
        let consequence = self.parse_block_statement();

        let alternative = if self.peek_token == TokenType::Else {
            self.next_token();
            self.next_token();
            Some(Box::new(self.parse_block_statement()))
        } else {
            None
        };

        Node::IfStatement {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative,
        }
    }

    fn parse_while_statement(&mut self) -> Node {
        self.next_token();
        let condition = self.parse_expression();

        if self.peek_token != TokenType::LeftBrace {
            panic!("Expected '{{' after while condition");
        }

        self.next_token();
        let body = self.parse_block_statement();

        Node::WhileStatement {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }

    fn parse_expression_statement(&mut self) -> Option<Node> {
        Some(self.parse_expression())
    }

    fn parse_expression(&mut self) -> Node {
        match &self.current_token {
            TokenType::Integer(n) => Node::IntegerLiteral(*n),
            TokenType::Float(f) => Node::FloatLiteral(*f),
            TokenType::Identifier(name) => Node::Identifier(name.clone()),
            _ => self.parse_binary_operation(),
        }
    }

    fn parse_binary_operation(&mut self) -> Node {
        let left = Box::new(self.parse_expression());

        let operator = match &self.current_token {
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Multiply => "*",
            TokenType::Divide => "/",
            _ => return *left,
        }.to_string();

        self.next_token();
        let right = Box::new(self.parse_expression());

        Node::BinaryOperation {
            left,
            operator,
            right,
        }
    }
}

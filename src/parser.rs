use std::os::linux::raw::stat;

use crate::ast::{Expression, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Program {
        let mut statements: Vec<Statement> = Vec::new();

        while let Some(token) = self.lexer.next() {
            match token.kind {
                TokenKind::Val => {
                    let identifier = if let Some(identifier) = self.lexer.next() {
                        identifier
                    } else {
                        panic!("Expected identifier after val keyword.");
                    };

                    if !matches!(
                        self.lexer.peek(),
                        Some(Token {
                            kind: TokenKind::Assign,
                            ..
                        })
                    ) {
                        panic!("Expected = after identifier.");
                    }

                    self.lexer.next();

                    let expression = self.parse_expression();

                    statements.push(Statement::Val {
                        name: identifier.literal,
                        initial: expression,
                    });
                }
                TokenKind::Var => {
                    let identifier = if let Some(identifier) = self.lexer.next() {
                        identifier
                    } else {
                        panic!("Expected identifier after var keyword.");
                    };

                    if !matches!(
                        self.lexer.peek(),
                        Some(Token {
                            kind: TokenKind::Assign,
                            ..
                        })
                    ) {
                        panic!("Expected = after identifier.");
                    }

                    self.lexer.next();

                    let expression = self.parse_expression();

                    statements.push(Statement::Var {
                        name: identifier.literal,
                        initial: expression,
                    });
                }
                _ => println!("{:?}", token),
            }
        }

        println!("{:?}", statements);

        statements
    }

    fn parse_expression(&mut self) -> Expression {
        match self.lexer.next() {
            Some(Token {
                kind: TokenKind::Integer,
                literal,
            }) => Expression::Integer(literal.parse().unwrap()),
            Some(Token {
                kind: TokenKind::Float,
                literal,
            }) => Expression::Float(literal.parse().unwrap()),
            Some(Token {
                kind: TokenKind::String,
                literal,
            }) => Expression::String(literal),
            _ => unimplemented!(),
        }
    }
}

pub type Program = Vec<Statement>;

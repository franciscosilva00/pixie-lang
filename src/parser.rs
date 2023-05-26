use crate::ast::{BinaryOperator, Expression, Statement};
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

                    let expression = self.parse_expression(0);

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

                    let expression = self.parse_expression(0);

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

    fn parse_expression(&mut self, bp: u8) -> Expression {
        let mut lhs = match self.lexer.next() {
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
        };

        loop {
            let infix = if let Some(infix) = self.lexer.peek() {
                infix
            } else {
                break;
            };

            if let Some((lbp, rbp)) = infix_binding_power(infix.kind) {
                if lbp < bp {
                    break;
                }

                let op = self.lexer.next().unwrap().kind;

                let rhs = self.parse_expression(rbp);

                lhs = make_infix_expression(lhs, op, rhs);

                continue;
            }

            break;
        }

        lhs
    }
}

fn infix_binding_power(kind: TokenKind) -> Option<(u8, u8)> {
    let bp = match kind {
        TokenKind::Multiply | TokenKind::Divide => (8, 9),
        TokenKind::Plus | TokenKind::Minus => (6, 7),
        _ => return None,
    };

    Some(bp)
}

fn make_infix_expression(lhs: Expression, op: TokenKind, rhs: Expression) -> Expression {
    let lhs = Box::new(lhs);
    let rhs = Box::new(rhs);

    match op {
        TokenKind::Multiply => Expression::Binary(lhs, BinaryOperator::Multiply, rhs),
        TokenKind::Plus => Expression::Binary(lhs, BinaryOperator::Plus, rhs),
        TokenKind::Minus => Expression::Binary(lhs, BinaryOperator::Minus, rhs),
        TokenKind::Divide => Expression::Binary(lhs, BinaryOperator::Divide, rhs),
        _ => unimplemented!(),
    }
}

pub type Program = Vec<Statement>;

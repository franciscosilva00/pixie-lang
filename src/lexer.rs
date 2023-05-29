use crate::token::{Token, TokenKind};

pub struct Lexer {
    source: Vec<char>,

    current: usize,
    next: usize,

    char: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut s = Self {
            source: input.chars().collect(),

            current: 0,
            next: 1,

            char: '\0',
        };

        s.char = s.source[s.current];

        s
    }

    fn read(&mut self) {
        if self.next >= self.source.len() {
            self.char = '0';
        } else {
            self.char = self.source[self.next];
        }

        self.current = self.next;

        self.next = self.current + 1;
    }

    fn skip_whitespace(&mut self) {
        while self.char.is_whitespace() {
            self.read();
        }
    }

    fn match_token(&mut self) -> Token {
        match self.char {
            '=' => {
                self.read();

                Token::new(TokenKind::Assign, "=".to_owned())
            }
            '+' => {
                self.read();

                Token::new(TokenKind::Plus, "+".to_owned())
            }
            '*' => {
                self.read();

                Token::new(TokenKind::Multiply, "*".to_owned())
            }
            '-' => {
                self.read();

                Token::new(TokenKind::Minus, "-".to_owned())
            }
            '/' => {
                self.read();

                Token::new(TokenKind::Divide, "/".to_owned())
            }
            '"' => {
                self.read();

                let mut buffer = String::new();

                while self.char != '\"' {
                    if self.char == '\\' {
                        self.read();
                    }

                    if self.current >= self.source.len() {
                        panic!("Unexpected EOF.");
                    }

                    buffer.push(self.char);

                    self.read();
                }

                self.read();

                Token::new(TokenKind::String, buffer)
            }
            _ if self.char.is_alphabetic() => {
                let mut buffer = String::new();

                buffer.push(self.char);

                self.read();

                while self.current < self.source.len() && self.char.is_alphabetic() {
                    buffer.push(self.char);

                    self.read();
                }

                let kind = match buffer.as_str() {
                    "val" => TokenKind::Val,
                    "var" => TokenKind::Var,
                    "true" => TokenKind::True,
                    "false" => TokenKind::False,
                    _ => TokenKind::Identifier,
                };

                Token::new(kind, buffer)
            }
            _ if self.char.is_numeric() => {
                let mut buffer = String::new();
                buffer.push(self.char);

                self.read();

                loop {
                    if self.current >= self.source.len() {
                        break;
                    }

                    if self.char == '_' {
                        self.read();
                    }

                    if !self.char.is_numeric() && self.char != '.' {
                        break;
                    }

                    buffer.push(self.char);

                    self.read();
                }

                if buffer.contains('.') {
                    Token::new(TokenKind::Float, buffer)
                } else {
                    Token::new(TokenKind::Integer, buffer)
                }
            }
            _ => unimplemented!(),
        }
    }

    pub fn peek(&mut self) -> Option<Token> {
        if self.next >= self.source.len() {
            return None;
        }

        let old_curr = self.current;
        let old_next = self.next;
        let old_char = self.char;

        self.char = self.source[self.next];

        let token = self.match_token();

        self.current = old_curr;
        self.next = old_next;
        self.char = old_char;

        Some(token)
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.next >= self.source.len() {
            return None;
        }

        self.skip_whitespace();

        let token = self.match_token();

        Some(token)
    }
}

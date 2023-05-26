use std::{env, fs};

#[derive(Debug)]
enum TokenKind {
    Identifier,
    Assign,
    Val,
    String,
    Integer,
    Float,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}

struct Lexer {
    source: Vec<char>,
    counter: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            counter: 0,
        }
    }

    fn increment(&mut self) {
        self.counter += 1;
    }

    pub fn lex(&mut self) {
        let mut tokens: Vec<Token> = Vec::new();

        while self.source.len() > self.counter {
            let c = self.curr_char();

            match c {
                '=' => {
                    tokens.push(Token::new(TokenKind::Assign, "=".to_owned()));

                    self.increment();
                }
                '"' => {
                    self.increment();

                    let mut buffer = String::new();

                    while self.curr_char() != '\"' {
                        if self.curr_char() == '\\' {
                            self.increment();
                        }

                        buffer.push(self.curr_char());

                        self.increment();
                    }

                    tokens.push(Token::new(TokenKind::String, buffer));

                    self.increment();
                }
                _ if c.is_numeric() => {
                    let mut buffer = String::new();
                    let mut is_float = false;

                    buffer.push(c);

                    self.increment();

                    loop {
                        if self.counter >= self.source.len() {
                            break;
                        }

                        if self.curr_char() == '_' {
                            self.increment();
                            continue;
                        }

                        if self.curr_char() == '.' && !is_float {
                            is_float = true;
                        } else if !self.curr_char().is_numeric() {
                            break;
                        }

                        buffer.push(self.curr_char());
                        self.increment();
                    }

                    if is_float {
                        tokens.push(Token::new(TokenKind::Float, buffer));
                    } else {
                        tokens.push(Token::new(TokenKind::Integer, buffer));
                    }
                }
                _ if c.is_alphabetic() => {
                    let mut buffer = String::new();

                    buffer.push(c);

                    self.increment();

                    while self.curr_char().is_alphabetic() {
                        buffer.push(self.curr_char());

                        self.increment();
                    }

                    let kind: TokenKind = match buffer.as_str() {
                        "val" => TokenKind::Val,
                        _ => TokenKind::Identifier,
                    };

                    tokens.push(Token::new(kind, buffer));
                }
                _ => {
                    self.increment();
                }
            }
        }

        println!("{:?}", tokens);
    }

    fn curr_char(&self) -> char {
        *self.source.get(self.counter).unwrap()
    }
}

fn main() {
    let file = env::args()
        .nth(1)
        .expect("Please provide a file path as an argument.");

    let source = fs::read_to_string(file).expect("Something went wrong reading the file.");

    let mut lexer = Lexer::new(source);

    lexer.lex();
}

#[derive(Debug)]
pub enum TokenKind {
    Identifier, // variable name
    Assign,     // =

    Val, // constant value (const)
    Var, // variable (let)

    String,  // "Hello, World!"
    Integer, // 123 / 123_456
    Float,   // 123.456 / 123_456.789

    Plus,
    Multiply,
    Minus,
    Divide,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}

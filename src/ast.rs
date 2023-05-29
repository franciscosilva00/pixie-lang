#[derive(Debug)]
pub enum Statement {
    Val { name: String, initial: Expression },
    Var { name: String, initial: Expression },
}

#[derive(Debug)]
pub enum Expression {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),

    Binary(Box<Expression>, BinaryOperator, Box<Expression>),
}

#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Multiply,   
    Minus,
    Divide,
}

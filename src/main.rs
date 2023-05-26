mod ast;
mod lexer;
mod parser;
mod token;

use std::{env, fs};

use lexer::Lexer;

fn main() {
    let file = env::args()
        .nth(1)
        .expect("Please provide a file path as an argument.");

    let source = fs::read_to_string(file).expect("Something went wrong reading the file.");

    let lexer = Lexer::new(source);

    // while let Some(token) = lexer.next() {
    //     println!("{:?}", token);
    // }

    let mut p = parser::Parser::new(lexer);

    let _ = p.parse();
}

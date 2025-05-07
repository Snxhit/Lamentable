mod lexer;
mod parser;
mod ast;
mod interpreter;

use std::fs;
use lexer::{tokenize, TokenList};
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let source = fs::read_to_string("main.lm")
        .expect("Failed to read Lamentable (.lm) file!");

    let tokens = tokenize(&source);
    for token in &tokens
    {
//        println!("{:?}", token);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
//    println!("{:?}", ast);
 
    let mut interpreter = Interpreter::new();
    interpreter.interpret(ast);
}

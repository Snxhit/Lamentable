use std::env;
use std::fs;

mod lexer;
mod parser;
mod ast;
mod interpreter;

use lexer::{tokenize, TokenList};
use parser::Parser;
use interpreter::Interpreter;

fn main() {
// UNCOMMENT FOR PRODUCTION
/*    let args: Vec<String> = env::args().collect();
    let source = fs::read_to_string(&args[1])
        .expect("Failed to read Lamentable (.lm) file!");*/

    let source = fs::read_to_string("main.lm")
        .expect("Failed to read Lamentable (.lm) file!");

    let tokens = tokenize(&source);
    for token in &tokens
    {
//        println!("{:?}", token);
    }

    let mut parser = Parser::new(tokens);
    let AST = parser.parse();
//    println!("{:?}", ast);
 
    let mut interpreter = Interpreter::new();
    interpreter.interpret(AST);
}

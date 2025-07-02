use std::env;
use std::fs;
use std::panic;
use std::collections::HashMap;

mod lexer;
mod parser;
mod ast;
mod interpreter;

use lexer::{tokenize, TokenList};
use parser::Parser;
use interpreter::Interpreter;

fn main() {
// CENTRALIZED ERROR HANDLER
    panic::set_hook(Box::new(|info|
    {
        if let Some(s) = info.payload().downcast_ref::<&str>()
        {
            eprintln!("{}", s);
        }
        else if let Some(s) = info.payload().downcast_ref::<String>()
        {
            eprintln!("{}", s);
        }
        else
        {
            eprintln!("Something went wrong :(");
        }
    }));

// UNCOMMENT FOR PRODUCTION
    let args: Vec<String> = env::args().collect();
    let source = fs::read_to_string(&args[1])
        .expect("Failed to read Lamentable (.lm) file!");

/*    let source = fs::read_to_string("main.lm")
        .expect("Failed to read Lamentable (.lm) file!");*/

    let tokens = tokenize(&source);
    for token in &tokens
    {
//        println!("{:?}", token);
    }

    let mut interpreter = Interpreter::new(HashMap::new(), HashMap::new(), HashMap::new());

    let mut parser = Parser::new(tokens, HashMap::new(), HashMap::new(), HashMap::new());
    let AST = parser.parse();
//    println!("{:?}", ast);
 
    interpreter.interpret(AST);
}

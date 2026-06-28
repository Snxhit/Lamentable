use std::fs;
mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    //let source = fs::read_to_string("main.lm").expect("Failed to read Lamentable source file!");
    let source = "++ 1 2 + 24+1";
    println!("{}", source);

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize();

    for t in &tokens {
        println!("{:?}", t);
    }
}

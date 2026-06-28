use std::fs;

fn main() {
    let source = fs::read_to_string("main.lm")
        .expect("Failed to read Lamentable source file!");
    println!("{}", source);
}

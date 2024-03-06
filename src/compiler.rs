use std::fs;

use self::{lexer::scan_tokens, parser::parse};

pub mod lexer;
mod parser;

pub fn compile(file: String) {
    let contents = fs::read_to_string(file)
        .expect("Unable to read file");
    
    let tokens = scan_tokens(contents);

    for token in &tokens {
        println!("{:?}", token);
    }

    let node = parse(tokens);
    println!("{:?}", node);
}
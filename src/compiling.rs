use std::{env, fs, path::Path};

use crate::compiling::{compiler::Compiler, parser::Node};

use self::{lexer::scan_tokens, parser::parse};

mod lexer;
pub mod compiler;
pub mod error_handler;
mod parser;

pub fn compile(file: String) {
    let path = Path::new(&file);

    if !path.exists() {
        panic!("File {} does not exist", file);
    }

    env::set_current_dir(path.parent().unwrap()).unwrap();

    let file = path.file_name().unwrap();

    let contents = fs::read_to_string(file)
        .expect("Unable to read file");
    
    let tokens = scan_tokens(contents);

    let mut node = parse(tokens, String::from(file.to_str().unwrap()));

    let mut compiler = Compiler::new();
    node.compile(&mut compiler);

    for b in &compiler.bytes {
        print!("{:08b} ", b);
    }
    println!();
}
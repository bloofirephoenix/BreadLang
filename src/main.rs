use std::env;

use crate::compiler::compile;

pub mod compiler;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    println!("BreadLang v2.0");

    let path = String::from("./program/main.bread");
    compile(path);
}

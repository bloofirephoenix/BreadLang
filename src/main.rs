use std::env;

use crate::compiling::compile;

pub mod compiling;

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");

    println!("BreadLang v2.0");

    let path = String::from("./program/main.bread");

    compile(path);
}

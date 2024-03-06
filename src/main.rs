use crate::compiler::compile;

pub mod compiler;

fn main() {
    println!("BreadLang v2.0");

    let path = String::from("./program/main.bread");
    compile(path);
}

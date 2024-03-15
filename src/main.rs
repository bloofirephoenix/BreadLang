#[macro_use] extern crate enum_primitive;

use std::{env, fs::{DirBuilder, File}, io::Write, path::Path};

use colored::Colorize;
use compiling::error_handler;

use crate::{compiling::compile, run::run};

pub mod compiling;
pub mod run;

fn main() {
    let v = env!("CARGO_PKG_VERSION");
    println!("{}", format!("BreadLang v{}", v).yellow().bold());

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        usage();
        return;
    }

    let command = &args[1];
    let mut arguments: Vec<String> = Vec::new();

    for i in 2..args.len() {
        arguments.push(args[i].clone());
    }

    match command.as_str() {
        "run" => {
            let debug:bool = arguments.contains(&String::from("--debug"));

            if let Ok(bytecode) = build() {
                run(bytecode, debug);
            }
        },
        "build" => {
            let _ = build();
        },
        "new" => {
            new();
        }
        _ => usage(),
    }
}

fn new() {
    let src = Path::new("src");
    let main = Path::new("src/main.bread");
    let gitignore = Path::new(".gitignore");

    if src.exists() {
        error_handler::print_error("src folder already exists");
        return;
    }
    if gitignore.exists() {
        error_handler::print_error(".gitignore already exists");
        return;
    }

    // make src folder
    DirBuilder::new().create(src).unwrap();
    
    // make gitignore
    let mut ignore_out = File::create(gitignore).unwrap();
    write!(ignore_out, "/bin \n *.crumbs").unwrap();

    // make main.bread
    let mut main_out = File::create(main).unwrap();
    write!(main_out, "main:\n\tHLT").unwrap();

    println!("{}", "Finished".green().bold());
}

fn build() -> Result<Vec<u8>, ()> {
    println!("Compiling project");
    let bytecode = compile();

    if let Ok(bytecode) = bytecode {
        println!("{}", "Finished".green().bold());
        
        Ok(bytecode)
    } else {
        println!("{}", "Failed to compile".red().bold());

        Err(())
    }
}

fn usage() {
    println!("Usage:");
    println!("  BreadLang run [--debug]");
    println!("  BreadLang build");
    println!("  BreadLang new");
}
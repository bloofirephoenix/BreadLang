#[macro_use] extern crate enum_primitive;

use std::env;

use colored::Colorize;

use crate::{compiling::compile, run::run};

pub mod compiling;
pub mod run;

fn main() {
    env::set_current_dir("program").unwrap();

    println!("{}", "BreadLang v2.0".yellow().bold());

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
        _ => usage(),
    }
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
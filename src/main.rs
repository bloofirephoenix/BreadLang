#[macro_use] extern crate enum_primitive;

use std::{env, fs::{self, DirBuilder, File}, io::Write, path::Path};

use colored::Colorize;
use compiling::error_handler;

use crate::{compiling::compile, run::run, upload::upload};

pub mod compiling;
pub mod run;
mod upload;
mod special_programs;

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
        "new" => new(),
        "upload" => {
            let program: Vec<u8>;
            if arguments.len() > 0 {
                // read file to upload
                match arguments[0].as_str() {
                    "--display" => {
                        program = special_programs::segment_display();
                    },
                    "--brain" => {
                        if arguments.len() < 2 {
                            usage();
                            return;
                        } else {
                            let input = &arguments[1];
                            let byte_select: u8;
                            if let Ok(num) = input.trim().parse::<u8>() {
                                byte_select = num;
                            } else {
                                error_handler::print_error("Invalid number");
                                return;
                            }
                            program = match special_programs::brain::get_program(byte_select) {
                                Ok(p) => p,
                                Err(msg) => {
                                    error_handler::print_error(&msg);
                                    return;
                                }
                            }
                        }
                    }
                    _ => {
                        match fs::read(&arguments[0]) {
                            Ok(p) => {
                                program = p;
                            },
                            Err(e) => {
                                error_handler::print_error(&format!("Failed to read file {}", e));
                                return;
                            }
                        }
                    }
                }
            } else {
                if let Ok(b) = build() {
                    program = b;
                } else {
                    return;
                }
            }
            
            upload(program);
        },
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
    println!("  BreadLang upload");
}
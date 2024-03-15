use std::{env, fs::{self, DirBuilder, File}, io::Write, path::Path};

use crate::compiling::compiler::Compiler;

use self::{error_handler::{CompilerError, ErrorCode}, lexer::{scan_tokens, Token}, parser::{parse, program_node::ProgramNode}};

mod lexer;
pub mod compiler;
pub mod error_handler;
mod parser;

pub fn compile() -> Result<Vec<u8>,()> {
    env::set_current_dir("src").unwrap();
    
    let file = "main.bread";
    let path = Path::new(&file);

    if !path.exists() {
        //throw_error(format!("File {} does not exist", file));
        CompilerError::new(
            ErrorCode::NoSuchFile(String::from("main.bread")), &String::from("n/a"), -1, true
        ).print();
        return Err(());
    }

    let file = path.file_name().unwrap();

    let contents = fs::read_to_string(file).unwrap();
    
    let tokens: Vec<Token>;

    match scan_tokens(contents, String::from(file.to_str().unwrap())) {
        Ok(t) => tokens = t,
        Err(e) => {
            e.print();
            return Err(());
        }
    }

    let node: ProgramNode;

    match parse(tokens, String::from(file.to_str().unwrap())) {
        Ok(n) => node = n,
        Err(e) => {
            for error in e {
                error.print();
            }
            return Err(());
        }
    }

    env::set_current_dir("..").unwrap();
    

    let mut compiler = Compiler::new();
    node.compile(&mut compiler);
    
    if !Path::new("bin").exists() {
        DirBuilder::new().create("bin").unwrap();
    }
    let mut file = File::create("bin/program.crumbs").unwrap();

    file.write_all(&compiler.bytes).unwrap();

    return Ok(compiler.bytes);
}

// enums
enum_from_primitive! {
    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum Instruction {
        //   = 0b00000,
        LW   = 0b00001,
        SW   = 0b00010,
        MW   = 0b00011,
        PUSH = 0b00100,
        POP  = 0b00101,
        LDA  = 0b00110,
        JMP  = 0b00111,
        JZ   = 0b01000,
        JO   = 0b01001,
        ADD  = 0b01010,
        SUB  = 0b01011,
        //   =  0b01100
        TEL  = 0b01101,
        OUT  = 0b01110,
        HLT  = 0b01111,

        NOP  = 0b11111,
    }
}

enum_from_primitive! {
    #[derive(PartialEq, Debug, Clone, Copy, Hash, Eq)]
    pub enum Register {
        A = 0b00,
        B = 0b01,
        H = 0b10,
        L = 0b11
    }
}
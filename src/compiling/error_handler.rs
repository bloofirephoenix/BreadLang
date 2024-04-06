use core::fmt;

use colored::Colorize;

use super::lexer::{Token, TokenType};

pub struct CompilerError {
    file: String,
    line: i32,
    code: ErrorCode,
    critical: bool
}

impl CompilerError {
    pub fn new(code: ErrorCode, file: &String, line: i32, critical: bool) -> CompilerError {
        CompilerError {
            file: file.clone(),
            line,
            code,
            critical
        }
    }

    pub fn expected(expected: &str, found: &Token, critical: bool) -> CompilerError {
        CompilerError::from_token(ErrorCode::ExpectedButFound(expected.to_string(), found.token_type.clone()), found, critical)
    }

    pub fn from_token(code: ErrorCode, token: &Token, critical: bool) -> CompilerError {
        CompilerError::new(code, &token.file, token.line, critical)
    }

    pub fn print(&self) {
        println!("{}: {} at {}:{}", "[Error]".red().bold(), self.code, self.file, self.line);
    }
}

pub fn has_critical(errors: &Vec<CompilerError>) -> bool {
    for error in errors {
        if error.critical {
            return true;
        }
    }

    false
}

pub fn print_error(msg: &str) {
    println!("{}: {}", "[Error]".red().bold(), msg);
}

pub fn print_warning(msg: &str) {
    println!("{}: {}", "[Warning]".yellow().bold(), msg);
}

pub enum ErrorCode {
    // lexer errors
    InvalidNumber,
    UnexpectedChar(char),

    // parser
    ExpectedButFound(String, TokenType),
    NumberTooBig(i32),
    NoSuchMacro(String),
    MacroCallsMacro,

    // files
    NoSuchFile(String),
    NoMainSubroutine,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::InvalidNumber => write!(f,
                "Invalid Number: Only positive decimal, hexadecimal, and binary numbers are allowed."),
            
            Self::UnexpectedChar(c) => write!(f, "Unexpected Char \'{}\'", c),

            Self::ExpectedButFound(expected, found) => 
                write!(f, "Expected {} but found {:?}", expected, found),
            
            Self::NumberTooBig(num) => write!(f, "Number {} is too large", num),

            Self::NoSuchMacro(name) => write!(f, "A macro named {} does not exist", name),

            Self::MacroCallsMacro => write!(f, "Macros cannot call other macros"),

            Self::NoSuchFile(filename) => write!(f, "File {} does not exist", filename),
            Self::NoMainSubroutine => write!(f, "A \"main\" subroutine is required"),
        }
    }
}
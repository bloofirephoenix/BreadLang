use crate::{compiling::{error_handler::{CompilerError, ErrorCode}, lexer::TokenType}, new};

use super::{Node, Parser};

#[derive(Debug)]
pub struct Imm8(u8);

impl Imm8 {
    pub fn new(value: i32) -> Option<Imm8> {
        match u8::try_from(value) {
            Ok(n) => Some(Imm8(n)),
            Err(_) => None,
        }
    }
    pub fn from_imm16(value: Imm16) -> Imm8 {
        Imm8::new(value.0 as i32).unwrap()
    }
}

impl Node for Imm8 {
    fn populate(parser: &mut Parser) -> Result<Imm8, CompilerError> {
        let num = get_number(parser)?;
        match Imm8::new(num) {
            Some(n) => Ok(n),
            None => Err(CompilerError::from_token(ErrorCode::NumberTooBig(num), parser.current(), false)),
        }
    }

    fn get_size(&self) -> i32 {
        1
    }
    
    fn compile(&self, compiler: &mut crate::compiling::compiler::Compiler) {
        compiler.add_byte(self.0);
    }
}

#[derive(Debug, Clone)]
pub struct Imm16(u16);

impl Imm16 {
    pub fn new(value: i32) -> Option<Imm16> {
        match u16::try_from(value) {
            Ok(n) => Some(Imm16(n)),
            Err(_) => None,
        }
    }
    pub fn from(value: u16) -> Imm16 {
        Imm16(value)
    }
}

impl Node for Imm16 {
    fn populate(parser: &mut Parser) -> Result<Imm16, CompilerError> {
        let num = get_number(parser)?;
        match Imm16::new(num) {
            Some(n) => Ok(n),
            None => Err(CompilerError::from_token(ErrorCode::NumberTooBig(num), parser.current(), false)),
        }
    }

    fn get_size(&self) -> i32 {
        2
    }
    
    fn compile(&self, compiler: &mut crate::compiling::compiler::Compiler) {
        compiler.add_bytes(&self.0.to_be_bytes())
    }
}

fn get_number(parser: &mut Parser) -> Result<i32, CompilerError> {
    let token = parser.advance();
    if let TokenType::Number(num) = &token.token_type {
        let num = num.replace("_", "");
        let result;
        if num.starts_with("0x") {
            // hexadecimal
            result = i32::from_str_radix(&num[2..], 16);
        } else if num.starts_with("0b") {
            // binary
            result = i32::from_str_radix(&num[2..], 2);
        } else {
            // decimal
            result = i32::from_str_radix(&num, 10);
        }

        return match result {
            Ok(num) => Ok(num),
            Err(_) => {
                Err(CompilerError::from_token(ErrorCode::InvalidNumber, token, false))
            }
        }
    }
    Err(CompilerError::expected("Number", token, false))
}
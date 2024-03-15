use crate::compiling::{error_handler::{CompilerError, ErrorCode}, lexer::TokenType};

use super::{Node, Parser};

#[derive(Debug)]
pub struct Imm8(u8);

impl Node for Imm8 {
    fn populate(parser: &mut Parser) -> Result<Imm8, CompilerError> {
        let num = get_number(parser)?;
        match u8::try_from(num) {
            Ok(n) => Ok(Imm8(n)),
            Err(_) => Err(CompilerError::from_token(ErrorCode::NumberTooBig(num), parser.current(), false)),
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

impl Node for Imm16 {
    fn populate(parser: &mut Parser) -> Result<Imm16, CompilerError> {
        let num = get_number(parser)?;
        match u16::try_from(num) {
            Ok(n) => Ok(Imm16(n)),
            Err(_) => Err(CompilerError::from_token(ErrorCode::NumberTooBig(num), parser.current(), false)),
        }    }

    fn get_size(&self) -> i32 {
        2
    }
    
    fn compile(&self, compiler: &mut crate::compiling::compiler::Compiler) {
        compiler.add_bytes(&self.0.to_be_bytes())
    }
}

impl Imm16 {
    pub fn from(value: u16) -> Imm16 {
        Imm16(value)
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
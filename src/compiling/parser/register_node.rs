use crate::compiling::{compiler::Compiler, error_handler::{CompilerError, ErrorCode}, lexer::TokenType, Register};

use super::{Node, Parser};

#[derive(Debug)]
pub struct RegisterNode(pub Register);

impl Node for RegisterNode {
    fn populate(parser: &mut Parser) -> Result<RegisterNode, CompilerError> {
        match &parser.advance().token_type {
            TokenType::Register(reg) => Ok(RegisterNode(reg.clone())),
            _ => {
                Err(CompilerError::from_token(
                    ErrorCode::ExpectedButFound("Register".to_string(), parser.current().token_type.clone()), 
                    parser.current(),
                    false
                ))
            }
        }
    }

    fn get_size(&self) -> i32 {
        panic!("Requesting the size of a register is not a valid operation")
    }
    
    fn compile(&self, _compiler: &mut Compiler) {
        panic!("Compiling a RegisterNode is not a valid operation")
    }
}
use crate::compiling::{compiler::Compiler, error_handler::CompilerError, lexer::TokenType};

use super::{placeholder_node::PlaceholderOrImm8Node, register_node::RegisterNode, Node, Parser};

#[derive(Debug)]
pub enum RegOrImmNode {
    Immediate(PlaceholderOrImm8Node),
    Register(RegisterNode)
}

impl Node for RegOrImmNode {
    fn populate(parser: &mut Parser) -> Result<RegOrImmNode, CompilerError> {
        let token = parser.peek();
        match token.token_type {
            TokenType::Register(_) => {
                Ok(RegOrImmNode::Register(RegisterNode::populate(parser)?))
            },
            TokenType::Number(_) | TokenType::Identifier(_) => {
                Ok(RegOrImmNode::Immediate(PlaceholderOrImm8Node::populate(parser)?))
            },
            _ => {
                Err(CompilerError::expected("Register or imm8", token, false))
            }
        }
    }

    fn get_size(&self) -> i32 {
        panic!("Requesting the size of a RegOrImmNode is not a valid operation")
    }
    
    fn compile(&self, _compiler: &mut Compiler) {
        panic!("Compiling a RegOrImmNode is not a valid operation");
    }
}
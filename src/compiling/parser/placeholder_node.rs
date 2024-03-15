use crate::compiling::{compiler::Compiler, error_handler::CompilerError, lexer::TokenType};

use super::{number_nodes::Imm16, Node, Parser};

#[derive(Debug)]
pub struct PlaceholderNode {
    name: String
}

impl Node for PlaceholderNode {
    fn populate(parser: &mut Parser) -> Result<PlaceholderNode, CompilerError> {
        let identifier = parser.advance();
        if let TokenType::Identifier(str) = &identifier.token_type {
            Ok(PlaceholderNode {
                name: String::from(str)
            })
        } else {
            Err(CompilerError::expected("Identifier", identifier, false))
        }
    }

    fn get_size(&self) -> i32 {
        2
    }

    fn compile(&self, compiler: &mut Compiler) {
        if compiler.scope.contains_key(&self.name) {
            let value = compiler.scope.get(&self.name).unwrap().clone();
            value.compile(compiler);
        } else {
            panic!("Placeholder does not exist")
        }
    }
}

#[derive(Debug)]
pub enum PlaceholderOrImmNode {
    PlaceholderNode(PlaceholderNode),
    Imm16(Imm16)
}

impl Node for PlaceholderOrImmNode {
    fn populate(parser: &mut Parser) -> Result<PlaceholderOrImmNode, CompilerError> {
        match parser.peek().token_type {
            TokenType::Identifier(_) => Ok(PlaceholderOrImmNode::PlaceholderNode(PlaceholderNode::populate(parser)?)),
            TokenType::Number(_) => Ok(PlaceholderOrImmNode::Imm16(Imm16::populate(parser)?)),
            _ => {
                Err(CompilerError::expected("Identifier or number", parser.peek(), false))
            }
        }
    }

    fn get_size(&self) -> i32 {
        return 2;
    }

    fn compile(&self, compiler: &mut Compiler) {
        match self {
            PlaceholderOrImmNode::PlaceholderNode(node) => node.compile(compiler),
            PlaceholderOrImmNode::Imm16(node) => node.compile(compiler)
        }
    }
}
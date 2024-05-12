use crate::compiling::{compiler::Compiler, error_handler::CompilerError, lexer::TokenType};

use super::{number_nodes::{Imm16, Imm8}, Node, Parser};

#[derive(Debug)]
pub struct PlaceholderNode {
    name: String
}

impl PlaceholderNode {
    pub fn populate(parser: &mut Parser) -> Result<PlaceholderNode, CompilerError> {
        let identifier = parser.advance();
        if let TokenType::Identifier(str) = &identifier.token_type {
            Ok(PlaceholderNode {
                name: String::from(str)
            })
        } else {
            Err(CompilerError::expected("Identifier", identifier, false))
        }
    }

    pub fn compile(&self, compiler: &mut Compiler, imm8: bool) {
        if compiler.scope.contains_key(&self.name) {
            let value = compiler.scope.get(&self.name).unwrap().clone();

            if imm8 {
                Imm8::from_imm16(value).compile(compiler);
            } else {
                value.compile(compiler);
            }
        } else {
            panic!("Placeholder {} does not exist", self.name);
        }
    }
}

#[derive(Debug)]
pub enum PlaceholderOrImm16Node {
    PlaceholderNode(PlaceholderNode),
    Imm16(Imm16)
}

impl Node for PlaceholderOrImm16Node {
    fn populate(parser: &mut Parser) -> Result<PlaceholderOrImm16Node, CompilerError> {
        match parser.peek().token_type {
            TokenType::Identifier(_) => Ok(PlaceholderOrImm16Node::PlaceholderNode(PlaceholderNode::populate(parser)?)),
            TokenType::Number(_) => Ok(PlaceholderOrImm16Node::Imm16(Imm16::populate(parser)?)),
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
            PlaceholderOrImm16Node::PlaceholderNode(node) => node.compile(compiler, false),
            PlaceholderOrImm16Node::Imm16(node) => node.compile(compiler)
        }
    }
}

#[derive(Debug)]
pub enum PlaceholderOrImm8Node {
    PlaceholderNode(PlaceholderNode),
    Imm8(Imm8)
}

impl Node for PlaceholderOrImm8Node {
    fn populate(parser: &mut Parser) -> Result<PlaceholderOrImm8Node, CompilerError> {
        match parser.peek().token_type {
            TokenType::Identifier(_) => Ok(PlaceholderOrImm8Node::PlaceholderNode(PlaceholderNode::populate(parser)?)),
            TokenType::Number(_) => Ok(PlaceholderOrImm8Node::Imm8(Imm8::populate(parser)?)),
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
            PlaceholderOrImm8Node::PlaceholderNode(node) => node.compile(compiler, true),
            PlaceholderOrImm8Node::Imm8(node) => node.compile(compiler)
        }
    }
}
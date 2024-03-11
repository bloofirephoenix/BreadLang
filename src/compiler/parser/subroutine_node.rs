use crate::compiler::lexer::TokenType;

use super::{instruction_node::InstructionNode, Node, Parser};

#[derive(Debug)]
pub struct SubroutineNode {
    name: String,
    instructions: Vec<InstructionNode>
}

impl Node for SubroutineNode {
    fn populate(parser: &mut Parser) -> Self {
        parser.skip_new_lines();

        // identifier
        let identifier = parser.advance();
        let name: String;
        if let TokenType::Identifier(n) = &identifier.token_type {
            name = n.clone();
        } else {
            panic!("Expected identifier. Found {:?}", identifier.token_type)
        }
        
        // expect colon
        if !matches!(parser.advance().token_type, TokenType::Colon) {
            panic!("Expected colon {:?}", parser.current().token_type)
        }

        // expect new line
        if !matches!(parser.advance().token_type, TokenType::NewLine) {
            panic!("Expected new line")
        }

        let mut instructions: Vec<InstructionNode> = Vec::new();

        while !parser.is_at_end() {
            parser.skip_new_lines();
            if !matches!(parser.peek().token_type, TokenType::Indent(_)) {
                break;
            }
            
            parser.advance(); // advance past indent

            instructions.push(InstructionNode::populate(parser));
        }

        SubroutineNode {
            name,
            instructions
        }
    }

    fn get_size(&self) -> i32 {
        let mut size = 0;
        
        for node in &self.instructions {
            size += node.get_size();
        }

        size
    }
}

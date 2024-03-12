use std::collections::HashMap;

use crate::compiling::lexer::TokenType;

use super::{number_nodes::Imm16, subroutine_node::SubroutineNode, Node, Parser};

#[derive(Debug)]
pub struct ProgramNode {
    subroutines: Vec<SubroutineNode>,
    placeholders: HashMap<String, Imm16>
}

impl Node for ProgramNode {
    fn populate(parser: &mut Parser) -> ProgramNode {
        let mut subroutines: Vec<SubroutineNode> = Vec::new();

        'parser: while !parser.is_at_end() {
            parser.skip_new_lines();

            match parser.peek().token_type {
                TokenType::EndOfFile => break 'parser,
                TokenType::Macro => todo!("Macros are currently not supported"),
                TokenType::Identifier(_) => subroutines.push(SubroutineNode::populate(parser)),
                _ => panic!("Expected a macro, subroutine, or end of file. Found {:?}", parser.peek())
            }
        }

        ProgramNode {
            subroutines,
            placeholders: HashMap::new()
        }
    }

    fn get_size(&self) -> i32 {
        let mut size = 0;

        for node in &self.subroutines {
            size += node.get_size();
        }

        size
    }
    
    fn compile(&self, compiler: &mut crate::compiling::compiler::Compiler) {
        compiler.scope = self.placeholders.clone();
        for sub in &self.subroutines {
            sub.compile(compiler);
        }
    }
}

impl ProgramNode {
    pub fn calculate_placeholders(&mut self) {
        let mut position = 0;
        for subroutine in &self.subroutines {
            self.placeholders.insert(subroutine.name.clone(), Imm16::from(position));
            position += subroutine.get_size() as u16;
        }
        position = 0;

        for subroutine in &mut self.subroutines {
            subroutine.calculate_placeholders(&mut position, &self.placeholders);
        }
    }
}
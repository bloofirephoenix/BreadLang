use crate::compiler::lexer::TokenType;

use super::{subroutine_node::SubroutineNode, Node, Parser};

#[derive(Debug)]
pub struct ProgramNode {
    subroutines: Vec<SubroutineNode>,
}

impl Node for ProgramNode {
    fn populate(parser: &mut Parser) -> ProgramNode {
        let mut subroutines: Vec<SubroutineNode> = Vec::new();

        'parser: while !parser.is_at_end() {
            parser.skip_new_lines();
            println!("{:?}", parser.peek());

            match parser.peek().token_type {
                TokenType::EndOfFile => break 'parser,
                TokenType::Macro => todo!("Macros are currently not supported"),
                TokenType::Identifier(_) => subroutines.push(SubroutineNode::populate(parser)),
                _ => panic!("Expected a macro, subroutine, or end of file")
            }
        }

        ProgramNode {
            subroutines,
        }
    }

    fn get_size(&self) -> i32 {
        let mut size = 0;

        for node in &self.subroutines {
            size += node.get_size();
        }

        size
    }
}

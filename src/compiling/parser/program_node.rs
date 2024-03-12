use std::collections::HashMap;

use crate::compiling::lexer::TokenType;

use super::{macros::Macro, number_nodes::Imm16, subroutine_node::SubroutineNode, Node, Parser};

#[derive(Debug)]
pub struct ProgramNode {
    subroutines: Vec<SubroutineNode>,
    placeholders: HashMap<String, Imm16>,
    macros: HashMap<String, Macro>
}

impl Node for ProgramNode {
    fn populate(parser: &mut Parser) -> ProgramNode {
        let mut subroutines: Vec<SubroutineNode> = Vec::new();
        let mut macros: HashMap<String, Macro> = HashMap::new();

        'parser: while !parser.is_at_end() {
            parser.skip_new_lines();

            match parser.peek().token_type {
                TokenType::EndOfFile => break 'parser,
                TokenType::Macro => {
                    let m = Macro::populate(parser);
                    macros.insert(m.name.clone(), m);
                }
                TokenType::Identifier(_) => subroutines.push(SubroutineNode::populate(parser)),
                _ => panic!("Expected a macro, subroutine, or end of file. Found {:?}", parser.peek())
            }
        }

        // populate macros
        for sub in &mut subroutines {
            sub.populate_macros(&macros);
        }

        let mut node = ProgramNode {
            subroutines,
            placeholders: HashMap::new(),
            macros
        };

        node.calculate_placeholders();

        node
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
use std::collections::HashMap;

use crate::compiling::lexer::TokenType;

use super::{instruction_node::InstructionNode, macros::{Macro, MacroHolder, MacroNode}, number_nodes::Imm16, Node, Parser};

#[derive(Debug)]
pub struct SubroutineNode {
    pub name: String,
    instructions: Vec<InstructionNode>,
    placeholders: HashMap<String, Imm16>
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

        SubroutineNode {
            name,
            instructions: get_instructions(parser),
            placeholders: HashMap::new()
        }
    }

    fn get_size(&self) -> i32 {
        let mut size = 0;
        
        for node in &self.instructions {
            size += node.get_size();
        }

        size
    }
    
    fn compile(&self, compiler: &mut crate::compiling::compiler::Compiler) {
        compiler.scope = self.placeholders.clone();
        for instructions in &self.instructions {
            instructions.compile(compiler);
        }
    }
}

impl SubroutineNode {
    pub fn calculate_placeholders(&mut self, position: &mut u16, placeholders: &HashMap<String, Imm16>) {
        let initial_position = *position;

        self.placeholders = placeholders.clone();
        for instruction in &self.instructions {
            if let InstructionNode::DEF(name) = instruction {
                self.placeholders.insert(name.clone(), Imm16::from(*position));
            } else {
                *position += instruction.get_size() as u16;
            }
        }

        // ok now macros
        *position = initial_position;

        for instruction in &mut self.instructions {
            if let InstructionNode::Macro(holder) = instruction {
                if let MacroHolder::Macro(node) = holder {
                    node.calculate_placeholders(position, placeholders)
                } else {
                    panic!("All macros should be populated by now");
                }
            } else {
                *position += instruction.get_size() as u16;
            }
        }
    }

    pub fn populate_macros(&mut self, macros: &HashMap<String, Macro>) {
        for instruction in &mut self.instructions {
            if let InstructionNode::Macro(holder) = instruction {
                if let MacroHolder::Placeholder(name, args) = holder {
                    let m = macros.get(name).unwrap(); // todo
                    *instruction = InstructionNode::Macro(MacroHolder::Macro(MacroNode::populate(m, args)));
                }
            }
        }
    }
}

pub fn get_instructions(parser: &mut Parser) -> Vec<InstructionNode> {
    let mut instructions: Vec<InstructionNode> = Vec::new();

    'parser: while !parser.is_at_end() {
        'lines: loop {
            parser.skip_new_lines();
            if !matches!(parser.peek().token_type, TokenType::Indent(_)) {
                break 'parser;
            }

            parser.advance(); // advance past indent
            
            if !matches!(parser.peek().token_type, TokenType::NewLine) {
                break 'lines;
            }
        }
        instructions.push(InstructionNode::populate(parser));
    }
    
    instructions
}
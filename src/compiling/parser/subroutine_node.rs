use std::collections::HashMap;

use colored::Colorize;

use crate::compiling::{error_handler::{self, CompilerError, ErrorCode}, lexer::TokenType};

use super::{instruction_node::InstructionNode, macros::{Macro, MacroHolder, MacroNode}, number_nodes::Imm16, Node, Parser};

#[derive(Debug)]
pub struct SubroutineNode {
    pub name: String,
    instructions: Vec<InstructionNode>,
    placeholders: HashMap<String, Imm16>
}

impl SubroutineNode {
    pub fn populate(parser: &mut Parser) -> Result<SubroutineNode, Vec<CompilerError>> {
        parser.skip_new_lines();

        // identifier
        let identifier = parser.advance();
        let name: String;
        if let TokenType::Identifier(n) = &identifier.token_type {
            name = n.clone();
        } else {
            return Err(vec![CompilerError::expected("Identifier", identifier, true)]);
        }

        // expect colon
        if !matches!(parser.advance().token_type, TokenType::Colon) {
            return Err(vec![CompilerError::expected("Colon", parser.current(), true)]);
        }

        // expect new line
        if !matches!(parser.advance().token_type, TokenType::NewLine) {
            return Err(vec![CompilerError::expected("New Line", parser.current(), true)]);
        }

        let instructions = get_instructions(parser)?;

        if !matches!(instructions.last(), Some(InstructionNode::HLT) | Some(InstructionNode::JMP(_)) | None) {
            error_handler::print_warning(&format!("Subroutine {} does not end in HLT or JMP", name));
        }

        if matches!(instructions.last(), None) {
            error_handler::print_warning(&format!("Subroutine {} does not contain any instructions", name));
        }

        Ok(SubroutineNode {
            name,
            instructions,
            placeholders: HashMap::new()
        })
    }

    pub fn get_size(&self) -> i32 {
        let mut size = 0;
        
        for node in &self.instructions {
            size += node.get_size();
        }

        size
    }
    
    pub fn compile(&self, compiler: &mut crate::compiling::compiler::Compiler) {
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
                    node.calculate_placeholders(position, &self.placeholders)
                } else {
                    panic!("All macros should be populated by now");
                }
            } else {
                *position += instruction.get_size() as u16;
            }
        }
    }

    pub fn populate_macros(&mut self, macros: &HashMap<String, Macro>) -> Result<(), Vec<CompilerError>> {
        for instruction in &mut self.instructions {
            if let InstructionNode::Macro(holder) = instruction {
                if let MacroHolder::Placeholder(name, args, token) = holder {
                    let m = macros.get(name);
                    
                    if let Some(m) = m {
                        *instruction = InstructionNode::Macro(MacroHolder::Macro(MacroNode::populate(m, args)?));
                    } else {
                        return Err(vec![CompilerError::from_token(ErrorCode::NoSuchMacro(name.clone()), token, false)]);
                    }
                }
            }
        }

        Ok(())
    }
}

pub fn get_instructions(parser: &mut Parser) -> Result<Vec<InstructionNode>, Vec<CompilerError>> {
    let mut instructions: Vec<InstructionNode> = Vec::new();
    let mut errors: Vec<CompilerError> = Vec::new();

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
        match InstructionNode::populate(parser) {
            Ok(i) => { 
                instructions.push(i);

                while !matches!(parser.peek().token_type, TokenType::NewLine | TokenType::EndOfFile) {
                    errors.push(CompilerError::expected("New Line", parser.advance(), false))
                }
            },
            Err(e) => {
                errors.push(e);
                // advance until new line
                while !matches!(parser.peek().token_type, TokenType::NewLine | TokenType::EndOfFile) {
                    parser.advance();
                }
            }
        }
    }
    
    if errors.is_empty() {
        Ok(instructions)
    } else {
        Err(errors)
    }
}
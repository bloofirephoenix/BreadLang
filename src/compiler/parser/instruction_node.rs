use std::process::id;

use crate::compiler::lexer::{Instruction, Token, TokenType};

use super::{number_nodes::Imm16, Node, Parser, PlaceholderNode, RegOrImmNode, RegisterNode};

#[derive(Debug)]
pub enum InstructionNode {
    NOP,
    LW(RegisterNode, Option<Imm16>),
    SW(RegisterNode, Option<Imm16>),
    MW(RegisterNode, RegOrImmNode),
    PUSH(RegOrImmNode),
    POP(RegisterNode),
    LDA(Imm16),
    JMP(Option<PlaceholderNode>),
    JZ(RegisterNode, Option<PlaceholderNode>),
    JO(Option<PlaceholderNode>),
    ADD(RegisterNode, RegOrImmNode),
    SUB(RegisterNode, RegOrImmNode),
    TEL(RegOrImmNode),
    OUT(RegOrImmNode),
    HLT,

    Macro(String, Vec<Token>),

    DEF(String)
}

impl Node for InstructionNode {
    fn populate(parser: &mut Parser) -> Self {
        let token = parser.advance();

        match token.token_type {
            TokenType::Instruction(Instruction::NOP) => InstructionNode::NOP,
            TokenType::Instruction(Instruction::LW) => {
                let register = RegisterNode::populate(parser);
                let number: Option<Imm16>;

                if matches!(parser.peek().token_type, TokenType::Number(_)) {
                    number = Some(Imm16::populate(parser))
                } else {
                    number = None;
                }
                
                InstructionNode::LW(register, number)
            },
            TokenType::Instruction(Instruction::SW) => {
                let register = RegisterNode::populate(parser);
                let number: Option<Imm16>;

                if matches!(parser.peek().token_type, TokenType::Number(_)) {
                    number = Some(Imm16::populate(parser))
                } else {
                    number = None;
                }
                
                InstructionNode::SW(register, number)
            },
            TokenType::Instruction(Instruction::MW) => {
                let register = RegisterNode::populate(parser);
                let reg_or_imm = RegOrImmNode::populate(parser);
                
                InstructionNode::MW(register, reg_or_imm)
            },
            TokenType::Instruction(Instruction::PUSH) => InstructionNode::PUSH(RegOrImmNode::populate(parser)),
            TokenType::Instruction(Instruction::POP) => InstructionNode::POP(RegisterNode::populate(parser)),
            TokenType::Instruction(Instruction::LDA) => InstructionNode::LDA(Imm16::populate(parser)),
            TokenType::Instruction(Instruction::JMP) => {
                if matches!(parser.peek().token_type, TokenType::Identifier(_)) {
                    InstructionNode::JMP(Some(PlaceholderNode::populate(parser)))
                } else {
                    InstructionNode::JMP(None)
                }
            },
            TokenType::Instruction(Instruction::JZ) => {
                let register = RegisterNode::populate(parser);
                if matches!(parser.peek().token_type, TokenType::Identifier(_)) {
                    InstructionNode::JZ(register, Some(PlaceholderNode::populate(parser)))
                } else {
                    InstructionNode::JZ(register, None)
                }
            },
            TokenType::Instruction(Instruction::JO) => {
                if matches!(parser.peek().token_type, TokenType::Identifier(_)) {
                    InstructionNode::JO(Some(PlaceholderNode::populate(parser)))
                } else {
                    InstructionNode::JO(None)
                }
            },
            TokenType::Instruction(Instruction::ADD) => InstructionNode::ADD(RegisterNode::populate(parser), RegOrImmNode::populate(parser)),
            TokenType::Instruction(Instruction::SUB) => InstructionNode::SUB(RegisterNode::populate(parser), RegOrImmNode::populate(parser)),
            TokenType::Instruction(Instruction::TEL) => InstructionNode::TEL(RegOrImmNode::populate(parser)),
            TokenType::Instruction(Instruction::OUT) => InstructionNode::OUT(RegOrImmNode::populate(parser)),
            TokenType::Instruction(Instruction::HLT) => InstructionNode::HLT,

            TokenType::Def => {
                if let TokenType::Identifier(name) = &parser.advance().token_type {
                    InstructionNode::DEF(name.clone())
                } else {
                    panic!("Expected an identifier");
                }
            }

            TokenType::Identifier(_) => {
                let macro_name: String;
                if let TokenType::Identifier(identifier) = &parser.current().token_type {
                    macro_name = identifier.clone();
                } else {
                    panic!("Expected identifier")
                }
                
                // grab the arguments
                let mut arguments: Vec<Token> = Vec::new();
                while !matches!(parser.peek().token_type, TokenType::NewLine | TokenType::EndOfFile) {
                    arguments.push(parser.advance().clone());
                }
                InstructionNode::Macro(macro_name, arguments)
            }

            _ => panic!("Invalid token. Expected instruction node"),
        }
    }

    fn get_size(&self) -> i32 {
        todo!()
    }
}
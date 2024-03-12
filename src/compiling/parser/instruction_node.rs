use crate::compiling::lexer::{Instruction, Token, TokenType};

use super::{macros::MacroHolder, number_nodes::Imm16, Node, Parser, PlaceholderNode, PlaceholderOrImmNode, RegOrImmNode, RegisterNode};

#[derive(Debug)]
pub enum InstructionNode {
    NOP,
    LW(RegisterNode, Option<Imm16>),
    SW(RegisterNode, Option<Imm16>),
    MW(RegisterNode, RegOrImmNode),
    PUSH(RegOrImmNode),
    POP(RegisterNode),
    LDA(PlaceholderOrImmNode),
    JMP(Option<PlaceholderNode>),
    JZ(RegisterNode, Option<PlaceholderNode>),
    JO(Option<PlaceholderNode>),
    ADD(RegisterNode, RegOrImmNode),
    SUB(RegisterNode, RegOrImmNode),
    TEL(RegOrImmNode),
    OUT(RegOrImmNode),
    HLT,

    Macro(MacroHolder),

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
            TokenType::Instruction(Instruction::LDA) => InstructionNode::LDA(PlaceholderOrImmNode::populate(parser)),
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
                InstructionNode::Macro(MacroHolder::Placeholder(macro_name, arguments))
            }

            _ => panic!("Invalid token. Expected instruction node. Found {:?}", token),
        }
    }

    fn get_size(&self) -> i32 {
        match self {
            Self::NOP | Self::HLT => 1,
            Self::LW(_, num) | Self::SW(_, num) => {
                if let None = num {
                    1
                } else {
                    3
                }
            },
            Self::MW(_, _) | Self::ADD(_, _) | Self::SUB(_, _) => {
                2
            },
            Self::PUSH(reg_imm) | Self::TEL(reg_imm) | Self::OUT(reg_imm) => {
                match reg_imm {
                    RegOrImmNode::Register(_) => 1,
                    RegOrImmNode::Immediate(_) => 2
                }
            }
            Self::POP(_) => 1,
            Self::LDA(_) => 3,
            Self::JMP(pos) | Self::JZ(_, pos) | Self::JO(pos) => {
                match pos {
                    Some(_) => 3,
                    None => 1
                }
            },
            Self::DEF(_) => 0,
            Self::Macro(holder) => {
                match holder {
                    MacroHolder::Placeholder(_, _) => panic!("Cannot get the size of a macro placeholder"),
                    MacroHolder::Macro(m) => m.get_size(),
                }
            }
        }
    }
    
    fn compile(&self, compiler: &mut crate::compiling::compiler::Compiler) {
        match self {
            InstructionNode::NOP => compiler.first_byte(Instruction::NOP, false, None),
            
            InstructionNode::LW(register, immediate) | 
                InstructionNode::SW(register, immediate) => {
                
                let register = register.0.clone();
                let instruction = node_to_instr(&self);

                if let Some(val) = immediate {
                    compiler.first_byte(instruction, true, Some(register));
                val.compile(compiler);
                } else {
                    compiler.first_byte(instruction, false, Some(register));
                }
            },

            InstructionNode::MW(reg, reg_imm) |
                InstructionNode::ADD(reg, reg_imm) |
                InstructionNode::SUB(reg, reg_imm) => {
                
                let instruction = node_to_instr(&self);

                match &reg_imm {
                    RegOrImmNode::Register(reg_b) => 
                        compiler.two_bytes(instruction, false, reg.0, reg_b.0),
                    
                    RegOrImmNode::Immediate(imm) => {
                        compiler.first_byte(instruction, true, Some(reg.0));
                        imm.compile(compiler)
                    }
                }
            }

            InstructionNode::PUSH(reg_imm) |
                InstructionNode::TEL(reg_imm) |
                InstructionNode::OUT(reg_imm) => {
                let instruction = node_to_instr(&self);

                match &reg_imm {
                    RegOrImmNode::Register(reg) => 
                        compiler.first_byte(instruction, false, Some(reg.0)),
                    
                    RegOrImmNode::Immediate(imm) => {
                        compiler.first_byte(instruction, true, None);
                        imm.compile(compiler)
                    }
                }
            }

            InstructionNode::POP(reg) => 
                compiler.first_byte(Instruction::POP, false, Some(reg.0)),
            
            InstructionNode::LDA(imm) => {
                compiler.first_byte(Instruction::LDA, true, None);
                imm.compile(compiler)
            }

            InstructionNode::JMP(imm) |
                InstructionNode::JO(imm) => {
                let instruction = node_to_instr(&self);
                match imm {
                    None => compiler.first_byte(instruction, false, None),
                    Some(val) => {
                        compiler.first_byte(instruction, true, None);
                        val.compile(compiler)
                    }
                }
            }

            InstructionNode::JZ(reg, imm) => {
                match imm {
                    None => compiler.first_byte(Instruction::JZ, false, Some(reg.0)),
                    Some(val) => {
                        compiler.first_byte(Instruction::JZ, true, Some(reg.0));
                        val.compile(compiler)
                    }
                }
            }
            
            InstructionNode::HLT => compiler.first_byte(Instruction::HLT, false, None),

            InstructionNode::Macro(holder) => {
                match holder {
                    MacroHolder::Placeholder(_, _) => panic!("Cannot compile a macro placeholder"),
                    MacroHolder::Macro(m) => m.compile(compiler),
                }
            },

            InstructionNode::DEF(_) => {} // do nothing.
        }
    }
}

fn node_to_instr(node: &InstructionNode) -> Instruction {
    match node {
        InstructionNode::NOP => Instruction::NOP,
        InstructionNode::ADD(_, _) => Instruction::ADD,
        InstructionNode::LW(_, _) => Instruction::LW,
        InstructionNode::SW(_, _) => Instruction::SW,
        InstructionNode::MW(_, _) => Instruction::MW,
        InstructionNode::PUSH(_) => Instruction::PUSH,
        InstructionNode::POP(_) => Instruction::POP,
        InstructionNode::LDA(_) => Instruction::LDA,
        InstructionNode::JMP(_) => Instruction::JMP,
        InstructionNode::JZ(_, _) => Instruction::JZ,
        InstructionNode::JO(_) => Instruction::JO,
        InstructionNode::SUB(_, _) => Instruction::SUB,
        InstructionNode::TEL(_) => Instruction::TEL,
        InstructionNode::OUT(_) => Instruction::OUT,
        InstructionNode::HLT => Instruction::HLT,
        InstructionNode::Macro(_) => panic!("Cannot convert a macro instruction node to an opcode"),
        InstructionNode::DEF(_) => panic!("Cannot convert a def instruction node to an opcode"),
    }
}
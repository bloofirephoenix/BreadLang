use crate::compiling::{error_handler::{CompilerError, ErrorCode}, lexer::{Token, TokenType}, Instruction};

use super::{macros::MacroHolder, placeholder_node::{PlaceholderNode, PlaceholderOrImm16Node}, reg_imm_node::RegOrImmNode, register_node::RegisterNode, Node, Parser};

#[derive(Debug)]
pub enum InstructionNode {
    NOP,
    LW(RegisterNode, Option<PlaceholderOrImm16Node>),
    SW(RegisterNode, Option<PlaceholderOrImm16Node>),
    MW(RegisterNode, RegOrImmNode),
    PUSH(RegOrImmNode),
    POP(RegisterNode),
    LDA(PlaceholderOrImm16Node),
    JMP(Option<PlaceholderNode>),
    JZ(RegisterNode, Option<PlaceholderNode>),
    JC(Option<PlaceholderNode>),
    ADD(RegisterNode, RegOrImmNode),
    SUB(RegisterNode, RegOrImmNode),
    OUT(RegOrImmNode),
    HLT,

    Macro(MacroHolder),

    DEF(String)
}

impl Node for InstructionNode {
    fn populate(parser: &mut Parser) -> Result<InstructionNode, CompilerError> {
        let token = parser.advance();

        match token.token_type {
            TokenType::Instruction(Instruction::NOP) => Ok(InstructionNode::NOP),
            TokenType::Instruction(Instruction::LW) => {
                let register = RegisterNode::populate(parser);
                let number: Option<PlaceholderOrImm16Node>;

                if matches!(parser.peek().token_type, TokenType::Number(_) | TokenType::Identifier(_)) {
                    number = Some(PlaceholderOrImm16Node::populate(parser)?)
                } else {
                    number = None;
                }
                
                Ok(InstructionNode::LW(register?, number))
            },
            TokenType::Instruction(Instruction::SW) => {
                let register = RegisterNode::populate(parser);
                let number: Option<PlaceholderOrImm16Node>;

                if matches!(parser.peek().token_type, TokenType::Number(_) | TokenType::Identifier(_)) {
                    number = Some(PlaceholderOrImm16Node::populate(parser)?)
                } else {
                    number = None;
                }
                
                Ok(InstructionNode::SW(register?, number))
            },
            TokenType::Instruction(Instruction::MW) => {
                let register = RegisterNode::populate(parser)?;
                let reg_or_imm = RegOrImmNode::populate(parser)?;
                
                Ok(InstructionNode::MW(register, reg_or_imm))
            },
            TokenType::Instruction(Instruction::PUSH) => Ok(InstructionNode::PUSH(RegOrImmNode::populate(parser)?)),
            TokenType::Instruction(Instruction::POP) => Ok(InstructionNode::POP(RegisterNode::populate(parser)?)),
            TokenType::Instruction(Instruction::LDA) => Ok(InstructionNode::LDA(PlaceholderOrImm16Node::populate(parser)?)),
            TokenType::Instruction(Instruction::JMP) => {
                if matches!(parser.peek().token_type, TokenType::Identifier(_)) {
                    Ok(InstructionNode::JMP(Some(PlaceholderNode::populate(parser)?)))
                } else {
                    Ok(InstructionNode::JMP(None))
                }
            },
            TokenType::Instruction(Instruction::JZ) => {
                let register = RegisterNode::populate(parser)?;
                if matches!(parser.peek().token_type, TokenType::Identifier(_)) {
                    Ok(InstructionNode::JZ(register, Some(PlaceholderNode::populate(parser)?)))
                } else {
                    Ok(InstructionNode::JZ(register, None))
                }
            },
            TokenType::Instruction(Instruction::JC) => {
                if matches!(parser.peek().token_type, TokenType::Identifier(_)) {
                    Ok(InstructionNode::JC(Some(PlaceholderNode::populate(parser)?)))
                } else {
                    Ok(InstructionNode::JC(None))
                }
            },
            TokenType::Instruction(Instruction::ADD) => 
                Ok(InstructionNode::ADD(RegisterNode::populate(parser)?, RegOrImmNode::populate(parser)?)),
            TokenType::Instruction(Instruction::SUB) => 
                Ok(InstructionNode::SUB(RegisterNode::populate(parser)?, RegOrImmNode::populate(parser)?)),
            TokenType::Instruction(Instruction::OUT) => 
                Ok(InstructionNode::OUT(RegOrImmNode::populate(parser)?)),
            TokenType::Instruction(Instruction::HLT) => 
                Ok(InstructionNode::HLT),

            TokenType::Def => {
                if let TokenType::Identifier(name) = &parser.advance().token_type {
                    Ok(InstructionNode::DEF(name.clone()))
                } else {
                    Err(CompilerError::from_token(
                        ErrorCode::ExpectedButFound("Identifier".to_string(), parser.current().token_type.clone()), parser.current(), false
                    ))
                }
            }

            TokenType::Identifier(_) => {
                let token = token.clone();
                let macro_name: String;
                if let TokenType::Identifier(identifier) = &parser.current().token_type {
                    macro_name = identifier.clone();
                } else {
                    return Err(CompilerError::from_token(
                        ErrorCode::ExpectedButFound("Identifier".to_string(), parser.current().token_type.clone()), parser.current(), false
                    ));
                }
                
                // grab the arguments
                let mut arguments: Vec<Token> = Vec::new();
                while !matches!(parser.peek().token_type, TokenType::NewLine | TokenType::EndOfFile) {
                    arguments.push(parser.advance().clone());
                }
                Ok(InstructionNode::Macro(MacroHolder::Placeholder(macro_name, arguments, token)))
            }

            _ => {
                Err(CompilerError::from_token(
                    ErrorCode::ExpectedButFound("Instruction".to_string(), parser.current().token_type.clone()), parser.current(), false
                ))
            }
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
            Self::PUSH(reg_imm) | Self::OUT(reg_imm) => {
                match reg_imm {
                    RegOrImmNode::Register(_) => 1,
                    RegOrImmNode::Immediate(_) => 2
                }
            }
            Self::POP(_) => 1,
            Self::LDA(_) => 3,
            Self::JMP(pos) | Self::JZ(_, pos) | Self::JC(pos) => {
                match pos {
                    Some(_) => 3,
                    None => 1
                }
            },
            Self::DEF(_) => 0,
            Self::Macro(holder) => {
                match holder {
                    MacroHolder::Placeholder(_, _, _) => panic!("Cannot get the size of a macro placeholder"),
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
                InstructionNode::JC(imm) => {
                let instruction = node_to_instr(&self);
                match imm {
                    None => compiler.first_byte(instruction, false, None),
                    Some(val) => {
                        compiler.first_byte(instruction, true, None);
                        val.compile(compiler, false)
                    }
                }
            }

            InstructionNode::JZ(reg, imm) => {
                match imm {
                    None => compiler.first_byte(Instruction::JZ, false, Some(reg.0)),
                    Some(val) => {
                        compiler.first_byte(Instruction::JZ, true, Some(reg.0));
                        val.compile(compiler, false)
                    }
                }
            }
            
            InstructionNode::HLT => compiler.first_byte(Instruction::HLT, false, None),

            InstructionNode::Macro(holder) => {
                match holder {
                    MacroHolder::Placeholder(_, _, _) => panic!("Cannot compile a macro placeholder"),
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
        InstructionNode::JC(_) => Instruction::JC,
        InstructionNode::SUB(_, _) => Instruction::SUB,
        InstructionNode::OUT(_) => Instruction::OUT,
        InstructionNode::HLT => Instruction::HLT,
        InstructionNode::Macro(_) => panic!("Cannot convert a macro instruction node to an opcode"),
        InstructionNode::DEF(_) => panic!("Cannot convert a def instruction node to an opcode"),
    }
}
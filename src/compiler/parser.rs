use core::panic;
use std::process::id;

use crate::compiler::{lexer::Instruction, parser};

use super::lexer::{Register, Token, TokenType};

pub fn parse(tokens: Vec<Token>) -> ProgramNode {
    let mut parser = Parser::new(tokens);
    ProgramNode::populate(&mut parser)
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0
        }
    }

    /// Returns the current token
    fn current(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Returns the next token
    fn peek(&self) -> &Token {
        &self.tokens[self.current + 1]
    }

    /// Returns the current token and advances to the next token
    fn advance(&mut self) -> &Token {
        if (self.is_at_end()) {
            return self.tokens.last().expect("No tokens");
        }
        
        self.current += 1;
        self.tokens.get(self.current - 1).expect("Out of bounds")
    }

    /// Returns true if at there are no more tokens
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    /// Skips NewLine tokens
    fn skip_new_lines(&mut self) {
        while matches!(self.current().token_type, TokenType::NewLine) {
            println!("advance new line {:?}", self.current().token_type);
            self.advance();
        }
    }
}

trait Node {
    fn populate(parser: &mut Parser) -> Self where Self: Sized;
    fn get_size(&self) -> i32;
}

#[derive(Debug)]
pub struct ProgramNode {
    subroutines: Vec<SubroutineNode>,
    macros: Vec<MacroNode>
}

impl Node for ProgramNode {
    fn populate(parser: &mut Parser) -> ProgramNode {
        let mut subroutines: Vec<SubroutineNode> = Vec::new();
        let mut macros: Vec<MacroNode> = Vec::new();

        'parser: while !parser.is_at_end() {
            parser.skip_new_lines();

            match parser.peek().token_type {
                TokenType::EndOfFile => break 'parser,
                TokenType::Macro => macros.push(MacroNode::populate(parser)),
                _ | TokenType::Identifier(_) => subroutines.push(SubroutineNode::populate(parser)),
                //_ => panic!("Invalid token {:?}", parser.peek().token_type)
            }
        }

        ProgramNode {
            subroutines,
            macros
        }
    }

    fn get_size(&self) -> i32 {
        let mut size = 0;

        for node in &self.subroutines {
            size += node.get_size();
        }

        for node in &self.macros {
            size += node.get_size();
        }

        size
    }
}

#[derive(Debug)]
struct SubroutineNode {
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
            if !matches!(parser.advance().token_type, TokenType::Indent(_)) {
                break;
            }
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

#[derive(Debug)]
struct MacroNode(Vec<InstructionNode>);

impl Node for MacroNode {
    fn populate(parser: &mut Parser) -> Self {
        todo!()
    }

    fn get_size(&self) -> i32 {
        todo!()
    }
}

#[derive(Debug)]
enum InstructionNode {
    NOP,
    LW(RegisterNode, Option<NumberNode>),
    SW(RegOrImmNode, Option<NumberNode>),
    MW(RegisterNode, RegOrImmNode),
    PUSH(RegOrImmNode),
    POP(RegisterNode),
    LDA(NumberNode),
    JMP(Option<PlaceholderNode>),
    JZ(RegisterNode, Option<PlaceholderNode>),
    JO(Option<PlaceholderNode>),
    ADD(RegisterNode, RegOrImmNode),
    SUB(RegisterNode, RegOrImmNode),
    TEL(RegOrImmNode),
    OUT(RegOrImmNode),
    HLT,

    //Macro(String, Vec<Box<dyn Node>>)
}

impl Node for InstructionNode {
    fn populate(parser: &mut Parser) -> Self {
        let token = parser.advance();

        match token.token_type {
            TokenType::Instruction(Instruction::NOP) => InstructionNode::NOP,
            TokenType::Instruction(Instruction::LW) => {
                let register = RegisterNode::populate(parser);
                let number: Option<NumberNode>;

                if matches!(parser.peek().token_type, TokenType::Number(_)) {
                    number = Some(NumberNode::populate(parser))
                } else {
                    number = None;
                }
                
                InstructionNode::LW(register, number)
            }
            TokenType::Instruction(Instruction::HLT) => InstructionNode::HLT,
            _ => panic!("Invalid token. Expected instruction node")
        }
    }

    fn get_size(&self) -> i32 {
        todo!()
    }
}

#[derive(Debug)]
struct PlaceholderNode(String);

#[derive(Debug)]
enum NumberNode {
    Imm8(u8),
    Imm16(u16)
}

impl Node for NumberNode {
    fn populate(parser: &mut Parser) -> Self where Self: Sized {
        todo!()
    }

    fn get_size(&self) -> i32 {
        todo!()
    }
}

#[derive(Debug)]
enum RegOrImmNode {
    Number(NumberNode),
    Register(RegisterNode)
}

#[derive(Debug)]
enum RegisterNode {
    A,
    B,
    H,
    L
}

impl Node for RegisterNode {
    fn populate(parser: &mut Parser) -> RegisterNode {
        match parser.advance().token_type {
            TokenType::Register(Register::A) => RegisterNode::A,
            TokenType::Register(Register::B) => RegisterNode::B,
            TokenType::Register(Register::H) => RegisterNode::H,
            TokenType::Register(Register::L) => RegisterNode::L,
            _ => panic!("Invalid token. Expected register")
        }
    }

    fn get_size(&self) -> i32 {
        panic!("Requesting the size of a register is not a valid operation")
    }
}
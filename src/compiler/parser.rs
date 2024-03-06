use core::panic;

use crate::compiler::{lexer::Instruction, parser};

use super::lexer::{Token, TokenType};

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
        &self.tokens.get(self.current).expect("Out of bounds")
    }

    /// Returns the next token
    fn peek(&self) -> &Token {
        &self.tokens.get(self.current + 1).expect("Out of bounds")
    }

    /// Returns the current token and advances to the next token
    fn advance(&mut self) -> &Token {
        let token = self.current();
        self.current += 1;

        token
    }

    /// Returns true if at there are no more tokens
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    /// Skips NewLine tokens
    fn skip_new_lines(&mut self) {
        while self.peek().token_type == TokenType::NewLine {
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

        while !parser.is_at_end() {
            parser.skip_new_lines();

            //if parser.check(TokenType::EndOfFile.type_id()) {
            //    println!("end file");
            //    break;
            //} else if parser.check(TokenType::Macro.type_id()) {
            //    println!("macro");
            //    // macro
            //    macros.push(MacroNode::populate(parser));
            //    continue;
            //} else {
            //    println!("subroutine");
            //    // subroutine
            //    subroutines.push(SubroutineNode::populate(parser));
            //}
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
        // get identifier
        if matches!(parser)
        
        //let identifier = &parser.expect(TokenType::Identifier(_)).token_type;

        
        let name: String;
        if let TokenType::Identifier(n) = identifier {
            name = n.clone();
        } else {
            panic!("Non Identifier token passed parser.expect");
        }

        //parser.expect(TokenType::Colon);

        let mut instructions = Vec::<InstructionNode>::new();

        parser.skip_new_lines();
        //parser.expect(TokenType::Indent);
        
        while !parser.is_at_end() {
            parser.advance();
            let token = parser.current();
            match &token.token_type {
                TokenType::NewLine => {
                    parser.skip_new_lines();
                    //parser.expect(TokenType::Indent.type_id());
                }
                
                TokenType::Instruction(_) | TokenType::Identifier(_) => {
                    instructions.push(InstructionNode::populate(parser));
                }

                TokenType::EndOfFile => break,

                _ => panic!("Unexpected token {:?}", token.token_type)
            }
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
        let token = parser.current();
        if let TokenType::Instruction(instruction) = &token.token_type {
            return match instruction {
                Instruction::NOP => InstructionNode::NOP,
                Instruction::LW => {
                    let register = RegisterNode::populate(parser);
                    //parser.expect(TokenType::Comma.type_id()); // advance past comma
                    
                    let immediate: Option<NumberNode>;
                    //if parser.check(TokenType::Number.type_id()) {
                    //    immediate = Some(NumberNode::populate(parser));
                    //} else {
                    //    immediate = None;
                    //}

                    InstructionNode::LW(register, immediate)
                }
                Instruction::HLT => InstructionNode::HLT,

                _ => todo!()
            }
        }
        
        todo!()
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
        todo!()
    }

    fn get_size(&self) -> i32 {
        panic!("Requesting the size of a register is not a valid operation")
    }
}
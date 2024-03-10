mod number_nodes;
mod instruction_node;
mod macros;

use core::panic;
use std::collections::HashMap;

use self::{instruction_node::InstructionNode, macros::Macro, number_nodes::{Imm16, Imm8}};

use super::lexer::{Register, Token, TokenType};

pub fn parse(tokens: Vec<Token>) -> ProgramNode {
    let mut parser = Parser::new(tokens);
    ProgramNode::populate(&mut parser)
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
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
        if self.current == 0 {
            &self.tokens.first().unwrap()
        } else {
            &self.tokens[self.current - 1]
        }
    }

    /// Returns the next token
    fn peek(&self) -> &Token {
        if self.is_at_end() {
            &self.tokens.last().unwrap()
        } else {
            &self.tokens[self.current]
        }
    }

    /// Returns the current token and advances to the next token
    fn advance(&mut self) -> &Token {
        if self.is_at_end() {
            return self.tokens.last().unwrap();
        }
        
        self.current += 1;
        self.tokens.get(self.current - 1).expect("Out of bounds")
    }

    fn insert(&mut self, tokens: Vec<Token>) {
        for i in 0..tokens.len() {
            self.tokens.insert(self.current + i, tokens[i].clone());
        }
    }

    /// Returns true if at there are no more tokens
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    /// Skips NewLine tokens
    fn skip_new_lines(&mut self) {
        while matches!(self.peek().token_type, TokenType::NewLine) {
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
    macros: Vec<Macro>
}

impl Node for ProgramNode {
    fn populate(parser: &mut Parser) -> ProgramNode {
        let mut subroutines: Vec<SubroutineNode> = Vec::new();
        let mut macros: Vec<Macro> = Vec::new();

        'parser: while !parser.is_at_end() {
            parser.skip_new_lines();
            println!("{:?}", parser.peek());

            match parser.peek().token_type {
                TokenType::EndOfFile => break 'parser,
                TokenType::Macro => macros.push(Macro::populate(parser)),
                TokenType::Identifier(_) => subroutines.push(SubroutineNode::populate(parser)),
                _ => panic!("Expected a macro, subroutine, or end of file")
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
            if !matches!(parser.peek().token_type, TokenType::Indent(_)) {
                break;
            }
            
            parser.advance(); // advance past indent

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
struct PlaceholderNode(String);

impl Node for PlaceholderNode {
    fn populate(parser: &mut Parser) -> PlaceholderNode {
        let identifier = parser.advance();
        if let TokenType::Identifier(str) = &identifier.token_type {
            PlaceholderNode(String::from(str))
        } else {
            panic!("Expected identifier");
        }
    }

    fn get_size(&self) -> i32 {
        todo!()
    }
}

#[derive(Debug)]
enum RegOrImmNode {
    Immediate(Imm8),
    Register(RegisterNode)
}

impl Node for RegOrImmNode {
    fn populate(parser: &mut Parser) -> RegOrImmNode {
        let token = parser.peek();
        match token.token_type {
            TokenType::Register(_) => {
                RegOrImmNode::Register(RegisterNode::populate(parser))
            },
            TokenType::Number(_) => {
                RegOrImmNode::Immediate(Imm8::populate(parser))
            },
            _ => panic!("Expected Register or Immediate 8")
        }
    }

    fn get_size(&self) -> i32 {
        todo!()
    }
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
            _ => panic!("Invalid token {:?}. Expected register", parser.advance().token_type)
        }
    }

    fn get_size(&self) -> i32 {
        panic!("Requesting the size of a register is not a valid operation")
    }
}
pub mod number_nodes;
mod instruction_node;
mod subroutine_node;
pub mod program_node;
mod macros;

use core::panic;
use self::{number_nodes::{Imm16, Imm8}, program_node::ProgramNode};

use super::{compiler::Compiler, lexer::{Register, Token, TokenType}};

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

pub trait Node {
    fn populate(parser: &mut Parser) -> Self where Self: Sized;
    fn get_size(&self) -> i32;
    fn compile(&self, compiler: &mut Compiler);
}
// misc nodes

#[derive(Debug)]
enum PlaceholderOrImmNode {
    PlaceholderNode(PlaceholderNode),
    Imm16(Imm16)
}

impl Node for PlaceholderOrImmNode {
    fn populate(parser: &mut Parser) -> Self where Self: Sized {
        match parser.peek().token_type {
            TokenType::Identifier(_) => PlaceholderOrImmNode::PlaceholderNode(PlaceholderNode::populate(parser)),
            TokenType::Number(_) => PlaceholderOrImmNode::Imm16(Imm16::populate(parser)),
            _ => panic!("Expected Identifier or Number")
        }
    }

    fn get_size(&self) -> i32 {
        return 2;
    }

    fn compile(&self, compiler: &mut Compiler) {
        match self {
            PlaceholderOrImmNode::PlaceholderNode(node) => node.compile(compiler),
            PlaceholderOrImmNode::Imm16(node) => node.compile(compiler)
        }
    }
}

#[derive(Debug)]
struct PlaceholderNode {
    name: String
}

impl Node for PlaceholderNode {
    fn populate(parser: &mut Parser) -> PlaceholderNode {
        let identifier = parser.advance();
        if let TokenType::Identifier(str) = &identifier.token_type {
            PlaceholderNode {
                name: String::from(str)
            }
        } else {
            panic!("Expected identifier");
        }
    }

    fn get_size(&self) -> i32 {
        2
    }

    fn compile(&self, compiler: &mut Compiler) {
        if compiler.scope.contains_key(&self.name) {
            let value = compiler.scope.get(&self.name).unwrap().clone();
            value.compile(compiler);
        } else {
            panic!("Placeholder does not exist")
        }
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
        panic!("Requesting the size of a RegOrImmNode is not a valid operation")
    }
    
    fn compile(&self, compiler: &mut Compiler) {
        panic!("Compiling a RegOrImmNode is not a valid operation");
    }
}

#[derive(Debug)]
struct RegisterNode(Register);

impl Node for RegisterNode {
    fn populate(parser: &mut Parser) -> RegisterNode {
        match &parser.advance().token_type {
            TokenType::Register(reg) => RegisterNode(reg.clone()),
            _ => panic!("Invalid token {:?}. Expected register", parser.advance().token_type)
        }
    }

    fn get_size(&self) -> i32 {
        panic!("Requesting the size of a register is not a valid operation")
    }
    
    fn compile(&self, compiler: &mut Compiler) {
        panic!("Compiling a RegisterNode is not a valid operation")
    }
}
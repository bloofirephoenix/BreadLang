use std::collections::HashMap;

use crate::compiling::{compiler::Compiler, lexer::{Token, TokenType}, parser::subroutine_node::get_instructions};

use super::{instruction_node::InstructionNode, number_nodes::Imm16, Node, Parser};

#[derive(Debug)]
pub struct Macro {
    pub name: String,
    tokens: Vec<Token>,
    arguments: HashMap<String, usize>
}

impl Macro {
    pub fn populate(parser: &mut Parser) -> Macro {
        // expect macro
        if !matches!(parser.advance().token_type, TokenType::Macro) {
            panic!("Expected @macro");
        }

        // expect new line
        if !matches!(parser.advance().token_type, TokenType::NewLine) {
            panic!("Expected a new line");
        }

        // expect a silly lil guy (identifier)
        let name: String;
        if let TokenType::Identifier(n) = &parser.advance().token_type {
            name = n.clone();
        } else {
            panic!("Expected an identifier");
        }

        // expect open parenthesis
        if !matches!(parser.advance().token_type, TokenType::OpenParenthesis) {
            panic!("Expected an open parenthesis");
        }

        // grab arguments
        let mut arguments: HashMap<String, usize> = HashMap::new();
        let mut index = 0;
        loop {
            println!("{:?}", parser.peek());
            match &parser.peek().token_type {
                TokenType::CloseParenthesis => {
                    parser.advance(); // advance past close parenthesis
                    break;
                },
                TokenType::Identifier(arg) => {
                    arguments.insert(arg.clone(), index);
                    index += 1;
                    
                    parser.advance();
                },
                _ => panic!("Expected a close parenthesis or identifier")
            }
        }

        // expect colon
        if !matches!(parser.advance().token_type, TokenType::Colon) {
            panic!("Expected a colon");
        }
        
        // expect new line
        if !matches!(parser.advance().token_type, TokenType::NewLine) {
            panic!("Expected new line")
        }

        // collect all the tokens inside the macro
        let mut tokens = Vec::<Token>::new();

        // cry
        'token_collection: while !parser.is_at_end() {
            'new_lines: loop {
                //parser.skip_new_lines();
                
                while matches!(parser.peek().token_type, TokenType::NewLine) {
                    tokens.push(parser.advance().clone());
                }

                if !matches!(parser.peek().token_type, TokenType::Indent(_)) {
                    break 'token_collection;
                }
                
                tokens.push(parser.advance().clone()); // advance past indent

                if !matches!(parser.peek().token_type, TokenType::NewLine) {
                    break 'new_lines;
                }
            }

            while !parser.is_at_end() && !matches!(parser.peek().token_type, TokenType::NewLine) {
                tokens.push(parser.advance().clone());
            }
        }

        Macro {
            name,
            tokens,
            arguments
        }
    }
}

#[derive(Debug)]
pub enum MacroHolder {
    Placeholder(String, Vec<Token>),
    Macro(MacroNode)
}

#[derive(Debug)]
pub struct MacroNode {
    instructions: Vec<InstructionNode>,
    placeholders: HashMap<String, Imm16>
}

// similar functions to Node
impl MacroNode {
    pub fn populate(definition: &Macro, args: &Vec<Token>) -> MacroNode {
        // replace arguments
        let mut tokens: Vec<Token> = Vec::new();

        for token in &definition.tokens {
            if let TokenType::Identifier(identifier) = &token.token_type {
                if definition.arguments.contains_key(identifier) {
                    tokens.push(args.get(*definition.arguments.get(identifier).unwrap()).unwrap().clone());
                    continue;
                }
            }
            tokens.push(token.clone());
        }

        tokens.push(Token::new(TokenType::EndOfFile, -1));

        let mut parser = Parser::new(tokens);

        MacroNode {
            instructions: get_instructions(&mut parser),
            placeholders: HashMap::new()
        }
    }

    pub fn get_size(&self) -> i32 {
        let mut size = 0;
        
        for node in &self.instructions {
            size += node.get_size();
        }

        size
    }

    pub fn compile(&self, compiler: &mut Compiler) {
        compiler.scope = self.placeholders.clone();
        for instructions in &self.instructions {
            instructions.compile(compiler);
        }
    }

    pub fn calculate_placeholders(&mut self, position: &mut u16, placeholders: &HashMap<String, Imm16>) {
        self.placeholders = placeholders.clone();
        for instruction in &self.instructions {
            if let InstructionNode::DEF(name) = instruction {
                self.placeholders.insert(name.clone(), Imm16::from(*position));
            } else {
                *position += instruction.get_size() as u16;
            }
        }
    }
}
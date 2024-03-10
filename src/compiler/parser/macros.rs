use std::collections::HashMap;

use uuid::Uuid;

use crate::compiler::lexer::{Token, TokenType};

use super::Parser;

#[derive(Debug)]
pub struct Macro {
    name: String,
    instructions: Vec<Token>,
    arguments: Vec<String>
}

impl Macro {
    pub fn populate(parser: &mut Parser) -> Self {
        // expect macro
        if !matches!(parser.advance().token_type, TokenType::Macro) {
            panic!("Expected macro");
        }

        // expect newline
        if !matches!(parser.advance().token_type, TokenType::NewLine) {
            panic!("Expected new line");
        }
        
        // identifier
        let name: String;
        if let TokenType::Identifier(n) = &parser.advance().token_type {
            name = n.clone();
        } else {
            panic!("Expected Identifier");
        }
        
        // arguments
        let mut arguments: Vec<String> = Vec::new();

        // expect (
        if !matches!(parser.advance().token_type, TokenType::OpenParenthesis) {
            panic!("Expected open parenthesis");
        }

        while !matches!(parser.peek().token_type, TokenType::CloseParenthesis) {
            // grab the argument names
            let arg = parser.advance();
            if let TokenType::Identifier(arg_name) = &arg.token_type {
                arguments.push(arg_name.clone());
            } else {
                panic!("Expected an identifier or a close parenthesis");
            }
        }
        parser.advance(); // advance past )

        // expect colon
        if !matches!(parser.advance().token_type, TokenType::Colon) {
            panic!("Expected colon {:?}", parser.current().token_type)
        }

        let mut instructions: Vec<Token> = Vec::new();

        while !parser.is_at_end() {
            parser.skip_new_lines();
            if !matches!(parser.advance().token_type, TokenType::Indent(_)) {
                break;
            }
            instructions.push(parser.advance().clone());
        }

        Macro {
            name,
            arguments,
            instructions
        }
    }

    fn get_tokens(&self, arguments: Vec<Token>) -> Vec<Token> {
        let mut placeholders: HashMap<String, String> = HashMap::new();
        let mut tokens = Vec::<Token>::new();

        for token in &self.instructions {
            if let TokenType::Identifier(identifier) = &token.token_type {
                // arguments
                if self.arguments.contains(identifier) {
                    let index = self.arguments.iter().position(|r| r == identifier).unwrap();
                    tokens.push(arguments[index].clone());
                    continue;
                }

                // replace placeholder names to prevent conflict
                let new_id: String;
                if placeholders.contains_key(identifier) {
                    new_id = placeholders.get(identifier).unwrap().clone();
                } else {
                    new_id = Uuid::new_v4().to_string();
                    placeholders.insert(identifier.to_string(), new_id.clone());
                }
                tokens.push(Token::new(TokenType::Identifier(new_id), token.line));
            }

            tokens.push(token.clone());
        }

        tokens
    }
}
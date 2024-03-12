use crate::compiling::lexer::{Token, TokenType};

use super::Parser;

pub struct Macro {
    name: String,
    tokens: Vec<Token>,
    arguments: Vec<String>
}

impl Macro {
    fn populate(parser: &mut Parser) -> Macro {
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
        let mut arguments: Vec<String> = Vec::new();
        loop {
            match &parser.peek().token_type {
                TokenType::CloseParenthesis => {
                    parser.advance(); // advance past close parenthesis
                    break;
                },
                TokenType::Identifier(arg) => {
                    arguments.push(arg.clone())
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
                parser.skip_new_lines();
                
                if !matches!(parser.peek().token_type, TokenType::Indent(_)) {
                    break 'token_collection;
                }
                
                parser.advance(); // advance past indent

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
pub mod number_nodes;
mod instruction_node;
mod subroutine_node;
pub mod program_node;
mod register_node;
mod macros;
mod reg_imm_node;
mod placeholder_node;

use std::{fs, path::Path};
use self::program_node::ProgramNode;

use super::{compiler::Compiler, error_handler::{CompilerError, ErrorCode}, lexer::{scan_tokens, Token, TokenType}};

pub fn parse(tokens: Vec<Token>, file: String) -> Result<ProgramNode, Vec<CompilerError>> {
    let mut parser = Parser::new(tokens, file);
    ProgramNode::populate(&mut parser)
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    files: Vec<String>
}

impl Parser {
    fn new(tokens: Vec<Token>, file: String) -> Parser {
        Parser {
            tokens,
            current: 0,
            files: vec![file]
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

    // files
    fn add_file(&mut self, file: &String) -> Result<(), CompilerError> {
        if !self.files.contains(file) {
            let path = Path::new(&file);

            if !path.exists() {
                return Err(CompilerError::new(
                    ErrorCode::NoSuchFile(file.clone()), &self.current().file, self.current().line, true)
                );
            }

            self.files.push(file.clone());

            // read a file
            let contents = fs::read_to_string(file)
                .expect(&format!("Unable to read file {}", file));
            
            

            let mut tokens: Vec<Token>;

            match scan_tokens(contents, file.clone()) {
                Ok(t) => tokens = t,
                Err(e) => return Err(e),
            }

            tokens.remove(tokens.len() - 1); // remove end of file token
            self.insert(tokens);
        }

        return Ok(());
    }
}

pub trait Node {
    fn populate(parser: &mut Parser) -> Result<Self, CompilerError> where Self: Sized;
    fn get_size(&self) -> i32;
    fn compile(&self, compiler: &mut Compiler);
}
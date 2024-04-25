use std::collections::HashMap;

use crate::compiling::{error_handler::{self, CompilerError, ErrorCode}, lexer::{Token, TokenType}};

use super::{macros::Macro, number_nodes::Imm16, subroutine_node::SubroutineNode, Node, Parser};

#[derive(Debug)]
pub struct ProgramNode {
    subroutines: Vec<SubroutineNode>,
    placeholders: HashMap<String, Imm16>,
}

impl ProgramNode {
    pub fn populate(parser: &mut Parser) -> Result<ProgramNode, Vec<CompilerError>> {
        let mut subroutines: Vec<SubroutineNode> = Vec::new();
        let mut macros: HashMap<String, Macro> = HashMap::new();
        let mut errors: Vec<CompilerError> = Vec::new();
        let mut placeholders: HashMap<String, Imm16> = HashMap::new();

        let mut main = false;

        'parser: while !parser.is_at_end() {
            parser.skip_new_lines();

            let file = parser.peek().file.clone();

            match parser.peek().token_type {
                TokenType::EndOfFile => break 'parser,
                TokenType::Macro => {
                    match Macro::populate(parser) {
                        Ok(m) => {
                            macros.insert(m.name.clone(), m);
                        },
                        Err(e) => return Err(vec![e]),
                    }
                }
                TokenType::Identifier(_) => {
                    let sub = SubroutineNode::populate(parser);
                    match sub {
                        Ok(sub) => {
                            if sub.name == "main" && file == "main.bread" {
                                main = true;
                                subroutines.insert(0, sub)
                            } else {
                                subroutines.push(sub)
                            }
                        },
                        Err(mut err) => {
                            if error_handler::has_critical(&err) {
                                errors.append(&mut err);
                                return Err(errors);
                            } else {
                                errors.append(&mut err);
                            }
                        }
                    }
                },
                TokenType::Include => {
                    parser.advance(); // advance past include

                    let mut path: String = String::from("");

                    while let TokenType::Identifier(p) = &parser.advance().token_type {
                        path += p;
                        path += " ";
                    }

                    if path.is_empty() {
                        errors.push(
                            CompilerError::expected("valid path", parser.current(), true)
                        );
                        return Err(errors);
                    }

                    if let Err(e) = parser.add_file(&String::from(path.trim())) {
                        errors.push(e);
                        return Err(errors);
                    }
                },
                TokenType::Constant => {
                    parser.advance(); // advance past constant
                    let name = if let TokenType::Identifier(name) = &parser.advance().token_type {
                        name.clone()
                    } else {
                        return Err(errors)
                    };
                    let value = match Imm16::populate(parser) {
                        Ok(v) => v,
                        Err(e) => {
                            errors.push(e);
                            return Err(errors);
                        }
                    };

                    placeholders.insert(name, value);
                }
                _ => {
                    errors.push(
                        CompilerError::expected("macro, subroutine, include, or end of file", parser.peek(), false));
                    parser.advance();
                }
            }
        }

        if !main {
            errors.push(CompilerError::new(ErrorCode::NoMainSubroutine, &"main.bread".to_string(), 0, true));
            return Err(errors);
        }

        // populate macros
        for sub in &mut subroutines {
            if let Err(mut e) = sub.populate_macros(&macros) {
                if error_handler::has_critical(&e) {
                    errors.append(&mut e);
                    return Err(errors);
                } else {
                    errors.append(&mut e);
                }
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        let mut node = ProgramNode {
            subroutines,
            placeholders,
        };

        node.calculate_placeholders();

        Ok(node)
    }
        
    pub fn compile(&self, compiler: &mut crate::compiling::compiler::Compiler) {
        compiler.scope = self.placeholders.clone();

        for sub in &self.subroutines {
            sub.compile(compiler);
        }
    }
}

impl ProgramNode {
    pub fn calculate_placeholders(&mut self) {
        let mut position = 0;
        for subroutine in &self.subroutines {
            self.placeholders.insert(subroutine.name.clone(), Imm16::from(position));
            position += subroutine.get_size() as u16;
        }
        position = 0;

        for subroutine in &mut self.subroutines {
            subroutine.calculate_placeholders(&mut position, &self.placeholders);
        }
    }
}
use core::panic;

use crate::compiler::lexer::TokenType;

use super::{Node, Parser};

#[derive(Debug)]
pub struct Imm8(u8);

#[derive(Debug)]
pub struct Imm16(u16);

impl Node for Imm16 {
    fn populate(parser: &mut Parser) -> Self where Self: Sized {
        let num = get_number(parser);
        panic!("{}", num);
    }

    fn get_size(&self) -> i32 {
        todo!()
    }
}

fn get_number(parser: &mut Parser) -> i32 {
    let token = parser.advance();
    if let TokenType::Number(num) = &token.token_type {
        let num = num.replace("_", "");
        let result;
        if num.starts_with("0x") {
            // hexadecimal
            result = i32::from_str_radix(&num[2..], 16);
        } else if num.starts_with("0b") {
            // binary
            result = i32::from_str_radix(&num[2..], 2);
        } else {
            // decimal
            result = i32::from_str_radix(&num, 10);
        }

        return match result {
            Ok(num) => num,
            Err(e) => panic!("Invalid number: {}", e)
        }
    }
    panic!("Expected a number")
}
use std::collections::HashMap;

use super::{parser::number_nodes::Imm16, Instruction, Register};

pub struct Compiler {
    pub bytes: Vec<u8>,
    pub scope: HashMap<String, Imm16>
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            bytes: Vec::new(),
            scope: HashMap::new()
        }
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.bytes.push(byte)
    }

    pub fn add_bytes(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.add_byte(*b);
        }
    }

    pub fn first_byte(&mut self, instruction: Instruction, immediate: bool, register: Option<Register>) {
        let opcode = instruction as u8;
        let mut byte: u8 = 0b00000_0_00;
        
        byte |= opcode << 3;
        if immediate {
            byte |= 0b0000_1_00;
        }
        if let Some(reg) = register {
            let reg = reg as u8;
            byte |= reg;
        }

        self.add_byte(byte);
    }
    
    pub fn two_bytes(&mut self, instruction: Instruction, immediate: bool, register_a: Register, register_b: Register) {
        self.first_byte(instruction, immediate, Some(register_a));
        
        let b_code = register_b as u8;
        let byte = 0b00_000000 | b_code << 6;
        self.add_byte(byte);
    }
}
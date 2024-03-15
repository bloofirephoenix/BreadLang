use std::{collections::HashMap, thread::sleep, time::Duration};

use enum_primitive::FromPrimitive;

use crate::compiling::{Instruction, Register};

pub struct State {
    registers: HashMap<Register, u8>,
    program_counter: u16,
    stack_pointer: u8,
    memory: HashMap<u16, u8>,
    rom: Vec<u8>,
    overflow: bool
}

impl State {
    fn get_byte(&self) -> u8 {
        if (self.program_counter as usize) < self.rom.len() {
            *self.rom.get(self.program_counter as usize).unwrap()
        } else {
            return 0b11111111;
        }
    }

    fn increment(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(1);
    }

    fn get_memory(&self, address: u16) -> u8 {
        if self.memory.contains_key(&address) {
            self.memory[&address]
        } else {
            0
        }
    }
}

pub fn run(rom: Vec<u8>, debug: bool) {
    let mut state = State {
        registers: HashMap::new(),
        program_counter: 0,
        stack_pointer: 0,
        memory: HashMap::new(),
        overflow: false,
        rom
    };
    state.registers.insert(Register::A, 0);
    state.registers.insert(Register::B, 0);
    state.registers.insert(Register::H, 0);
    state.registers.insert(Register::L, 0);

    'main: loop {
        let byte = state.get_byte();
        state.increment();
    
        let instruction = (byte & 0b11111_0_00) >> 3;
        let immediate: bool;
        if byte & 0b00000_1_00 == 0 {
            immediate = false;
        } else {
            immediate = true;
        }
    
        let reg_a = Register::from_u8(byte & 0b00000_0_11).unwrap();
        let instruction = Instruction::from_u8(instruction).unwrap();
        match instruction {
            Instruction::LW => {
                let address = get_word16(&mut state, immediate);
                let word = state.get_memory(address);
                state.registers.insert(reg_a, word);

                if debug {
                    println!("{:?} = MEM(a={},w={})", reg_a, address, word);
                }
            },
            Instruction::SW => {
                let address = get_word16(&mut state, immediate);
                let word = state.registers[&reg_a];
                state.memory.insert(address, word);

                if debug {
                    println!("MEM(a={}) = {}", address, word);
                }
            },
            Instruction::MW => {
                let value: u8;
                if immediate {
                    value = state.get_byte();
                    state.increment();

                    if debug {
                        println!("{:?} = {}", reg_a, value);
                    }
                } else {
                    let reg_b = get_second_register(&mut state);
                    state.increment();
                    value = state.registers[&reg_b];

                    if debug {
                        println!("{:?} = {:?}({})", reg_a, reg_b, value);
                    }
                }
                state.registers.insert(reg_a, value);
            },
            Instruction::PUSH => {
                let word: u8;
                if immediate {
                    word = state.get_byte();
                    state.increment();

                    if debug {
                        println!("PUSH(w={})", word);
                    }
                } else {
                    word = state.registers[&reg_a];
                    if debug {
                        println!("PUSH(r={:?}, w={})", reg_a, word);
                    }
                }
                state.memory.insert(state.stack_pointer as u16, word);
                state.stack_pointer = state.stack_pointer.wrapping_add(1);
            },
            Instruction::POP => {
                state.stack_pointer = state.stack_pointer.wrapping_sub(1);
                let word = state.get_memory(state.stack_pointer as u16);
                state.registers.insert(reg_a, word);

                if debug {
                    println!("{:?} = POP({})", reg_a, word);
                }
            },
            Instruction::LDA => {
                let h = state.get_byte();
                state.registers.insert(Register::H, h);
                state.increment();

                let l = state.get_byte();
                state.registers.insert(Register::L, l);
                state.increment();

                if debug {
                    println!("LDA(h={}, l={})", h, l);
                }
            },
            Instruction::JMP => {
                let address = get_word16(&mut state, immediate);
                state.program_counter = address;

                if debug {
                    println!("JMP({})", address);
                }
            },
            Instruction::JZ => {
                let address = get_word16(&mut state, immediate);
                let jumping: bool;
                if state.registers[&reg_a] == 0 {
                    jumping = true;
                    state.program_counter = address;
                } else {
                    jumping = false;
                }

                if debug {
                    println!("JZ({}, j={})", address, jumping);
                }
            },
            Instruction::JO => {
                let address = get_word16(&mut state, immediate);

                let jumping: bool;
                if state.overflow {
                    state.program_counter = address;
                    jumping = true;
                } else {
                    jumping = false;
                }

                if debug {
                    println!("JO({}, j={})", address, jumping);
                }
            },
            Instruction::ADD => {
                let left = state.registers[&reg_a];
                let right: u8;
                if immediate {
                    right = state.get_byte();
                    state.increment();

                    if debug {
                        println!("{:?} = {:?}({}) + {}", reg_a, reg_a, left, right);
                    }
                } else {
                    let reg_b = get_second_register(&mut state);
                    state.increment();
                    right = state.registers[&reg_b];

                    if debug {
                        println!("{:?} = {:?}({}) + {:?}({})", reg_a, reg_a, left, reg_b, right);
                    }
                }
                let result = add(left, right, 0, &mut state);
                state.registers.insert(reg_a, result);
            },
            Instruction::SUB => {
                // to make sure overflow is set correctly, we will add a 2s complement instead of subtract
                let left = state.registers[&reg_a];
                let right: u8;
                if immediate {
                    right = state.get_byte();
                    state.increment();
                    if debug {
                        println!("{:?} = {:?}({}) - {:?}", reg_a, reg_a, left, right);
                    }
                } else {
                    let reg_b = get_second_register(&mut state);
                    state.increment();
                    right = state.registers[&reg_b];
                    if debug {
                        println!("{:?} = {:?}({}) - {:?}({})", reg_a, reg_a, left, reg_b, right);
                    }
                }

                let result = add(left, !right, 1, &mut state);
                state.registers.insert(reg_a, result);
            },
            Instruction::TEL => todo!(),
            Instruction::OUT => {
                let value: u8;
                if immediate {
                    value = state.get_byte();
                    state.increment();
                    if debug {
                        println!("OUT {}", value);
                    }
                } else {
                    value = state.registers[&reg_a];
                    if debug {
                        println!("OUT {:?}({})", reg_a, value);
                    }
                }
                if !debug {
                    println!("OUT {}", value);
                }
            },
            Instruction::HLT => {
                if debug {
                    println!("HLT");
                }
                break 'main
            },
            Instruction::NOP => {
                if debug {
                    println!("DEBUG");
                }
            },
        }

        if debug {
            sleep(Duration::from_millis(20));
        }
    }
}

fn get_word16(state: &mut State, immediate: bool) -> u16 {
    let high: u16;
    let low: u16;
    if immediate {
        high = state.get_byte() as u16;
        state.increment();
        low = state.get_byte() as u16;
        state.increment();
    } else {
        high = state.registers[&Register::H] as u16;
        low = state.registers[&Register::L] as u16;
    }
    (high << 8) | low
}

fn get_second_register(state: &mut State) -> Register {
    Register::from_u8((state.get_byte() & 0b11_000000) >> 6).unwrap()
}

fn add(left: u8, right: u8, carry: u8, state: &mut State) -> u8 {
    let with_carry = right.wrapping_add(carry);
    state.overflow = left.checked_add(with_carry) == None || right.checked_add(carry) == None;

    left.wrapping_add(with_carry)
}
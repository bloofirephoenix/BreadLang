use std::ops::{BitAnd, BitOr};

use enum_primitive::FromPrimitive;

use crate::compiling::{Instruction, Register};

enum Signal {
    // Registers
    AOut            = 0b00000000_00000000_00000000_00000001,
    AIn             = 0b00000000_00000000_00000000_00000010,
    BOut            = 0b00000000_00000000_00000000_00000100,
    BIn             = 0b00000000_00000000_00000000_00001000,
    HOut            = 0b00000000_00000000_00000000_00010000,
    HIn             = 0b00000000_00000000_00000000_00100000,
    LOut            = 0b00000000_00000000_00000000_01000000,
    LIn             = 0b00000000_00000000_00000000_10000000,
    // ALU
    AluAIn          = 0b00000000_00000000_00000001_00000000,
    AluBIn          = 0b00000000_00000000_00000010_00000000,
    Subtract        = 0b00000000_00000000_00000100_00000000,
    SumsOut         = 0b00000000_00000000_00001000_00000000,
    // Stack
    StackUp         = 0b00000000_00000000_00010000_00000000,
    StackDown       = 0b00000000_00000000_00100000_00000000,
    StackOut        = 0b00000000_00000000_01000000_00000000,
    StackIn         = 0b00000000_00000000_10000000_00000000,
    // Program Counter
    PCUp            = 0b00000000_00000001_00000000_00000000,
    PCApply         = 0b00000000_00000010_00000000_00000000,
    PCLIn           = 0b00000000_00000100_00000000_00000000,
    PCHIn           = 0b00000000_00001000_00000000_00000000,
    // ROM
    ROMOut          = 0b00000000_00010000_00000000_00000000,
    InstRegIn       = 0b00000000_00100000_00000000_00000000,
    InstRegBIn      = 0b00000000_01000000_00000000_00000000,
    // Micro Ops
    MicroOpsReset   = 0b00000000_10000000_00000000_00000000,
    // RAM
    RamLIn          = 0b00000001_00000000_00000000_00000000,
    RamHIn          = 0b00000010_00000000_00000000_00000000,
    RamOut          = 0b00000100_00000000_00000000_00000000,
    RamIn           = 0b00001000_00000000_00000000_00000000,
    RamAddrClear    = 0b00010000_00000000_00000000_00000000,
    // OUT
    DisplayIn       = 0b00100000_00000000_00000000_00000000,
    GenericOutIn    = 0b01000000_00000000_00000000_00000000,
    // MISC
    Halt            = 0b10000000_00000000_00000000_00000000,
}

enum Input {
    Instruction = 0b0000_0_0_0_0_0_00_00_0_11111,
    Immediate   = 0b0000_0_0_0_0_0_00_00_1_00000,
    RegA        = 0b0000_0_0_0_0_0_00_11_0_00000,
    RegB        = 0b0000_0_0_0_0_0_11_00_0_00000,
    Overflow    = 0b0000_0_0_0_0_1_00_00_0_00000,
    AZero       = 0b0000_0_0_0_1_0_00_00_0_00000,
    BZero       = 0b0000_0_0_1_0_0_00_00_0_00000,
    HZero       = 0b0000_0_1_0_0_0_00_00_0_00000,
    LZero       = 0b0000_1_0_0_0_0_00_00_0_00000,
    MicroOp     = 0b1111_0_0_0_0_0_00_00_0_00000
}

pub fn get_program(byte_select: u8) -> Result<Vec<u8>, String> {
    let mut program = Vec::with_capacity(0b1111_1_1_1_1_1_11_11_1_11111);

    if byte_select > 3 {
        return Err(String::from("Invalid byte select"));
    }

    for i in 0..=0b1111_1_1_1_1_1_11_11_1_11111 {
        let signals: u32 = get_signal(i);
        program.push((signals >> (byte_select * 8)) as u8)
    }
    Ok(program)
}

fn get_signal(address: u32) -> u32 {
    let instruction = address & Input::Instruction;
    let immediate = (address & Input::Immediate) >> 5;
    let reg_a = (address & Input::RegA) >> 6;
    let reg_b = (address & Input::RegB) >> 8;
    let overflow = (address & Input::Overflow) >> 10;
    let a_zero = (address & Input::AZero) >> 11;
    let b_zero = (address & Input::BZero) >> 12;
    let h_zero = (address & Input::HZero) >> 13;
    let l_zero = (address & Input::LZero) >> 14;
    let micro_op = (address & Input::MicroOp) >> 15;

    let immediate = {
        if immediate == 1 {
            true
        } else {
            false
        }
    };

    match micro_op {
        0 => Signal::InstRegIn | Signal::ROMOut,
        1 => Signal::PCUp as u32,
        _ => {
            let inst: Instruction;
            match Instruction::from_u32(instruction) {
                Some(i) => inst = i,
                None => inst = Instruction::NOP
            };
            match inst {
                Instruction::LW => {
                    let reg_a = Register::from_u32(reg_a).unwrap();
                    if immediate {
                        match micro_op {
                            2 => Signal::ROMOut | Signal::RamHIn,
                            3 => Signal::PCUp as u32,
                            4 => Signal::ROMOut | Signal::RamLIn,
                            5 => Signal::PCUp as u32,
                            6 => Signal::RamOut | get_reg_in(reg_a),
                            _ => Signal::MicroOpsReset as u32,
                        }
                    } else {
                        match micro_op {
                            2 => Signal::LOut | Signal::RamHIn,
                            3 => Signal::HOut | Signal::RamLIn,
                            4 => Signal::RamOut | get_reg_in(reg_a),
                            _ => Signal::MicroOpsReset as u32,
                        }
                    }
                },
                Instruction::SW => {
                    let reg_a = Register::from_u32(reg_a).unwrap();
                    if immediate {
                        match micro_op {
                            2 => Signal::ROMOut | Signal::RamHIn,
                            3 => Signal::PCUp as u32,
                            4 => Signal::ROMOut | Signal::RamLIn,
                            5 => Signal::PCUp as u32,
                            6 => Signal::RamIn | get_reg_out(reg_a),
                            _ => Signal::MicroOpsReset as u32,
                        }
                    } else {
                        match micro_op {
                            2 => Signal::LOut | Signal::RamHIn,
                            3 => Signal::HOut | Signal::RamLIn,
                            4 => Signal::RamIn | get_reg_out(reg_a),
                            _ => Signal::MicroOpsReset as u32,
                        }
                    }
                },
                Instruction::MW => {
                    let reg_a = Register::from_u32(reg_a).unwrap();
                    if immediate {
                        match micro_op {
                            2 => get_reg_in(reg_a) | Signal::ROMOut,
                            3 => Signal::PCUp as u32,
                            _ => Signal::MicroOpsReset as u32,
                        }
                    } else {
                        let reg_b = Register::from_u32(reg_b).unwrap();
                        match micro_op {
                            2 => Signal::InstRegBIn | Signal::ROMOut,
                            3 => Signal::PCUp as u32,
                            4 => get_reg_in(reg_a) | get_reg_out(reg_b),
                            _ => Signal::MicroOpsReset as u32,
                        }
                    }
                },
                Instruction::PUSH => {
                    if immediate {
                        match micro_op {
                            2 => Signal::RamAddrClear as u32,
                            3 => Signal::StackOut | Signal::RamLIn,
                            4 => Signal::RamIn | Signal::ROMOut,
                            5 => Signal::PCUp as u32,
                            6 => Signal::StackUp as u32,
                            _ => Signal::MicroOpsReset as u32,
                        }
                    } else {
                        let reg_a = Register::from_u32(reg_a).unwrap();
                        match micro_op {
                            2 => Signal::RamAddrClear as u32,
                            3 => Signal::StackOut | Signal::RamLIn,
                            4 => Signal::RamIn | get_reg_out(reg_a),
                            5 => Signal::StackUp as u32,
                            _ => Signal::MicroOpsReset as u32,
                        }
                    }
                },
                Instruction::POP => {
                    let reg_a = Register::from_u32(reg_a).unwrap();
                    match micro_op {
                        2 => Signal::StackDown as u32,
                        3 => Signal::RamAddrClear as u32,
                        4 => Signal::StackOut | Signal::RamLIn,
                        5 => Signal::RamOut | get_reg_in(reg_a),
                        _ => Signal::MicroOpsReset as u32,
                    }
                },
                Instruction::LDA => {
                    match micro_op {
                        2 => Signal::HIn | Signal::ROMOut,
                        3 => Signal::PCUp as u32,
                        4 => Signal::LIn | Signal::ROMOut,
                        5 => Signal::PCUp as u32,
                        _ => Signal::MicroOpsReset as u32,
                    }
                },
                Instruction::JMP => {
                    if immediate {
                        match micro_op {
                            2 => Signal::ROMOut | Signal::PCHIn,
                            3 => Signal::PCUp as u32,
                            4 => Signal::ROMOut | Signal::PCLIn,
                            5 => Signal::PCUp as u32,
                            6 => Signal::PCApply as u32,
                            _ => Signal::MicroOpsReset as u32,
                        }
                    } else {
                        match micro_op {
                            2 => Signal::HOut | Signal::PCHIn,
                            3 => Signal::LOut | Signal::PCLIn,
                            4 => Signal::PCApply as u32,
                            _ => Signal::MicroOpsReset as u32,
                        }
                    }
                },
                Instruction::JZ => {
                    let reg_a = Register::from_u32(reg_a).unwrap();
                    let zero = match reg_a {
                        Register::A => a_zero == 1,
                        Register::B => b_zero == 1,
                        Register::H => h_zero == 1,
                        Register::L => l_zero == 1,
                    };
                    if zero {
                        if immediate {
                            match micro_op {
                                2 => Signal::ROMOut | Signal::PCHIn,
                                3 => Signal::PCUp as u32,
                                4 => Signal::ROMOut | Signal::PCLIn,
                                5 => Signal::PCUp as u32,
                                6 => Signal::PCApply as u32,
                                _ => Signal::MicroOpsReset as u32,                            
                            }
                        } else {
                            match micro_op {
                                2 => Signal::HOut | Signal::PCHIn,
                                3 => Signal::LOut | Signal::PCLIn,
                                4 => Signal::PCApply as u32,
                                _ => Signal::MicroOpsReset as u32,    
                            }
                        }
                    } else {
                        Signal::MicroOpsReset as u32
                    }
                },
                Instruction::JO => {
                    let overflow = overflow == 1;

                    if overflow {
                        if immediate {
                            match micro_op {
                                2 => Signal::ROMOut | Signal::RamHIn,
                                3 => Signal::PCUp as u32,
                                4 => Signal::ROMOut | Signal::RamLIn,
                                5 => Signal::PCUp as u32,
                                6 => Signal::PCApply as u32,
                                _ => Signal::MicroOpsReset as u32,                            
                            }
                        } else {
                            match micro_op {
                                2 => Signal::HOut | Signal::RamHIn,
                                3 => Signal::LOut | Signal::RamLIn,
                                4 => Signal::PCApply as u32,
                                _ => Signal::MicroOpsReset as u32,    
                            }
                        }
                    } else {
                        Signal::MicroOpsReset as u32
                    }
                },
                Instruction::ADD => {
                    let reg_a = Register::from_u32(reg_a).unwrap();
                    if immediate {
                        match micro_op {
                            2 => Signal::AluAIn | get_reg_out(reg_a),
                            3 => Signal::AluBIn | Signal::ROMOut,
                            4 => Signal::PCUp as u32,
                            5 => Signal::SumsOut | get_reg_in(reg_a),
                            _ => Signal::MicroOpsReset as u32,
                        }
                    } else {
                        let reg_b = Register::from_u32(reg_b).unwrap();
                        match micro_op {
                            2 => Signal::AluAIn | get_reg_out(reg_a),
                            3 => Signal::AluBIn | get_reg_out(reg_b),
                            4 => Signal::SumsOut | get_reg_in(reg_a),
                            _ => Signal::MicroOpsReset as u32,
                        }
                    }
                },
                Instruction::SUB => {
                    let reg_a = Register::from_u32(reg_a).unwrap();
                    if immediate {
                        match micro_op {
                            2 => Signal::AluAIn | get_reg_out(reg_a),
                            3 => Signal::AluBIn | Signal::ROMOut,
                            4 => Signal::PCUp as u32,
                            5 => Signal::Subtract | Signal::SumsOut | get_reg_in(reg_a),
                            _ => Signal::MicroOpsReset as u32,
                        }
                    } else {
                        let reg_b = Register::from_u32(reg_b).unwrap();
                        match micro_op {
                            2 => Signal::AluAIn | get_reg_out(reg_a),
                            3 => Signal::AluBIn | get_reg_out(reg_b),
                            5 => Signal::Subtract | Signal::SumsOut | get_reg_in(reg_a),
                            _ => Signal::MicroOpsReset as u32,
                        }
                    }
                },
                Instruction::TEL => Signal::MicroOpsReset as u32,
                Instruction::OUT => {
                    if immediate {
                        match micro_op {
                            2 => Signal::DisplayIn | Signal::ROMOut,
                            _ => Signal::MicroOpsReset as u32,
                        }
                    } else {
                        let reg_a = Register::from_u32(reg_a).unwrap();
                        match micro_op {
                            2 => Signal::DisplayIn | get_reg_out(reg_a),
                            _ => Signal::MicroOpsReset as u32,
                        }
                    }
                },
                Instruction::HLT => Signal::Halt as u32,
                Instruction::NOP => Signal::MicroOpsReset as u32,
            }
        }
    }
}

fn get_reg_out(reg: Register) -> Signal {
    match reg {
        Register::A => Signal::AOut,
        Register::B => Signal::BOut,
        Register::H => Signal::HOut,
        Register::L => Signal::LOut,
    }
}

fn get_reg_in(reg: Register) -> Signal {
    match reg {
        Register::A => Signal::AIn,
        Register::B => Signal::BIn,
        Register::H => Signal::HIn,
        Register::L => Signal::LIn,
    }
}

impl BitAnd<u32> for Input {
    type Output = u32;

    fn bitand(self, rhs: u32) -> Self::Output {
        self as u32 & rhs
    }
}

impl BitAnd<Input> for u32 {
    type Output = u32;

    fn bitand(self, rhs: Input) -> Self::Output {
        self & rhs as u32
    }
}

impl BitOr<Signal> for Signal {
    type Output = u32;

    fn bitor(self, rhs: Signal) -> Self::Output {
        self as u32 | rhs as u32
    }
}

impl BitOr<Signal> for u32 {
    type Output = u32;

    fn bitor(self, rhs: Signal) -> Self::Output {
        self | rhs as u32
    }
}

impl BitOr<u32> for Signal {
    type Output = u32;

    fn bitor(self, rhs: u32) -> Self::Output {
        self as u32 | rhs
    }
}
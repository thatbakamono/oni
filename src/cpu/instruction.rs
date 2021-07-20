use crate::cpu::{Flag, Register};
use byteorder::{BigEndian, ReadBytesExt};
use eyre::{eyre, Result};
use std::io::Cursor;

#[derive(Debug)]
pub enum MathOperation {
    Increment,
    Decrement,
}

#[derive(Debug)]
pub enum Instruction {
    NoOperation,
    Stop,
    Halt,
    Reset {
        location: u8,
    },
    LoadOneByteOfDataIntoRegister {
        data: u8,
        register: Register,
        treat_value_in_register_as_memory_address: bool,
    },
    LoadTwoBytesOfDataIntoRegister {
        data: u16,
        register: Register,
    },
    LoadValueOfFirstRegisterIntoSecondRegister {
        register1: Register,
        register2: Register,
        treat_value_in_first_register_as_memory_address: bool,
        treat_value_in_second_register_as_memory_address: bool,
        operation_on_first_register: Option<MathOperation>,
        operation_on_second_register: Option<MathOperation>,
    },
    IncrementValueInRegister {
        register: Register,
        treat_value_in_register_as_memory_address: bool,
    },
    DecrementValueInRegister {
        register: Register,
        treat_value_in_register_as_memory_address: bool,
    },
    AbsoluteJump {
        address: u16,
    },
    AbsoluteJumpIfFlagIsZero {
        flag: Flag,
        address: u16,
    },
    AbsoluteJumpIfFlagIsOne {
        flag: Flag,
        address: u16,
    },
    AbsoluteJumpToAddressInRegister {
        register: Register,
    },
    RelativeJump {
        steps: u8,
    },
    RelativeJumpIfFlagIsZero {
        flag: Flag,
        steps: u8,
    },
    RelativeJumpIfFlagIsOne {
        flag: Flag,
        steps: u8,
    },
    Return,
    ReturnIfFlagIsZero {
        flag: Flag,
    },
    ReturnIfFlagIsOne {
        flag: Flag,
    },
    ReturnAfterInterrupt,
    Call {
        address: u16,
    },
    CallIfFlagIsZero {
        flag: Flag,
        address: u16,
    },
    CallIfFlagIsOne {
        flag: Flag,
        address: u16,
    },
    RotateContentOfRegisterToLeft {
        register: Register,
    },
    RotateContentOfRegisterToLeftThroughCarryFlag {
        register: Register,
    },
    RotateContentOfRegisterToRight {
        register: Register,
    },
    RotateContentOfRegisterToRightThroughCarryFlag {
        register: Register,
    },
    Not {
        register: Register,
    },
    SetCarryFlag,
    NotCarryFlag,
    AdjustAccumulatorToBCDNumber,
    AddValueOfSecondRegisterToFirstRegister {
        register1: Register,
        register2: Register,
        treat_value_in_second_register_as_memory_address: bool,
    },
    AddOneByteToAccumulator {
        value: u8,
    },
    AddOneByteAndCarryFlagToAccumulator {
        value: u8,
    },
    SubtractValueOfSecondRegisterFromFirstRegister {
        register1: Register,
        register2: Register,
        treat_value_in_second_register_as_memory_address: bool,
    },
    SubtractOneByteFromAccumulator {
        value: u8,
    },
    SubtractOneByteAndCarryFlagFromAccumulator {
        value: u8,
    },
    LogicalAndOnAccumulatorAndRegister {
        register: Register,
        treat_value_in_register_as_memory_address: bool,
    },
    LogicalAndOnAccumulatorAndOneByte {
        value: u8,
    },
    LogicalOrOnAccumulatorAndRegister {
        register: Register,
        treat_value_in_register_as_memory_address: bool,
    },
    LogicalOrOnAccumulatorAndOneByte {
        value: u8,
    },
    LogicalXorOnAccumulatorAndRegister {
        register: Register,
        treat_value_in_register_as_memory_address: bool,
    },
    LogicalXorOnAccumulatorAndOneByte {
        value: u8,
    },
    CompareAccumulatorAndRegister {
        register: Register,
        treat_value_in_register_as_memory_address: bool,
    },
    CompareAccumulatorAndOneByte {
        value: u8,
    },
    PushValueOfRegisterOntoStack {
        register: Register,
    },
    PopValueFromStackIntoRegister {
        register: Register,
    },
    ResetInterruptMasterEnableFlag,
    SetInterruptMasterEnableFlag,
    StoreAccumulatorInMemory {
        address: u16,
    },
    LoadAccumulatorFromMemory {
        address: u16,
    },
    StoreAccumulatorInMemorySpecifiedByRegisterC,
    LoadAccumulatorFromMemorySpecifiedByRegisterC,
    StoreStackPointerInMemory {
        address: u16,
    },
    StoreContentOfRegisterHLInStackPointer,
    AddValueToStackPointer,
    AddValueToStackPointerAndStoreResultInRegisterHL,
}

impl Instruction {
    pub fn decode(memory: &mut Cursor<Vec<u8>>) -> Result<Instruction> {
        let opcode = memory.read_u8()?;

        match opcode {
            0x00 => Ok(Instruction::NoOperation),
            0x10 => {
                memory.read_u8()?;
                Ok(Instruction::Stop)
            }
            0x76 => Ok(Instruction::Halt),

            0xC7 | 0xD7 | 0xE7 | 0xF7 => Ok(Instruction::Reset {
                location: ((opcode >> 4) - 0xC) * 2,
            }),
            0xCF | 0xDF | 0xEF | 0xFF => Ok(Instruction::Reset {
                location: (((opcode >> 4) - 0xC) * 2) + 1,
            }),

            0xF3 => Ok(Instruction::ResetInterruptMasterEnableFlag),
            0xFB => Ok(Instruction::SetInterruptMasterEnableFlag),

            0x07 => Ok(Instruction::RotateContentOfRegisterToLeft {
                register: Register::A,
            }),
            0x17 => Ok(Instruction::RotateContentOfRegisterToLeftThroughCarryFlag {
                register: Register::A,
            }),
            0x0F => Ok(Instruction::RotateContentOfRegisterToRight {
                register: Register::A,
            }),
            0x1F => Ok(
                Instruction::RotateContentOfRegisterToRightThroughCarryFlag {
                    register: Register::A,
                },
            ),

            0x27 => Ok(Instruction::AdjustAccumulatorToBCDNumber),

            0x02 | 0x12 => Ok(Instruction::LoadValueOfFirstRegisterIntoSecondRegister {
                register1: Register::A,
                register2: match opcode & 0b11110000 {
                    0x0 => Register::BC,
                    0x1 => Register::DE,
                    _ => unreachable!(),
                },
                treat_value_in_first_register_as_memory_address: false,
                treat_value_in_second_register_as_memory_address: true,
                operation_on_first_register: None,
                operation_on_second_register: None,
            }),

            0x22 | 0x32 => Ok(Instruction::LoadValueOfFirstRegisterIntoSecondRegister {
                register1: Register::A,
                register2: Register::HL,
                treat_value_in_first_register_as_memory_address: false,
                treat_value_in_second_register_as_memory_address: true,
                operation_on_first_register: None,
                operation_on_second_register: match opcode & 0b11110000 {
                    0x2 => Some(MathOperation::Increment),
                    0x3 => Some(MathOperation::Decrement),
                    _ => unreachable!(),
                },
            }),

            0xC3 => Ok(Instruction::AbsoluteJump {
                address: memory.read_u16::<BigEndian>()?,
            }),

            0xC2 => Ok(Instruction::AbsoluteJumpIfFlagIsZero {
                flag: Flag::Z,
                address: memory.read_u16::<BigEndian>()?,
            }),
            0xD2 => Ok(Instruction::AbsoluteJumpIfFlagIsZero {
                flag: Flag::CY,
                address: memory.read_u16::<BigEndian>()?,
            }),

            0xCA => Ok(Instruction::AbsoluteJumpIfFlagIsOne {
                flag: Flag::Z,
                address: memory.read_u16::<BigEndian>()?,
            }),
            0xDA => Ok(Instruction::AbsoluteJumpIfFlagIsOne {
                flag: Flag::CY,
                address: memory.read_u16::<BigEndian>()?,
            }),

            0xE9 => Ok(Instruction::AbsoluteJumpToAddressInRegister {
                register: Register::HL,
            }),

            0x18 => Ok(Instruction::RelativeJump {
                steps: memory.read_u8()?,
            }),

            0x20 => Ok(Instruction::RelativeJumpIfFlagIsZero {
                flag: Flag::Z,
                steps: memory.read_u8()?,
            }),
            0x30 => Ok(Instruction::RelativeJumpIfFlagIsZero {
                flag: Flag::CY,
                steps: memory.read_u8()?,
            }),

            0x28 => Ok(Instruction::RelativeJumpIfFlagIsOne {
                flag: Flag::Z,
                steps: memory.read_u8()?,
            }),
            0x38 => Ok(Instruction::RelativeJumpIfFlagIsOne {
                flag: Flag::CY,
                steps: memory.read_u8()?,
            }),

            0xC9 => Ok(Instruction::Return),
            0xC0 => Ok(Instruction::ReturnIfFlagIsZero { flag: Flag::Z }),
            0xD0 => Ok(Instruction::ReturnIfFlagIsZero { flag: Flag::CY }),
            0xD9 => Ok(Instruction::ReturnAfterInterrupt),

            0xC8 => Ok(Instruction::ReturnIfFlagIsOne { flag: Flag::Z }),
            0xD8 => Ok(Instruction::ReturnIfFlagIsOne { flag: Flag::CY }),

            0xCD => Ok(Instruction::Call {
                address: memory.read_u16::<BigEndian>()?,
            }),

            0xC4 => Ok(Instruction::CallIfFlagIsZero {
                flag: Flag::Z,
                address: memory.read_u16::<BigEndian>()?,
            }),
            0xD4 => Ok(Instruction::CallIfFlagIsZero {
                flag: Flag::CY,
                address: memory.read_u16::<BigEndian>()?,
            }),

            0xCC => Ok(Instruction::CallIfFlagIsOne {
                flag: Flag::Z,
                address: memory.read_u16::<BigEndian>()?,
            }),
            0xDC => Ok(Instruction::CallIfFlagIsOne {
                flag: Flag::CY,
                address: memory.read_u16::<BigEndian>()?,
            }),

            0x2F => Ok(Instruction::Not {
                register: Register::A,
            }),

            0x37 => Ok(Instruction::SetCarryFlag),
            0x3F => Ok(Instruction::NotCarryFlag),

            0x03 | 0x13 | 0x23 | 0x33 => Ok(Instruction::IncrementValueInRegister {
                register: match opcode >> 4 {
                    0x0 => Register::BC,
                    0x1 => Register::DE,
                    0x2 => Register::HL,
                    0x3 => Register::SP,
                    _ => unreachable!(),
                },
                treat_value_in_register_as_memory_address: false,
            }),

            0x04 | 0x14 | 0x24 | 0x34 => Ok(Instruction::IncrementValueInRegister {
                register: match opcode >> 4 {
                    0x0 => Register::B,
                    0x1 => Register::D,
                    0x2 => Register::H,
                    0x3 => Register::HL,
                    _ => unreachable!(),
                },
                treat_value_in_register_as_memory_address: opcode == 0x34,
            }),

            0x0C | 0x1C | 0x2C | 0x3C => Ok(Instruction::IncrementValueInRegister {
                register: match opcode >> 4 {
                    0x0 => Register::C,
                    0x1 => Register::E,
                    0x2 => Register::L,
                    0x3 => Register::A,
                    _ => unreachable!(),
                },
                treat_value_in_register_as_memory_address: false,
            }),

            0x05 | 0x15 | 0x25 | 0x35 => Ok(Instruction::DecrementValueInRegister {
                register: match opcode >> 4 {
                    0x0 => Register::B,
                    0x1 => Register::D,
                    0x2 => Register::H,
                    0x3 => Register::HL,
                    _ => unreachable!(),
                },
                treat_value_in_register_as_memory_address: opcode == 0x35,
            }),

            0x0B | 0x1B | 0x2B | 0x3B => Ok(Instruction::DecrementValueInRegister {
                register: match opcode >> 4 {
                    0x0 => Register::BC,
                    0x1 => Register::DE,
                    0x2 => Register::HL,
                    0x3 => Register::SP,
                    _ => unreachable!(),
                },
                treat_value_in_register_as_memory_address: false,
            }),

            0x0D | 0x1D | 0x2D | 0x3D => Ok(Instruction::DecrementValueInRegister {
                register: match opcode >> 4 {
                    0x0 => Register::C,
                    0x1 => Register::E,
                    0x2 => Register::L,
                    0x3 => Register::A,
                    _ => unreachable!(),
                },
                treat_value_in_register_as_memory_address: false,
            }),

            0x06 | 0x16 | 0x26 | 0x36 => Ok(Instruction::LoadOneByteOfDataIntoRegister {
                data: memory.read_u8()?,
                register: match opcode >> 4 {
                    0x0 => Register::B,
                    0x1 => Register::D,
                    0x2 => Register::H,
                    0x3 => Register::HL,
                    _ => unreachable!(),
                },
                treat_value_in_register_as_memory_address: opcode == 0x36,
            }),

            0x0E | 0x1E | 0x2E | 0x3E => Ok(Instruction::LoadOneByteOfDataIntoRegister {
                data: memory.read_u8()?,
                register: match opcode >> 4 {
                    0x0 => Register::C,
                    0x1 => Register::E,
                    0x2 => Register::L,
                    0x3 => Register::A,
                    _ => unreachable!(),
                },
                treat_value_in_register_as_memory_address: false,
            }),

            0x01 | 0x11 | 0x21 | 0x31 => Ok(Instruction::LoadTwoBytesOfDataIntoRegister {
                data: memory.read_u16::<BigEndian>()?,
                register: match opcode >> 4 {
                    0x0 => Register::BC,
                    0x1 => Register::DE,
                    0x2 => Register::HL,
                    0x3 => Register::SP,
                    _ => unreachable!(),
                },
            }),

            0x40..=0x4F | 0x50..=0x5F | 0x60..=0x6F | 0x70..=0x75 | 0x77..=0x7F => {
                Ok(Instruction::LoadValueOfFirstRegisterIntoSecondRegister {
                    register1: match opcode & 0b00001111 {
                        0x0 | 0x8 => Register::B,
                        0x1 | 0x9 => Register::C,
                        0x2 | 0xA => Register::D,
                        0x3 | 0xB => Register::E,
                        0x4 | 0xC => Register::H,
                        0x5 | 0xD => Register::L,
                        0x6 | 0xE => Register::HL,
                        0x7 | 0xF => Register::A,
                        _ => unreachable!(),
                    },
                    register2: match opcode & 0b00001111 {
                        0x0..=0x7 => match opcode >> 4 {
                            0x4 => Register::B,
                            0x5 => Register::D,
                            0x6 => Register::H,
                            0x7 => Register::HL,
                            _ => unreachable!(),
                        },
                        0x8..=0xF => match opcode >> 4 {
                            0x4 => Register::C,
                            0x5 => Register::E,
                            0x6 => Register::L,
                            0x7 => Register::A,
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    },
                    treat_value_in_first_register_as_memory_address: opcode & 0b00001111 == 0x6
                        || opcode & 0b00001111 == 0xE,
                    treat_value_in_second_register_as_memory_address: opcode >> 4 == 0x7
                        && opcode & 0b00001111 < 0xE,
                    operation_on_first_register: None,
                    operation_on_second_register: None,
                })
            }

            0x0A | 0x1A | 0x2A | 0x3A => {
                Ok(Instruction::LoadValueOfFirstRegisterIntoSecondRegister {
                    register1: match opcode >> 4 {
                        0x0 => Register::BC,
                        0x1 => Register::DE,
                        0x2 | 0x3 => Register::HL,
                        _ => unreachable!(),
                    },
                    register2: Register::A,
                    treat_value_in_first_register_as_memory_address: true,
                    treat_value_in_second_register_as_memory_address: false,
                    operation_on_first_register: match opcode >> 4 {
                        0x0 | 0x1 => None,
                        0x2 => Some(MathOperation::Increment),
                        0x3 => Some(MathOperation::Decrement),
                        _ => unreachable!(),
                    },
                    operation_on_second_register: None,
                })
            }

            0x09 | 0x19 | 0x29 | 0x39 => Ok(Instruction::AddValueOfSecondRegisterToFirstRegister {
                register1: Register::HL,
                register2: match opcode >> 4 {
                    0x0 => Register::BC,
                    0x1 => Register::DE,
                    0x2 => Register::HL,
                    0x3 => Register::SP,
                    _ => unreachable!(),
                },
                treat_value_in_second_register_as_memory_address: false,
            }),

            0x80..=0x87 => Ok(Instruction::AddValueOfSecondRegisterToFirstRegister {
                register1: match opcode & 0b00001111 {
                    0x0 => Register::B,
                    0x1 => Register::C,
                    0x2 => Register::D,
                    0x3 => Register::E,
                    0x4 => Register::H,
                    0x5 => Register::L,
                    0x6 => Register::HL,
                    0x7 => Register::A,
                    _ => unreachable!(),
                },
                register2: Register::A,
                treat_value_in_second_register_as_memory_address: opcode == 0x86,
            }),

            0xC6 => Ok(Instruction::AddOneByteToAccumulator {
                value: memory.read_u8()?,
            }),

            0xCE => Ok(Instruction::AddOneByteAndCarryFlagToAccumulator {
                value: memory.read_u8()?,
            }),

            0x90..=0x97 => Ok(
                Instruction::SubtractValueOfSecondRegisterFromFirstRegister {
                    register1: match opcode & 0b00001111 {
                        0x0 => Register::B,
                        0x1 => Register::C,
                        0x2 => Register::D,
                        0x3 => Register::E,
                        0x4 => Register::H,
                        0x5 => Register::L,
                        0x6 => Register::HL,
                        0x7 => Register::A,
                        _ => unreachable!(),
                    },
                    register2: Register::A,
                    treat_value_in_second_register_as_memory_address: opcode == 0x96,
                },
            ),

            0xD6 => Ok(Instruction::SubtractOneByteFromAccumulator {
                value: memory.read_u8()?,
            }),

            0xDE => Ok(Instruction::SubtractOneByteAndCarryFlagFromAccumulator {
                value: memory.read_u8()?,
            }),

            0xA0..=0xA7 => Ok(Instruction::LogicalAndOnAccumulatorAndRegister {
                register: match opcode & 0b00001111 {
                    0x0 => Register::B,
                    0x1 => Register::C,
                    0x2 => Register::D,
                    0x3 => Register::E,
                    0x4 => Register::H,
                    0x5 => Register::L,
                    0x6 => Register::HL,
                    0x7 => Register::A,
                    _ => unreachable!(),
                },
                treat_value_in_register_as_memory_address: opcode == 0xA6,
            }),

            0xE6 => Ok(Instruction::LogicalAndOnAccumulatorAndOneByte {
                value: memory.read_u8()?,
            }),

            0xA8..=0xAF => Ok(Instruction::LogicalXorOnAccumulatorAndRegister {
                register: match opcode & 0b00001111 {
                    0x8 => Register::B,
                    0x9 => Register::C,
                    0xA => Register::D,
                    0xB => Register::E,
                    0xC => Register::H,
                    0xD => Register::L,
                    0xE => Register::HL,
                    0xF => Register::A,
                    _ => unreachable!(),
                },
                treat_value_in_register_as_memory_address: opcode == 0xAE,
            }),

            0xEE => Ok(Instruction::LogicalXorOnAccumulatorAndOneByte {
                value: memory.read_u8()?,
            }),

            0xB0..=0xB7 => Ok(Instruction::LogicalOrOnAccumulatorAndRegister {
                register: match opcode & 0b00001111 {
                    0x0 => Register::B,
                    0x1 => Register::C,
                    0x2 => Register::D,
                    0x3 => Register::E,
                    0x4 => Register::H,
                    0x5 => Register::L,
                    0x6 => Register::HL,
                    0x7 => Register::A,
                    _ => unreachable!(),
                },
                treat_value_in_register_as_memory_address: opcode == 0xB6,
            }),

            0xF6 => Ok(Instruction::LogicalOrOnAccumulatorAndOneByte {
                value: memory.read_u8()?,
            }),

            0xB8..=0xBF => Ok(Instruction::CompareAccumulatorAndRegister {
                register: match opcode & 0b00001111 {
                    0x8 => Register::B,
                    0x9 => Register::C,
                    0xA => Register::D,
                    0xB => Register::E,
                    0xC => Register::H,
                    0xD => Register::L,
                    0xE => Register::HL,
                    0xF => Register::A,
                    _ => unreachable!(),
                },
                treat_value_in_register_as_memory_address: opcode == 0xBE,
            }),

            0xFE => Ok(Instruction::CompareAccumulatorAndOneByte {
                value: memory.read_u8()?,
            }),

            0xC1 | 0xD1 | 0xE1 | 0xF1 => Ok(Instruction::PopValueFromStackIntoRegister {
                register: match opcode >> 4 {
                    0xC => Register::BC,
                    0xD => Register::DE,
                    0xE => Register::HL,
                    0xF => Register::AF,
                    _ => unreachable!(),
                },
            }),

            0xC5 | 0xD5 | 0xE5 | 0xF5 => Ok(Instruction::PushValueOfRegisterOntoStack {
                register: match opcode >> 4 {
                    0xC => Register::BC,
                    0xD => Register::DE,
                    0xE => Register::HL,
                    0xF => Register::AF,
                    _ => unreachable!(),
                },
            }),

            0xE0 => Ok(Instruction::StoreAccumulatorInMemory {
                address: (0xFF << 8) | (memory.read_u8()? as u16),
            }),
            0xEA => Ok(Instruction::StoreAccumulatorInMemory {
                address: memory.read_u16::<BigEndian>()?,
            }),

            0xF0 => Ok(Instruction::LoadAccumulatorFromMemory {
                address: (0xFF << 8) | (memory.read_u8()? as u16),
            }),
            0xFA => Ok(Instruction::LoadAccumulatorFromMemory {
                address: memory.read_u16::<BigEndian>()?,
            }),

            0xE2 => Ok(Instruction::StoreAccumulatorInMemorySpecifiedByRegisterC),

            0xF2 => Ok(Instruction::LoadAccumulatorFromMemorySpecifiedByRegisterC),

            0x08 => Ok(Instruction::StoreStackPointerInMemory {
                address: memory.read_u16::<BigEndian>()?,
            }),

            0xF9 => Ok(Instruction::StoreContentOfRegisterHLInStackPointer),

            0xE8 => Ok(Instruction::AddValueToStackPointer),

            0xF8 => Ok(Instruction::AddValueToStackPointerAndStoreResultInRegisterHL),

            0xCB => Err(eyre!("Unknown 16 bit opcode")), // 16 bit opcodes
            _ => Err(eyre!("Unknown 8 bit opcode")),
        }
    }
}

use crate::lc_3::registers::Registers;

use super::opcodes::Ops;

fn sign_extend(num: i32, len: u32) -> u32 {
    let tmp = (-num) as u32;
    let mask = (1 << (len + 1)) - 1;
    ((!tmp & mask) + 1) & mask
}
/// add constant value
pub fn add_const(dest: Registers, source: Registers, number: u8) -> u16 {
    ((Ops::Add as u16) << 12)
        | ((dest as u16) << 9)
        | ((source as u16) << 6)
        | (0b1 << 5)
        | (number as u16 & 0b11111)
}

/// add value in register
pub fn add_reg(dest: Registers, source1: Registers, source2: Registers) -> u16 {
    ((Ops::Add as u16) << 12) | ((dest as u16) << 9) | ((source1 as u16) << 6) | (source2 as u16)
}

/// bitwise and with constant value
pub fn and_const(dest: Registers, source: Registers, number: u8) -> u16 {
    ((Ops::And as u16) << 12)
        | ((dest as u16) << 9)
        | ((source as u16) << 6)
        | (0b1 << 5)
        | (number as u16 & 0b11111)
}

/// bitwise and with value in register
pub fn and_reg(dest: Registers, source1: Registers, source2: Registers) -> u16 {
    ((Ops::And as u16) << 12) | ((dest as u16) << 9) | ((source1 as u16) << 6) | (source2 as u16)
}

/// bitwise xor with constant value
pub fn xor_const(dest: Registers, source: Registers, number: u8) -> u16 {
    ((Ops::Xor as u16) << 12)
        | ((dest as u16) << 9)
        | ((source as u16) << 6)
        | (0b1 << 5)
        | (number as u16 & 0b11111)
}

/// bitwise xor with value in register
pub fn xor_reg(dest: Registers, source1: Registers, source2: Registers) -> u16 {
    ((Ops::Xor as u16) << 12) | ((dest as u16) << 9) | ((source1 as u16) << 6) | (source2 as u16)
}

/// bitwise not
pub fn not(dest: Registers, source: Registers) -> u16 {
    xor_const(dest, source, 0b11111)
}

/// jump to address stored in register
pub fn jmp(base: Registers) -> u16 {
    ((Ops::Jmp as u16) << 12) | ((base as u16) << 6)
}

/// return from subroutine
pub fn ret() -> u16 {
    jmp(Registers::R7)
}

/// branch (BR) instruction
/// branch `if (negative && cc < 0) || (zero && cc == 0) || (positive && cc > 0)`
pub fn branch(negative: bool, zero: bool, positive: bool, pc_offset: i16) -> u16 {
    let encoded_pc_offset = sign_extend(pc_offset as i32, 9) as u16;

    ((Ops::Br as u16) << 12)
        | ((negative as u16) << 11)
        | ((zero as u16) << 10)
        | ((positive as u16) << 9)
        | encoded_pc_offset
}

/// left shift
pub fn lshf(dest: Registers, source: Registers, amount: u8) -> u16 {
    ((Ops::Shf as u16) << 12) | ((dest as u16) << 9) | ((source as u16) << 6) | (amount as u16)
}

/// right shift logical
pub fn rshfl(dest: Registers, source: Registers, amount: u8) -> u16 {
    ((Ops::Shf as u16) << 12)
        | ((dest as u16) << 9)
        | ((source as u16) << 6)
        | (0b1 << 4)
        | (amount as u16)
}

/// right shift arithmetic
pub fn rshfa(dest: Registers, source: Registers, amount: u8) -> u16 {
    ((Ops::Shf as u16) << 12)
        | ((dest as u16) << 9)
        | ((source as u16) << 6)
        | (0b1 << 5)
        | (0b1 << 4)
        | (amount as u16)
}

pub fn lea(dest: Registers, amount: i16) -> u16 {
    let amount = sign_extend(amount as i32, 9) as u16;
    ((Ops::Lea as u16) << 12) | ((dest as u16) << 9) | amount & 0b111111111

}

pub fn return_from_interrupt() -> u16 {
    (Ops::Rti as u16) << 12
}

/// jump to subroutine, label
pub fn jsr(offset: i32) -> u16 {
    let offset = sign_extend(offset, 11) as u16;
    ((Ops::Jsr as u16) << 12) | (1 << 11) | offset
}

/// jump to subroutine, base reg
pub fn jsrr(base: Registers) -> u16 {
    ((Ops::Jsr as u16) << 12) | ((base as u16) << 6)
}

pub fn load_byte(dest: Registers, base: Registers, offset: i32) -> u16 {
    let offset = sign_extend(offset, 6) as u16;
    ((Ops::Ldb as u16) << 12) | ((dest as u16) << 9) | ((base as u16) << 6) | offset
}

pub fn load_word(dest: Registers, base: Registers, offset: i32) -> u16 {
    let offset = sign_extend(offset, 6) as u16;
    ((Ops::Ldw as u16) << 12) | ((dest as u16) << 9) | ((base as u16) << 6) | offset
}

pub fn store_byte(source: Registers, base: Registers, offset: i32) -> u16 {
    let offset = sign_extend(offset, 6) as u16;
    ((Ops::Stb as u16) << 12) | ((source as u16) << 9) | ((base as u16) << 6) | offset
}

pub fn store_word(source: Registers, base: Registers, offset: i32) -> u16 {
    let offset = sign_extend(offset, 6) as u16;
    ((Ops::Stw as u16) << 12) | ((source as u16) << 9) | ((base as u16) << 6) | offset
}

pub fn trap(vect: u8) -> u16 {
    ((Ops::Trap as u16) << 12) | (vect as u16)
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u8)]
pub enum Ops {
    Br = 0b0000,
    Add = 0b0001,
    And = 0b0101,
    Jmp = 0b1100,
    Xor = 0b1001,
    Shf = 0b1101,
    Lea = 0b1110,
    Rti = 0b1000,
    Jsr = 0b0100,
    Ldb = 0b0010,
    Ldw = 0b0110,
    Stb = 0b0011,
    Stw = 0b0111,
    Trap = 0b1111,
}

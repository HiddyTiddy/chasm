use crate::lc_3::ops::{jsr, jsrr, store_word};
use std::collections::HashMap;

use crate::lc_3::{
    ops::{
        add_const, add_reg, and_const, and_reg, branch, jmp, lea, load_byte, load_word, lshf, not,
        ret, return_from_interrupt, rshfa, rshfl, store_byte, trap, xor_const, xor_reg,
    },
    registers::Registers,
};

use super::tokenizer::{Token, Tokenizer};

fn eat_comment(tokenizer: &mut Tokenizer) {
    for next in tokenizer {
        if let Token::Linebreak = next {
            return;
        }
    }
}

macro_rules! comma {
    ($tokenizer:ident) => {
        let next = $tokenizer.next();
        if !matches!(next, Some(Token::Comma)) {
            return Err(());
        }
    };
}

macro_rules! register {
    ($tokenizer:ident) => {
        if let Some(Token::Register(register)) = $tokenizer.next() {
            register
        } else {
            return Err(());
        }
    };
}

fn negative_5bit(num: i32) -> u8 {
    let tmp = (-num) as u32;
    (((!tmp & 0b11111) + 1) & 0b11111).try_into().unwrap()
}

fn parse_add(tokenizer: &mut Tokenizer) -> Result<u16, ()> {
    let dest = register!(tokenizer);

    comma!(tokenizer);

    let source = register!(tokenizer);

    comma!(tokenizer);

    let operand = tokenizer.next();
    if let Some(Token::Number(num)) = operand {
        let num = negative_5bit(num);
        Ok(add_const(dest, source, num))
    } else if let Some(Token::Register(reg)) = operand {
        Ok(add_reg(dest, source, reg))
    } else {
        Err(())
    }
}

fn parse_and(tokenizer: &mut Tokenizer) -> Result<u16, ()> {
    let dest = register!(tokenizer);

    comma!(tokenizer);

    let source = register!(tokenizer);

    comma!(tokenizer);

    let operand = tokenizer.next();
    if let Some(Token::Number(num)) = operand {
        Ok(and_const(dest, source, negative_5bit(num)))
    } else if let Some(Token::Register(reg)) = operand {
        Ok(and_reg(dest, source, reg))
    } else {
        Err(())
    }
}

fn parse_xor(tokenizer: &mut Tokenizer) -> Result<u16, ()> {
    let dest = register!(tokenizer);

    comma!(tokenizer);

    let source = register!(tokenizer);

    comma!(tokenizer);

    let operand = tokenizer.next();
    if let Some(Token::Number(num)) = operand {
        Ok(xor_const(dest, source, negative_5bit(num)))
    } else if let Some(Token::Register(reg)) = operand {
        Ok(xor_reg(dest, source, reg))
    } else {
        Err(())
    }
}

fn parse_not(tokenizer: &mut Tokenizer) -> Result<u16, ()> {
    let dest = register!(tokenizer);

    comma!(tokenizer);

    let source = register!(tokenizer);

    Ok(not(dest, source))
}

fn parse_jmp(tokenizer: &mut Tokenizer) -> Result<u16, ()> {
    if let Some(Token::Register(base_register)) = tokenizer.next() {
        Ok(jmp(base_register))
    } else {
        Err(())
    }
}

fn parse_br(tokenizer: &mut Tokenizer) -> Result<String, ()> {
    if let Some(Token::Word(label)) = tokenizer.next() {
        Ok(label)
    } else {
        Err(())
    }
}

fn parse_lea(tokenizer: &mut Tokenizer) -> Result<(Registers, String), ()> {
    let reg = register!(tokenizer);

    comma!(tokenizer);

    if let Some(Token::Word(label)) = tokenizer.next() {
        Ok((reg, label))
    } else {
        Err(())
    }
}

fn parse_shift(tokenizer: &mut Tokenizer) -> Result<(Registers, Registers, u8), ()> {
    let dest = register!(tokenizer);

    comma!(tokenizer);

    let source = register!(tokenizer);

    comma!(tokenizer);

    let amount = if let Some(Token::Number(amount)) = tokenizer.next() {
        amount
    } else {
        return Err(());
    };

    Ok((dest, source, (amount & 0b1111) as u8))
}

fn parse_trap(tokenizer: &mut Tokenizer) -> Result<u16, ()> {
    if let Some(Token::Number(vect)) = tokenizer.next() {
        Ok(trap(vect.try_into().unwrap()))
    } else {
        Err(())
    }
}

fn parse_stb(tokenizer: &mut Tokenizer) -> Result<u16, ()> {
    let source = register!(tokenizer);

    comma!(tokenizer);

    let base = register!(tokenizer);

    comma!(tokenizer);

    let offset = if let Some(Token::Number(offset)) = tokenizer.next() {
        offset
    } else {
        return Err(());
    };

    Ok(store_byte(source, base, offset))
}

fn parse_stw(tokenizer: &mut Tokenizer) -> Result<u16, ()> {
    let source = register!(tokenizer);

    comma!(tokenizer);

    let base = register!(tokenizer);

    comma!(tokenizer);

    let offset = if let Some(Token::Number(offset)) = tokenizer.next() {
        offset
    } else {
        return Err(());
    };

    Ok(store_word(source, base, offset))
}

fn parse_ldb(tokenizer: &mut Tokenizer) -> Result<u16, ()> {
    let dest = register!(tokenizer);

    comma!(tokenizer);

    let base = register!(tokenizer);

    comma!(tokenizer);

    let offset = if let Some(Token::Number(offset)) = tokenizer.next() {
        offset
    } else {
        return Err(());
    };

    Ok(load_byte(dest, base, offset))
}

fn parse_ldw(tokenizer: &mut Tokenizer) -> Result<u16, ()> {
    let dest = register!(tokenizer);

    comma!(tokenizer);

    let base = register!(tokenizer);

    comma!(tokenizer);

    let offset = if let Some(Token::Number(offset)) = tokenizer.next() {
        offset
    } else {
        return Err(());
    };

    Ok(load_word(dest, base, offset))
}

fn parse_jsrr(tokenizer: &mut Tokenizer) -> Result<u16, ()> {
    if let Some(Token::Register(base)) = tokenizer.next() {
        Ok(jsrr(base))
    } else {
        Err(())
    }
}

fn parse_jsr(tokenizer: &mut Tokenizer) -> Result<String, ()> {
    if let Some(Token::Word(label)) = tokenizer.next() {
        Ok(label)
    } else {
        Err(())
    }
}

fn parse_define_bytes(tokenizer: &mut Tokenizer) -> Result<Vec<u16>, ()> {
    let mut bytes: Vec<u8> = vec![];

    while let Some(next) = tokenizer.next() {
        if let Token::Number(num) = next {
            if (0..0x100).contains(&num) {
                bytes.push(num.try_into().unwrap())
            } else {
                // todo report
                return Err(());
            }
        } else if let Token::Str(string) = next {
            for ch in string.bytes() {
                bytes.push(ch);
            }
        } else {
            return Err(());
        }

        if let Some(next) = tokenizer.next() {
            match next {
                Token::Comma => {}
                Token::Linebreak => break,
                _ => return Err(()),
            }
        }
    }

    let mut bytes = bytes.iter();
    let mut words = vec![];
    while let Some(&lower) = bytes.next() {
        let &upper = bytes.next().unwrap_or(&0);
        words.push((upper as u16) << 8 | (lower as u16));
    }
    Ok(words)
}
fn parse_define_words(tokenizer: &mut Tokenizer) -> Result<Vec<u16>, ()> {
    let mut words: Vec<u16> = vec![];
    while let Some(next) = tokenizer.next() {
        if let Token::Number(num) = next {
            if (0..0x10000).contains(&num) {
                words.push(num.try_into().unwrap())
            } else {
                // todo report
                return Err(());
            }
        } else {
            return Err(());
        }

        if let Some(next) = tokenizer.next() {
            match next {
                Token::Comma => {}
                Token::Linebreak => break,
                _ => return Err(()),
            }
        }
    }

    Ok(words)
}

fn parse_set_loc(tokenizer: &mut Tokenizer) -> Result<i32, ()> {
    if !matches!(tokenizer.next(), Some(Token::Equals)) {
        return Err(());
    }
    if let Some(Token::Number(addr)) = tokenizer.next() {
        // shift bc u16
        Ok(addr >> 1)
    } else {
        Err(())
    }
}

macro_rules! parse {
    ($func_name:ident, $display_name:expr, $line_number:expr, $tokenizer:ident, $instructions:ident, $address:ident) => {{
        if let Ok(instr) = $func_name(&mut $tokenizer) {
            $instructions.push(instr);
            $address += 1;
        } else {
            return Err(ParseError::StatementSyntaxError(
                $display_name.to_owned(),
                $line_number,
            ));
        }
    }};
}

#[derive(Debug)]
struct Branch {
    current_addr: i32,
    n: bool,
    z: bool,
    p: bool,
    index: usize,
    label: String,
    line_number: u32,
}

#[derive(Debug)]
struct LoadEffectiveAddress {
    current_addr: i32,
    index: usize,
    dest: Registers,
    label: String,
    line_number: u32,
}

#[derive(Debug)]
struct JumpSubroutine {
    current_addr: i32,
    index: usize,
    label: String,
    line_number: u32,
}

const PLACEHOLDER: u16 = 0xaaaa;

#[derive(Debug)]
enum AddressResolving {
    Branch(Branch),
    Lea(LoadEffectiveAddress),
    Jsr(JumpSubroutine),
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    StatementSyntaxError(String, u32),
    LabelSyntaxError(String, u32),
    UnexpectedToken(String, u32),
    InvalidLocation(u32),
}

#[derive(Debug, PartialEq)]
pub enum LinkError {
    LabelNotResolvedError(String, u32),
}

#[derive(Debug)]
pub struct TranslationOutput {
    instructions: Vec<u16>,
    labels: HashMap<String, i32>,
    to_resolve: Vec<AddressResolving>,
    last_address: i32,
}

impl TranslationOutput {
    pub fn extend(&mut self, other: TranslationOutput) {
        let offset_index = self.instructions.len();
        // TODO make this more efficient maybe
        // if it turns out to be slow
        let mut labels = HashMap::new();
        for (key, &value) in &other.labels {
            labels.insert(key.to_owned(), value + self.last_address);
        }
        self.labels.extend(labels);

        let mut to_resolve = Vec::with_capacity(other.to_resolve.len());
        for res in other.to_resolve {
            to_resolve.push(match res {
                AddressResolving::Branch(mut branch) => {
                    branch.current_addr += self.last_address;
                    branch.index += offset_index;
                    AddressResolving::Branch(branch)
                }
                AddressResolving::Lea(mut lea) => {
                    lea.current_addr += self.last_address;
                    lea.index += offset_index;
                    AddressResolving::Lea(lea)
                }
                AddressResolving::Jsr(mut jsr) => {
                    jsr.current_addr += self.last_address;
                    jsr.index += offset_index;
                    AddressResolving::Jsr(jsr)
                }
            });
        }

        self.instructions.extend(other.instructions);
        self.to_resolve.extend(to_resolve);
        self.last_address += other.last_address;
    }
}

pub fn translate(text: &str) -> Result<TranslationOutput, ParseError> {
    let mut tokenizer = Tokenizer::new(text);
    let mut instructions = vec![];
    let mut line_number = 1;
    let mut current_addr = 0;
    let mut labels = HashMap::new();
    let mut branches: Vec<AddressResolving> = vec![];
    while let Some(next) = tokenizer.next() {
        match next {
            Token::Add => parse!(
                parse_add,
                "ADD",
                line_number,
                tokenizer,
                instructions,
                current_addr
            ),
            Token::And => parse!(
                parse_and,
                "AND",
                line_number,
                tokenizer,
                instructions,
                current_addr
            ),
            Token::Xor => parse!(
                parse_xor,
                "XOR",
                line_number,
                tokenizer,
                instructions,
                current_addr
            ),
            Token::Not => parse!(
                parse_not,
                "NOT",
                line_number,
                tokenizer,
                instructions,
                current_addr
            ),
            Token::Jmp => parse!(
                parse_jmp,
                "JMP",
                line_number,
                tokenizer,
                instructions,
                current_addr
            ),
            Token::Ret => {
                let parse_ret = |_tokenizer: &mut Tokenizer| -> Result<u16, ()> { Ok(ret()) };
                parse!(
                    parse_ret,
                    "RET",
                    line_number,
                    tokenizer,
                    instructions,
                    current_addr
                )
            }
            Token::Br(n, z, p) => {
                if let Ok(label) = parse_br(&mut tokenizer) {
                    branches.push(AddressResolving::Branch(Branch {
                        current_addr,
                        n,
                        z,
                        p,
                        index: instructions.len(),
                        label,
                        line_number,
                    }));
                    current_addr += 1;
                    // placeholder value
                    instructions.push(PLACEHOLDER);
                } else {
                    return Err(ParseError::StatementSyntaxError(
                        format!(
                            "BR{}{}{}",
                            if n { "n" } else { "" },
                            if z { "z" } else { "" },
                            if p { "p" } else { "" }
                        ),
                        line_number,
                    ));
                }
            }
            Token::Lea => {
                if let Ok((dest, label)) = parse_lea(&mut tokenizer) {
                    branches.push(AddressResolving::Lea(LoadEffectiveAddress {
                        current_addr,
                        index: instructions.len(),
                        label,
                        dest,
                        line_number,
                    }));
                    current_addr += 1;
                    instructions.push(PLACEHOLDER);
                } else {
                    return Err(ParseError::StatementSyntaxError(
                        "LEA".to_owned(),
                        line_number,
                    ));
                }
            }
            Token::Jsr => {
                if let Ok(label) = parse_jsr(&mut tokenizer) {
                    branches.push(AddressResolving::Jsr(JumpSubroutine {
                        current_addr,
                        label,
                        index: instructions.len(),
                        line_number,
                    }));
                    current_addr += 1;
                    instructions.push(PLACEHOLDER);
                } else {
                    return Err(ParseError::StatementSyntaxError(
                        "JSR".to_owned(),
                        line_number,
                    ));
                }
            }
            Token::Lshf => {
                if let Ok((dest, source, amount)) = parse_shift(&mut tokenizer) {
                    instructions.push(lshf(dest, source, amount));
                    current_addr += 1;
                } else {
                    return Err(ParseError::StatementSyntaxError(
                        "LSHF".to_owned(),
                        line_number,
                    ));
                }
            }
            Token::Rshfl => {
                if let Ok((dest, source, amount)) = parse_shift(&mut tokenizer) {
                    instructions.push(rshfl(dest, source, amount));
                    current_addr += 1;
                } else {
                    return Err(ParseError::StatementSyntaxError(
                        "RSHFL".to_owned(),
                        line_number,
                    ));
                }
            }
            Token::Rshfa => {
                if let Ok((dest, source, amount)) = parse_shift(&mut tokenizer) {
                    instructions.push(rshfa(dest, source, amount));
                    current_addr += 1;
                } else {
                    return Err(ParseError::StatementSyntaxError(
                        "RSHFA".to_owned(),
                        line_number,
                    ));
                }
            }
            Token::Rti => {
                instructions.push(return_from_interrupt());
                current_addr += 1;
            }
            Token::Trap => {
                parse!(
                    parse_trap,
                    "TRAP",
                    line_number,
                    tokenizer,
                    instructions,
                    current_addr
                )
            }
            Token::Halt => {
                let parse_halt = |_tokenizer: &mut Tokenizer| -> Result<u16, ()> { Ok(trap(0x25)) };
                parse!(
                    parse_halt,
                    "HALT",
                    line_number,
                    tokenizer,
                    instructions,
                    current_addr
                )
            }
            Token::Getc => {
                let parse_getc = |_tokenizer: &mut Tokenizer| -> Result<u16, ()> { Ok(trap(0x20)) };
                parse!(
                    parse_getc,
                    "GETC",
                    line_number,
                    tokenizer,
                    instructions,
                    current_addr
                )
            }
            Token::Out => {
                let parse_out = |_tokenizer: &mut Tokenizer| -> Result<u16, ()> { Ok(trap(0x21)) };
                parse!(
                    parse_out,
                    "OUT",
                    line_number,
                    tokenizer,
                    instructions,
                    current_addr
                )
            }
            Token::Puts => {
                let parse_puts = |_tokenizer: &mut Tokenizer| -> Result<u16, ()> { Ok(trap(0x22)) };
                parse!(
                    parse_puts,
                    "PUTS",
                    line_number,
                    tokenizer,
                    instructions,
                    current_addr
                )
            }
            Token::In => {
                let parse_in = |_tokenizer: &mut Tokenizer| -> Result<u16, ()> { Ok(trap(0x23)) };
                parse!(
                    parse_in,
                    "IN",
                    line_number,
                    tokenizer,
                    instructions,
                    current_addr
                )
            }
            Token::Stb => {
                parse!(
                    parse_stb,
                    "STB",
                    line_number,
                    tokenizer,
                    instructions,
                    current_addr
                )
            }
            Token::Stw => {
                parse!(
                    parse_stw,
                    "STW",
                    line_number,
                    tokenizer,
                    instructions,
                    current_addr
                )
            }
            Token::Ldb => {
                parse!(
                    parse_ldb,
                    "LDB",
                    line_number,
                    tokenizer,
                    instructions,
                    current_addr
                )
            }
            Token::Ldw => {
                parse!(
                    parse_ldw,
                    "LDW",
                    line_number,
                    tokenizer,
                    instructions,
                    current_addr
                )
            }
            Token::Jsrr => {
                parse!(
                    parse_jsrr,
                    "JSRR",
                    line_number,
                    tokenizer,
                    instructions,
                    current_addr
                )
            }

            // static memory
            Token::DefineBytes => match parse_define_bytes(&mut tokenizer) {
                Ok(words) => {
                    for word in words {
                        instructions.push(word);
                        current_addr += 1;
                    }
                }
                Err(()) => {
                    return Err(ParseError::StatementSyntaxError(
                        "DB".to_owned(),
                        line_number,
                    ))
                }
            },
            Token::DefineWords => match parse_define_words(&mut tokenizer) {
                Ok(words) => {
                    for word in words {
                        instructions.push(word);
                        current_addr += 1;
                    }
                }
                Err(()) => {
                    return Err(ParseError::StatementSyntaxError(
                        "DW".to_owned(),
                        line_number,
                    ))
                }
            },

            Token::Period => {
                if let Ok(skip_to) = parse_set_loc(&mut tokenizer) {
                    if skip_to < current_addr {
                        return Err(ParseError::InvalidLocation(line_number));
                    }
                    assert!(
                        skip_to >= current_addr,
                        ". = : skip should be after current address; TODO implement error"
                    );
                    instructions.extend(vec![0x0; (skip_to - current_addr) as usize]);
                    current_addr = skip_to;
                } else {
                    return Err(ParseError::StatementSyntaxError(
                        ".".to_owned(),
                        line_number,
                    ));
                }
            }

            Token::Semicolon => eat_comment(&mut tokenizer),
            Token::Linebreak => {
                line_number += 1;
            }
            Token::Word(label) => {
                if let Some(Token::Colon) = tokenizer.next() {
                    labels.insert(label, current_addr);
                } else {
                    return Err(ParseError::LabelSyntaxError(label, line_number));
                }
            }
            Token::Comma
            | Token::Equals
            | Token::Number(_)
            | Token::Register(_)
            | Token::Colon
            | Token::Str(_) => {
                return Err(ParseError::UnexpectedToken(
                    format!("{next:?}"),
                    line_number,
                ));
            }
        }
    }

    // println!("{labels:?}");

    Ok(TranslationOutput {
        labels,
        instructions,
        to_resolve: branches,
        last_address: current_addr,
    })
}

pub fn link(mut translation: TranslationOutput) -> Result<Vec<u16>, LinkError> {
    // resolve branches
    for load in translation.to_resolve {
        match load {
            AddressResolving::Branch(br) => {
                if let Some(label_loc) = translation.labels.get(&br.label) {
                    let offset: i32 = label_loc - br.current_addr - 1;
                    let offset: i16 = offset.try_into().unwrap();
                    translation.instructions[br.index] = branch(br.n, br.z, br.p, offset);
                } else {
                    return Err(LinkError::LabelNotResolvedError(
                        br.label.to_owned(),
                        br.line_number,
                    ));
                }
            }
            AddressResolving::Lea(load_effective_address) => {
                if let Some(label_loc) = translation.labels.get(&load_effective_address.label) {
                    let offset: i32 = label_loc - load_effective_address.current_addr - 1;
                    let offset: i16 = offset.try_into().unwrap();
                    translation.instructions[load_effective_address.index] =
                        lea(load_effective_address.dest, offset);
                } else {
                    return Err(LinkError::LabelNotResolvedError(
                        load_effective_address.label.to_owned(),
                        load_effective_address.line_number,
                    ));
                }
            }
            AddressResolving::Jsr(jump_subroutine) => {
                if let Some(label_loc) = translation.labels.get(&jump_subroutine.label) {
                    let offset: i32 = label_loc - jump_subroutine.current_addr - 1;
                    translation.instructions[jump_subroutine.index] = jsr(offset);
                } else {
                    return Err(LinkError::LabelNotResolvedError(
                        jump_subroutine.label.to_owned(),
                        jump_subroutine.line_number,
                    ));
                }
            }
        }
    }

    Ok(translation.instructions)
}

#[cfg(test)]
mod tests {

    use super::{link, translate};

    #[allow(clippy::unusual_byte_groupings)]
    #[test]
    fn should_parse_basic() {
        {
            let text = "ADD R0, R2, #12";
            let translation = translate(text);
            assert!(translation.is_ok());
            let translation = translation.unwrap();
            let instructions = link(translation);
            assert_eq!(instructions, Ok(vec![0x10ac]));
        }
        {
            let text = "AND R0, R2, #12";
            let translation = translate(text);
            assert!(translation.is_ok());
            let translation = translation.unwrap();
            let instructions = link(translation);
            assert_eq!(instructions, Ok(vec![0b0101_000_010_1_01100]));
        }
        {
            let text = "JMP R5";
            let translation = translate(text);
            assert!(translation.is_ok());
            let translation = translation.unwrap();
            let instructions = link(translation);
            assert_eq!(instructions, Ok(vec![0b1100_000_101_000000]));
        }
        {
            let text = "LDB R1, R4, xa\nLDW R3, R6, #5";
            let translation = translate(text);
            assert!(translation.is_ok());
            let translation = translation.unwrap();
            let instructions = link(translation);
            assert_eq!(
                instructions,
                Ok(vec![0b0010_001_100_001010, 0b0110_011_110_000101,])
            );
        }
    }

    #[allow(clippy::unusual_byte_groupings)]
    #[test]
    fn should_parse_labels() {
        let text = "LABEL:\nXOR R0, R3, R4\nBRnp LABEL";
        let translation = translate(text).expect("should parse input");
        let instructions = link(translation);
        assert_eq!(
            instructions,
            Ok(vec![0b1001_000_011_0_00_100, 0b0000_1_0_1_111111110])
        );
    }

    #[allow(clippy::unusual_byte_groupings)]
    #[test]
    fn should_reparse() {
        let text = "test:\nXOR R0, R3, R4";
        let mut translation = translate(text).expect("should parse input");
        let text = "BRnp test";
        let second_translation = translate(text).expect("should parse input");

        translation.extend(second_translation);

        let instructions = link(translation);
        assert_eq!(
            instructions,
            Ok(vec![0b1001_000_011_0_00_100, 0b0000_1_0_1_111111110])
        );
    }

    #[test]
    fn should_set_loc() {
        let text = ". = x10";
        let translation = translate(text).expect("should parse valid input");
        let instructions = link(translation);
        assert_eq!(instructions, Ok(vec![0x0000; 0x10 >> 1]));

        let text = "DB \"hi\"\n. = #10";
        let translation = translate(text).expect("should parse valid input");
        let instructions = link(translation);
        assert_eq!(
            instructions,
            Ok(vec![0x6968, 0x0000, 0x0000, 0x0000, 0x0000])
        );
    }
}

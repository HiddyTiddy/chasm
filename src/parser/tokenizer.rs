use crate::lc_3::registers::Registers;
use std::str::FromStr;

use super::scanner::Scanner;

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Add,
    And,
    Xor,
    Not,
    Jmp,
    Ret,
    Lea,
    /// BR n z p
    Br(bool, bool, bool),
    Lshf,
    Rshfl,
    Rshfa,
    Rti,
    Ldb,
    Ldw,
    Stb,
    Stw,
    Jsr,
    Jsrr,
    // static memory layout
    DefineBytes,
    DefineWords,
    Trap,
    // traps
    Halt,
    Getc,
    Out,
    Puts,
    In,
    Register(Registers),
    Number(i32),
    Comma,
    Colon,
    Linebreak,
    Semicolon,
    Period,
    Equals,
    Word(String),
    Str(String),
}

pub struct Tokenizer<'a> {
    scanner: Scanner<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            scanner: Scanner::new(text),
        }
    }
}

/// returns whether `int_str` is an integer and if so, sets value
fn is_int(int_str: &str, value: &mut Option<i32>) -> bool {
    let radix = if int_str.starts_with('#') {
        10
    } else if int_str.starts_with('x') {
        16
    } else if int_str.starts_with('b') {
        2
    } else {
        return false;
    };

    let without_prefix = &int_str[1..];
    if let Ok(val) = i32::from_str_radix(without_prefix, radix) {
        *value = Some(val);
        true
    } else {
        false
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;
        if let Some(next) = self.scanner.next() {
            let mut value = None;
            let mut str_value = None;
            let next: String = next.iter().map(|&ch| ch as char).collect::<String>();
            match next.as_str() {
                "ADD" => Some(Add),
                "AND" => Some(And),
                "XOR" => Some(Xor),
                "NOT" => Some(Not),
                "JMP" => Some(Jmp),
                "RET" => Some(Ret),
                "BR" => Some(Br(true, true, true)),
                "BRn" => Some(Br(true, false, false)),
                "BRnp" => Some(Br(true, false, true)),
                "BRnz" => Some(Br(true, true, false)),
                "BRzp" => Some(Br(false, true, true)),
                "BRnzp" => Some(Br(true, true, true)),
                "BRz" => Some(Br(false, true, false)),
                "BRp" => Some(Br(false, false, true)),
                "LSHF" => Some(Lshf),
                "RSHFL" => Some(Rshfl),
                "RSHFA" => Some(Rshfa),
                "LEA" => Some(Lea),
                "RTI" => Some(Rti),
                // trap
                "TRAP" => Some(Trap),
                "HALT" => Some(Halt),
                "GETC" => Some(Getc),
                "OUT" => Some(Out),
                "PUTS" => Some(Puts),
                "IN" => Some(In),
                "STB" => Some(Stb),
                "STW" => Some(Stw),
                "LDB" => Some(Ldb),
                "LDW" => Some(Ldw),
                "JSR" => Some(Jsr),
                "JSRR" => Some(Jsrr),
                "DB" => Some(DefineBytes),
                "DW" => Some(DefineWords),
                "," => Some(Comma),
                "." => Some(Period),
                "=" => Some(Equals),
                "R0" | "R1" | "R2" | "R3" | "R4" | "R5" | "R6" | "R7" => {
                    Some(Register(Registers::from_str(next.as_str()).unwrap()))
                }
                "\n" => Some(Linebreak),
                ";" => Some(Semicolon),
                ":" => Some(Colon),
                x if is_int(x, &mut value) => Some(Number(value.unwrap())),
                x if is_str(x, &mut str_value) => Some(Str(str_value.unwrap())),
                x => Some(Word(x.to_owned())),
            }
        } else {
            None
        }
    }
}

fn is_str(maybe_str: &str, str_value: &mut Option<String>) -> bool {
    if maybe_str.starts_with('"') && maybe_str.ends_with('"') {
        *str_value = Some(String::from(&maybe_str[1..maybe_str.len() - 1]));
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{lc_3::registers::Registers, parser::tokenizer::Token};

    use super::Tokenizer;

    #[test]
    fn test_1() {
        let text = "ADD R1, #10\n";
        let mut tokenizer = Tokenizer::new(text);
        assert_eq!(tokenizer.next(), Some(Token::Add));
        assert_eq!(tokenizer.next(), Some(Token::Register(Registers::R1)));
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(tokenizer.next(), Some(Token::Number(10)));
        assert_eq!(tokenizer.next(), Some(Token::Linebreak));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_2() {
        use super::Token::*;
        let text = "ADD\nADD R1,R0\n; Comment b1010\nADD R0, x-a";
        let mut tokenizer = Tokenizer::new(text);
        assert_eq!(tokenizer.next(), Some(Add));
        assert_eq!(tokenizer.next(), Some(Linebreak));
        assert_eq!(tokenizer.next(), Some(Add));
        assert_eq!(tokenizer.next(), Some(Register(Registers::R1)));
        assert_eq!(tokenizer.next(), Some(Comma));
        assert_eq!(tokenizer.next(), Some(Register(Registers::R0)));
        assert_eq!(tokenizer.next(), Some(Linebreak));
        assert_eq!(tokenizer.next(), Some(Semicolon));
        assert_eq!(tokenizer.next(), Some(Word("Comment".to_owned())));
        assert_eq!(tokenizer.next(), Some(Number(0b1010)));
        assert_eq!(tokenizer.next(), Some(Linebreak));
        assert_eq!(tokenizer.next(), Some(Add));
        assert_eq!(tokenizer.next(), Some(Register(Registers::R0)));
        assert_eq!(tokenizer.next(), Some(Comma));
        assert_eq!(tokenizer.next(), Some(Number(-10)));
        assert_eq!(tokenizer.next(), None);
    }
}

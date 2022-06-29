//! # Chasm C interface
//!
//! with the following interface, it is possible to use chasm in a non-rust way.
//!
//! ## Example
//!
//! chasm can be used from a C++ context like this:
//!
//! ```cpp
//! int32_t code;
//! uintptr_t len;
//! std::string assembly_text = "AND R0, R0, #0\nADD R0, R0, xa";
//! // generate intermediate representation
//! const chasm::TranslationOutput *intermediate = chasm::parse_asm(text.c_str(), &code);
//!
//! if (code != 0) return handle_error(code);
//!
//! const uint16_t *instructions = chasm::link_asm(intermediate, &err, &len);
//! if (code != 0) return handle_error(code);
//!
//! ```

use crate::parser::translator::{link, translate, LinkError, ParseError, TranslationOutput};
use libc::c_char;
use std::ffi::CStr;

/// takes an assembly-c_str and parses it.
/// if parsing fails, `parse_asm` will set `*err` and return `nullptr`
/// else `parse_asm` will return a pointer to an intermediate representation of the assembled instructions
///
/// if any of the pointers supplied are invalid or nullptr, the function may crash
///
/// ## `*err`
/// err can be
/// - `-1`: Statement Syntax Error
/// - `-2`: Label Syntax Error
/// - `-3`: Unexpected Token
/// - `-4`: Label Not Resolved Error
/// - `-5`: Cannot set location
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn parse_asm(assembly: *const c_char, err: *mut i32) -> *const TranslationOutput {
    let assembly = unsafe {
        assert!(!assembly.is_null());
        CStr::from_ptr(assembly)
    };

    let assembly = assembly.to_str().unwrap();
    match translate(assembly) {
        Ok(translation) => Box::into_raw(Box::new(translation)) as *const TranslationOutput,
        Err(error) => {
            match error {
                ParseError::StatementSyntaxError(_, _) => unsafe { *err = -1 },
                ParseError::LabelSyntaxError(_, _) => unsafe { *err = -2 },
                ParseError::UnexpectedToken(_, _) => unsafe { *err = -3 },
                ParseError::InvalidLocation(_) => unsafe { *err = -5 },
            }

            std::ptr::null::<TranslationOutput>()
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn parse_asm_extend(
    assembly: *const c_char,
    previous: *const TranslationOutput,
    err: *mut i32,
) -> *const TranslationOutput {
    let assembly = unsafe {
        assert!(!assembly.is_null());
        CStr::from_ptr(assembly)
    };
    let assembly = assembly.to_str().unwrap();

    let mut previous = unsafe {
        assert!(!previous.is_null());
        previous.read()
    };

    match translate(assembly) {
        Ok(translation) => {
            previous.extend(translation);
            println!("{previous:?}");
            Box::into_raw(Box::new(previous)) as *const TranslationOutput
        }
        Err(error) => {
            match error {
                ParseError::StatementSyntaxError(_, _) => unsafe { *err = -1 },
                ParseError::LabelSyntaxError(_, _) => unsafe { *err = -2 },
                ParseError::UnexpectedToken(_, _) => unsafe { *err = -3 },
                ParseError::InvalidLocation(_) => unsafe { *err = -5 },
            }

            std::ptr::null::<TranslationOutput>()
        }
    }
}

/// takes a pointer to the intermediate representation struct and links it.
/// if parsing fails, `parse_asm` will set `*err` and return `nullptr`
/// else `parse_asm` will return a pointer to an array of assembled instructions of length `len`
///
/// if any of the pointers supplied are invalid or nullptr, the function may crash
///
/// ## `*err`
/// err can be
/// - `-1`: Statement Syntax Error
/// - `-2`: Label Syntax Error
/// - `-3`: Unexpected Token
/// - `-4`: Label Not Resolved Error
/// - `-5`: Cannot set location
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn link_asm(
    translation: *const TranslationOutput,
    err: *mut i32,
    len: *mut usize,
) -> *const u16 {
    let translation = unsafe {
        assert!(!translation.is_null());
        translation.read()
    };
    match link(translation) {
        Ok(instructions) => {
            unsafe {
                *err = 0;
                *len = instructions.len();
            }

            let ptr = instructions.as_ptr();
            std::mem::forget(instructions);

            ptr
        }
        Err(error) => {
            match error {
                LinkError::LabelNotResolvedError(_, _) => unsafe { *err = -4 },
            }
            std::ptr::null::<u16>()
        }
    }
}

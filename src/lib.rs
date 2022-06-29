//! # Chasm – LC-3 Assembler
//!
//! ## build
//!
//! - install [rustup](https://rustup.rs/) and use it to install `rustc` and `cargo`
//!  
//! ```sh
//! $ touch main.asm
//! $ echo "AND R1, R1, #0" >> main.asm
//! $ cargo run -- main.asm -o main.bin
//! $ # run chasm in debug mode
//! $ cargo run --release -- main.asm -o main.bin
//! $ # run chasm in release mode
//! ```
//!
//! ## quality
//!
//! ```sh
//! $ cargo test # run tests
//! $ cargo clippy # run lints
//! $ cargo doc --open # generate documentation
//! ```

pub mod interface;
pub mod lc_3;
pub mod parser;

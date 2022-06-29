# Chasm – LC-3 Assembler

Chasm is an assembler for the [LC-3](https://en.wikipedia.org/wiki/Little_Computer_3) assembly language.

To execute the generated binary, use a virtual machine such as [NALE](https://github.com/Qwendu/NALE)

## build

- install [rustup](https://rustup.rs/) and use it to install `rustc` and `cargo`

```console
$ touch main.asm
$ echo "AND R1, R1, #0" >> main.asm
$ # run chasm in debug mode
$ cargo run -- main.asm -o main.bin
$ # run chasm in release mode
$ cargo run --release -- main.asm -o main.bin
```

## The LC-3 Assembly Language

*section to be added*

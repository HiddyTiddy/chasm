
## features

### instructions

- [x] ADD
- [x] AND
- [x] BR
- [x] JMP
- [x] RET
- [x] JSR
- [x] JSRR
- [x] LDB
- [x] LDW
- [x] LEA
- [x] RTI
- [x] SHF
- [x] STB
- [x] STW
- [x] TRAP
- [x] XOR
- [x] NOT

- [x] HALT
- [x] GETC
- [x] OUT
- [x] PUTS
- [x] IN

### data management

- [x] load static data

escaped literals (e.g. \x69) could be invalid char
a) rewrite scanner to work on bytearrays
b) parse escaped literals in a later stage

## other

- include

## interface

### CLI

- [x] add clap

### dylib

expose `C`-callable function

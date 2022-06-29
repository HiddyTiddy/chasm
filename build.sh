#!/bin/sh

set -xe

# only when changing rust code
cbindgen --config cbindgen.toml --crate chasm --output chasm.hpp
cargo build

# build c prog
g++ example.cpp -L ./target/debug -lchasm -o example

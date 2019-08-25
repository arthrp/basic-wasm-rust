# basic-wasm-rust [![Build Status](https://travis-ci.org/arthrp/basic-wasm-rust.svg?branch=master)](https://travis-ci.org/arthrp/basic-wasm-rust)
Extremely simple example of using functions written in Rust from JS via WebAssembly. This is pure Rust and pure wasm, no frameworks or "glue" JS files. As currently WASM understands only numeric types, operations are performed on numbers.

## How to run

1. Make sure you have Rust and *wasm32-unknown-unknown* toolchain
2. Run build.sh
3. Run index.html from browser that supports running wasm from filesystem (Firefox) or serve index.html and wasm file from some simple server.

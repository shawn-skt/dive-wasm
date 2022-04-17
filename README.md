# dive-wasm

![](https://img.shields.io/badge/blog-%40shawn--skt-success.svg?tyle=plastic)

This is a repo of the research on the consistency of the  WebAssembly representation of float-point number on x86 and ARM and some ideas about the binary representations of fixed pint type numbers in Liquid smart contract.

## Aims

Sharing our conduct with wasm learners.

## Contributors

Conducted by `Shuo Yang` and `Wei Li` from `Sun Yat-sen University` during the internship in `WeBank`, Shenzhen, China, Decemeber 10, 2021.

## Overview

### Representation Tests

`Rust`: folder `src`

```sh
$ cargo build
$ cargo run
```

`C++`: folder `c++`

```sh
$ cd c++
$ emcc hello.cpp -s WASM=1 -o hello.html
$ serve .
```

`Web` injection: folder `hello-wasm`

```sh
$ cd hello-wasm
$ wasm-pack build --target web
$ serve .
```

### Ewasm
Compilers `Solc 0.8.x` and `Solang` support ewasm

Some demo tests

```sh
./solc -o ./solidity/solc_res --ewasm ./solidity/contract.sol
./solang ./solidity/contract.sol --target ewasm -o ./solidity/solang_res
```


### Dependencies

1. wasmer

2. wasmtime

3. wasm-pack

4. node module: serve

5. emscripten




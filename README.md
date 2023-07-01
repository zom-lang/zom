# ⚡ Zom Language

[![lines of code](https://tokei.rs/b1/github/zom-lang/zom)](https://github.com/Aaronepower/tokei)
[![License][licence-badge]](https://github.com/zom-lang/zom/tree/main#license)
[![Rust tests](https://github.com/zom-lang/zom/actions/workflows/rust.yml/badge.svg)](https://github.com/zom-lang/zom/actions/workflows/rust.yml)

[licence-badge]: https://img.shields.io/badge/License-%20Apache--2.0%20with%20LLVM--Exception-lightblue

Zom is a Ahead Of Time compiled programming language, the code generation and compilation is made with LLVM.  

## Features

- **Ahead of Time** compiled programmaing language, that mean you will have great performance, because the transformation of the source code to assembly is done before. Contrary to Just In Time compilation who's occur at the same time as the execution.
- **Statically typed**, Zom solves types at compile time, and not at run time.
- **Performance**, because Zom is Ahead of Time compiled, all the hard work is done before the execution. And because Zom uses LLVM, a bunch of optimization is made at compile-time.
- **Safety and Simplicity**, Zom is safe and simple at the same time, because it doesn't have a very strict design.
- **Memory managment**, Zom use static analysis and a system inspired by the Rust Borrow checker.

> For the moment some of the features are not yet implemented.

## Usage

After [build](#build-zom) Zom, just run it and you will see that :
```
Usage: zom <COMMAND>

Commands:
  bobj        Builds a given file into an object file
  version     Get the current version of Zom
  get-target  Get the current target detected by LLVM
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

There are

- `bobj`, it's the contraction of `build an object`, this will transform the file passed in arguments and compiles it to an object file.
- `version`, that output the current version of Zom
- `get-target`, return the target found by LLVM.

## Build Zom

To build the source code of Zom, there are three steps :
1. Clone the repository / download the source code
2. Build with Cargo, in the root of the repository, `cargo build --all-targets --release`
3. The binary, now is in `./target/release/zom`, you can put it in your binary folder, use it like that etc...

## Work to be done :
- [x] Lexer
- [x] Parser
- [x] A AOT compiler
- [ ] A good error system for the parser and the compiler -> https://github.com/zom-lang/zom/issues/4
- [ ] Implement the docs ;)

## Source layout:
```
Cargo.toml               - Manifest for Cargo workspace

NOTICE                   - NOTICE file for the Apache-2.0 license for Zom
LICENSE                  - The Apache-2.0 license of Zom with LLVM-Exception.


zom/                     - Binary for the Zom Compiler.
zom_fe/                  - Crate where the lexer, parser, token list and AST are.
zom_common/              - Common crate for Zom like, errors etc..
zom_codegen/             - Crate responsible for the generation of the LLVM IR.
zom_compiler/            - Where the transformation of the LLVM IR to object files and then binary / lib.

docs/                    - The documentation of how Zom works
docs/lang/               - The documentation of the Zom programming language
docs/lang/000-readme.md  - Readme of the Zom Lang's doc
```

## License

Licensed under Apache License, Version 2.0 [LICENSE](/LICENSE) or <http://www.apache.org/licenses/LICENSE-2.0> 
with LLVM-exception <https://foundation.llvm.org/relicensing/LICENSE.txt>

This files may not be copied, modified, or distributed except according to those terms.

> More informations [here](/NOTICE).

## Contribution

Feel free to contribute. For the moment there is a documentation but it needs to be improved.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.

A much more detailed version, on how to contribute to Zom can be found [here](/CONTRIBUTING.md)

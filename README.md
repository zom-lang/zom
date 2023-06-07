# Mona

[![lines of code](https://tokei.rs/b1/github/Larsouille25/mona)](https://github.com/Aaronepower/tokei)
[![codecov](https://codecov.io/gh/Larsouille25/mona/branch/main/graph/badge.svg)](https://github.com/Larsouille25/mona)
[![MIT/Apache][licence-badge]](https://github.com/Larsouille25/mona/tree/main#license)

[licence-badge]: https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-lightgrey

Mona is a Ahead Of Time compiled programming language written in Rust, the code generation and compilation is made with LLVM. 

## Features

- **Ahead of Time** compiled programmaing language, that mean you will have great performance, because the transformation of the source code to executable code is done before. Contrary to Just In Time compilation who's occur at the same time as the execution.
- **Statically typed**, Mona solves types at compile time, and not at run time and you'll also know when you develop with Mona, the type of variables / arguments.
- **Performance**, because Mona is Ahead of Time compiled, all the hard work is done before the execution.
- **Safety and Simplicity**, Mona is safe and simple at the same time, because it doesn't have a very strict design.
- **Memory managment**, Mona uses a Garbage collector, for the heap allocation, it is developed in safe Rust. The GC is simple and lightweight but have all the functionnality needed.

> For the moment some of the features are not yet implemented.

## Usage

After [build](#build-mona) Mona, just run it and you will see that :
```
Usage: mona [OPTIONS] <COMMAND>

Commands:
  bobj        Builds a given file into an object file
  version     Get the current version of Mona
  get-target  Get the current target detected by LLVM
  help        Print this message or the help of the given subcommand(s)

Options:
  -V, --verbose  Print verbose ouput if enabled
  -h, --help     Print help
```

There are

- `bobj`, it's the contraction of `build an object`, this will transform the file passed in arguments and compiles it to an object file.
- `version`, that output the current version of Mona
- `get-target`, return the target found by LLVM.
- and `--verbose`, that output more details.

## Build Mona

To build the source code of Mona, there are three steps :
1. Clone the repository / download the source code
2. Build with Cargo, in the root of the repository, `cargo build --all-targets --release`
3. The binary, now is in `./target/release/mona`, you can put it in your binary folder, use it like that etc...

## Work to be done :
- [x] Lexer
- [x] Parser
- [ ] ~~Interpreter~~
- [x] ~~Make a good [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop)~~
- [ ] ~~a JIT with LLVM ?~~
- [ ] A AOT compiler
- [ ] Optimization and cleaning
- [ ] A good error system for the parser and the compiler -> https://github.com/Larsouille25/mona/issues/4
- [ ] Implement the docs ;)

## Source layout:
```
Cargo.toml               - Manifest for Cargo workspace
LICENSE-APACHE           - The Apache-2.0 license of Mona
LICENSE-MIT              - The MIT license of Mona


mona/                    - Binary crate.
mona_fe/                 - Crate where the lexer, parser, token list and AST are.
mona_common/             - Common crate for Mona like, errors etc..
mona_codegen/            - Crate responsible for the generation of the LLVM IR.
mona_compiler/           - Where the transformation of the LLVM IR to object files and then binary / lib.

docs/                    - The documentation of how Mona works
docs/lang/               - The documentation of the Mona programming language
docs/lang/000-readme.md  - Readme of the Mona Lang's doc
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

> More informations [here](/COPYRIGHT).

## Contribution

Feel free to contribute. For the moment there is a documentation but it needs to be improved.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

A much more detailed version, on how to contribute to Mona can be found [here](/CONTRIBUTING.md)

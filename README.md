# Mona

[![lines of code](https://tokei.rs/b1/github/Larsouille25/mona)](https://github.com/Aaronepower/tokei)
[![codecov](https://codecov.io/gh/Larsouille25/mona/branch/main/graph/badge.svg)](https://github.com/Larsouille25/mona)
[![MIT/Apache][licence-badge]](https://github.com/Larsouille25/mona/tree/main#license)

[licence-badge]: https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-lightgrey

Mona is a Ahead Of Time compiled programming language written in Rust, the code generation and compilation is made with LLVM. 

## Usage

After [build](#) Mona, just run it and you will see that :
```
Usage: mona <COMMAND>

Commands:
  bobj  Builds a given file into an object file
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

For now only one subcommand exits, it's `bobj`, it's the contraction of `build an object`, this will transform the file passed in arguments and compiles it to an object file.

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
Cargo.toml               - Manifest for Cargo, where version, doc link, dependencies etc is 
LICENSE-APACHE           - The Apache-2.0 license of Mona
LICENSE-MIT              - The MIT license of Mona

src/               The source code folder
src/main.rs              - Entry of the binary
src/driver.rs            - The logic behind the REPL
src/lib.rs               - Where modules are declared

src/error.rs             - Common functions that error structs use 
src/error/               - Parser, Lexer, Runtime error for Mona

src/fe/                  - Tokens, Lexer, Parser, AST .. Front-End
src/fe/lexer.rs          - Lexing logic
src/fe/parser.rs         - Parsing occurs here
src/fe/token.rs          - Tokens of Mona

src/gc/                  - Garbage Collector (not yet implemented)

src/typesys/             - Type System (not yet implemented)
src/typesys/primitive.rs - Primitive types of Rust (not yet implemented)
src/typesys/class.rs     - Class in Mona (not yet implemented)

docs/                    - The documentation of Mona works
docs/lang/               - The documentation of the Mona programming language
docs/lang/000-readme.md  - Readme of the Mona's doc

benches/           The benchmarks of every component of Mona
benches/lexer_bench.rs   - Lexer benchmarks
benches/parser_bench.rs  - Parser benchmarks
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution
Feel free to contribute. For the moment there is a documentation but it needs to be improved.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.

And thanks to all the people who helped me when I had issues with the borrow checker 😂

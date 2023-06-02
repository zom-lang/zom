# Mona

[![lines of code](https://tokei.rs/b1/github/Larsouille25/mona)](https://github.com/Aaronepower/tokei)
[![codecov](https://codecov.io/gh/Larsouille25/mona/branch/main/graph/badge.svg)](https://github.com/Larsouille25/mona)
[![MIT/Apache][licence-badge]](https://github.com/Larsouille25/mona/tree/main#license)

[licence-badge]: https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-lightgrey

Mona is an interpreted programming language written in Rust. Run Mona code with the REPL or passing a source file.

## How to use it?

It's not fully implemented yet, but you can use it. After compiling Mona, run it like this:

```
$ ./mona
Mona 0.1.0-alpha, to exit enter `.quit`
~> 
``` 
Type anything you want, if there is an error it'll show you like that for lexing errors: 

```
Err: Lexer, in file `<stdin>` at line 1 :
 ... |
  1  | 1 + 2 * $
 ... |         ^
               Illegal Character
```

By default when you type nothing shows up, it's normal you need to enable flags. By typing `./mona --help`, you will see a bunch of flags, if you want to see the result of the lexer add `-l` to the command, and if you want to also see the result of the parser you just need to add `-p`. The command will look like that `./mona -lp`.

e.g:

```
$ ./mona
Mona 0.1.0-alpha, to exit enter `.quit`
~> (2 + 8) * 4
 Lexer : 
[OpenParen, Int(2), Operator("+"), Int(8), CloseParen, Operator("*"), Int(4)]

 Parser : 
[
    FunctionNode(
        Function {
            prototype: Prototype {
                name: "",
                args: [],
            },
            body: BinaryExpr(
                "*",
                BinaryExpr(
                    "+",
                    LiteralExpr(
                        2,
                    ),
                    LiteralExpr(
                        8,
                    ),
                ),
                LiteralExpr(
                    4,
                ),
            ),
        },
    ),
]

```
The slice is the Mona AST and the vector is the vector that is passed to the parser of Mona.

## Work to be done :
- [x] Lexer
- [x] Parser
- [ ] ~~Interpreter~~
- [x] Make a good [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop)
- [ ] a JIT with LLVM ?
- [ ] Optimization and cleaning
- [ ] A good error system for the parser and the JIT -> https://github.com/Larsouille25/mona/issues/4
- [ ] Implement the docs ;)
- [ ] A AOT compiler ?!...

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

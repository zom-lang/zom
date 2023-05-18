# Mona
Mona is an interpreted programming language written in Rust. Run Mona code with the REPL or passing a source file.

## How to use it?

It's not fully implemented yet, but you can use it. After compiling Mona, run it like this:

```
$ ./mona
Mona 0.0.0-dev, to exit enter `.quit`
~> 
``` 
Type anything you want, if there is an error it'll show you like that for lexing errors: 

```
Err: Lexer, in file `<stdin>` at line 1 :
 ... |
  1  | 1 + 2 * a
 ... |         ^
               Illegal Character
```
Just keep in mind that for the moment, Mona can only lexer numbers and math operators like `+` `-` `/` `*`.

By default when you type nothing shows up, it's normal you need to enable flags. By typing `./mona --help`, you will see a bunch of flags, by default the interpreter flag is enabled but if you want to see the result of the lexer add `-l` to the command, and if you want to also see the result of the parser you just need to add `-p`. The command will look like that `./mona -lp`.

e.g:

```
$ ./mona
Mona 0.0.0-dev, to exit enter `.quit`
~> (2 + 8) * 4
[LParen, Int(2), Plus, Int(8), RParen, Mul, Int(4)]

ParseNode {
    children: [
        ParseNode {
            children: [
                ParseNode {
                    children: [],
                    entry: Int(
                        2,
                    ),
                },
                ParseNode {
                    children: [],
                    entry: Int(
                        8,
                    ),
                },
            ],
            entry: Plus,
        },
        ParseNode {
            children: [],
            entry: Int(
                4,
            ),
        },
    ],
    entry: Mul,
}
```
The ParseNode is the Mona AST and the vector is the vector that is passed to Mona

## Work to be done :
- [x] Lexer
- [x] Parser
- [ ] ~~Interpreter~~
- [x] Make a good [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop)
- [ ] a JIT with LLVM ?
- [ ] Optimization and cleaning
- [ ] A good error system for the parser and the interpreter / JIT -> [#4](https://github.com/Larsouille25/mona/issues/4)
- [ ] Implement the docs ;)

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

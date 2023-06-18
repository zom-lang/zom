# Evolution of Zom

For now, the compiler is written with Rust, but it will be written in Zom.
Some steps need to be done before we can start to make a self-compiling compiler :

- [ ] Write a grammar that will that wont change much
- [ ] Implement the basis -> [#25](https://github.com/zom-lang/zom/issues/25) + Enumerations.
- [ ] A multi file system -> [#26](https://github.com/zom-lang/zom/issues/26)
- [ ] A standard library :
  - [ ] `stdout`, `stdin`, `stderr` functions like `print()` or `input()`, things like that
  - [ ] Some iterators, like peekable etc..
- [ ] A way to create bindings between C and Zom (and why not C++ and Zom and Rust and Zom)
- [ ] Create the C bindings of LLVM in Zom

*and finally,*
- [ ] Tranlate the Zom compiler written in Rust to a Zom Compiler written in Zom.

## What the lang will look like ?

You can find in `/docs/evolution.zom`, code that (for now don't compile) show how Zom will work.
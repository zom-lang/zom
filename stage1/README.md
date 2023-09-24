Stage 1
=======

This directory contains the stage 1 of the Zom Compiler.

Why stage 1 ?
-------------

Because for now the compiler is written in Rust and C++ but it may be self hosted at some point.

Stage 1 architecture
--------------------

```
zomc                    Zom Compiler CLI, contains the 'main' function.
zom_lexer               Lexer, transform the text input into a vector of Tokens.
zom_parser              Parser, transform a vector of Tokens into HLIR
zom_common              Common, contains shared behavior between zom compiler packages.
                        some content of this package may move to its own package
zom_errors              Errors, contains the error system, used to show pretty error messages.
```

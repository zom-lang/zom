# ⚡ Zom Language

[![lines of code](https://tokei.rs/b1/github/zom-lang/zom)](https://github.com/Aaronepower/tokei)
[![License][licence-badge]](https://github.com/zom-lang/zom/tree/main#license)
[![Zom checks](https://github.com/zom-lang/zom/actions/workflows/checks_zom.yml/badge.svg)](https://github.com/zom-lang/zom/actions/workflows/checks_zom.yml)
[![discord server](https://img.shields.io/discord/1115546838729240596?label=Discord%20Server&color=5765F2)](https://discord.gg/pcDknYP9Bf)

[licence-badge]: https://img.shields.io/badge/License-%20Apache--2.0%20with%20LLVM--Exception-lightblue

> **Warning**
> the compiler doesn't actually works, and even if it did, the Zom Language is experimental and shouldn't be used in production for now.

Zom is a Ahead Of Time compiled system programming language.

<!--
## Features

* **Safe**, using a system inspired by C++ *RAII* and Rust *OBRM*, Zom automatically clears data when they can't be accessed anymore.
* **Comptime**, Zom doesn't have macros or preprocessor but compile-time execution of code without RT overhead.
* **Pointers**, Zom have pointers; slices, a pointer with a runtime length, and array, a pointer with length at compile-time, to empower safety.
* **Type as value**, Zom treat types as values. You can pass type as argument to functions, 

## Examples

* **Hello World!**

```zom
const print = @import("std").debug.print;

func main() void {
  print("Hello world!");
}
```

* **Recursive Fibonacci**

```zom
func fibonacci(n: u32) u32 {
  if n <= 1 {
    return n;
  }
  fibonacci(n - 1) + fibonacci(n - 2)
}
```

* **Generics**

*This code is experimental and doesn't actually works (and compile), it's just a proof of concept.*

```zom
/// Reference Counted Smart Pointer
pub func Rc(T: type) type {
  const RcInner = struct {
    count: usize,
    data: T,
  };
  return struct {
    ptr: *RcInner,

    /// Create an Rc for a Pointer
    pub func from_ptr(ptr: *T) Self {
      return Rc(T) {
        ptr: _ {
          count: 1,
          ptr: *T
        }
      }
    }

    /// Increase the count of references to the data.
    func increase_count(self: *Self) void {
      self->ptr->count += 1;
    }

    /// Decrease the count of references to the data.
    func decrease_count(self: *Self) void {
      self->ptr->count -= 1;
    }

    /// Clone the Rc into a new Rc.
    pub func clone(self: *Self) Self {
      self->increase_count();
      return Self {
        ptr: self->ptr
      }
    }
  }
}

/// Implementation of the trait 'Drop' for every Rc, because T is a generic expression.
impl(T) Drop for Rc(T) {
  func drop(self: Self) void {
    (&self).decrease_count();
    if self.ptr->count == 0 {
      self.ptr->data.drop();
    }
  }
}
```
-->
## Project status

I'm a student, and so developing the Zom compiler isn't my priority, this is why you can see some
inactiveness in my commits and even if I don't actually write code, I think alot what features I
could implement, how, and what the language will look.

So if you're asking, no the project isn't dead, it's the opposit, but I don't have the time to make
my ideas real.

And for know it's not production-ready if you're asking  ¯\_(ツ)_/¯

## Usage

Not yet done, see [#42](https://github.com/zom-lang/zom/issues/42)

## Build Zom

**TODO, make a simple bash script**

## Source layout:
```
Cargo.toml               - Cargo Workspace Manifest

NOTICE                   - NOTICE file for the Apache-2.0 license for Zom
LICENSE                  - The Apache-2.0 license of Zom with LLVM-Exception.

stage1/                  - Stage 1 of the compiler.
       zomc/             - Zom Compiler Binary
       zom_lexer/        - Lexer, transform the text input into a vector of Tokens.
       zom_parser/       - Parser, transform a vector of Tokens into HLIR.
       zom_common/       - Common, contains shared behavior between zom compiler packages.
                          some content of this package may move to its own package
       zom_errors/       - Errors, contains the error system, used to show pretty error messages.
       zom_codegen/      - (DEPRECATED -> #44) Crate responsible for the generation of the LLVM IR.
       zom_compiler/     - (DEPRECATED -> #44) Where the transformation of the LLVM IR to object files and then binary / lib.

docs/                    - Documentation of the Zom Project.
docs/zomlang/            - Documentation of the Zom programming language.
docs/compiler/           - Documentation of the Zom Compiler.
```

## License

Licensed under Apache License, Version 2.0 [LICENSE](/LICENSE) or <http://www.apache.org/licenses/LICENSE-2.0> 
with LLVM-exception <https://foundation.llvm.org/relicensing/LICENSE.txt>

This files may not be copied, modified, or distributed except according to those terms.

> More informations [here](/NOTICE).

## Contribution

Feel free to contribute. For the moment there is a documentation but it needs to be improved #22.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.

A much more detailed version, on how to contribute to Zom can be found [here](/CONTRIBUTING.md)

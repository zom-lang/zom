> **Warning**
> the compiler doesn't actually works, and even if it did, the Zom Language is highly experimental and shouldn't be used in production for now.

# Introduction

⚡ Zom, a safe system programming.

I will highly appreciate your contribution on the Zom Project !

## Features

* **Safe**, using a system inspired by C++ *RAII* and Rust *OBRM*, like that everything is cleaned up when it needs.
* **Comptime**, Zom doesn't have macros or preprocessor but compile-time execution of code without RT overhead.
* **Pointers**, Zom have pointers, pointers with length at RT / comptime, to empower safety.
* **Type as value**, Zom treat types as values so you define types like global variables, Zom handles generics by
  passing type as argument; without runtime overhead.

## Examples

* **Hello World!**

```zom
const print = @import("std")::debug::print;

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


# Syntax of Mona Programming Language

In this file you will find the syntax / grammar of mona.

  Todo :
- [ ] Variable
  - [ ] Declaration
  - [ ] Change stored value
  - [ ] Use
  - [ ] Primitive types
    - [ ] byte
    - [ ] int
    - [ ] long
    - [ ] long long
    - [ ] unsined byte
    - [ ] unsigned int
    - [ ] unsigned long
    - [ ] unsigned long long
    - [ ] float
    - [ ] double
    - [ ] quadruple
    - [ ] bool
    - [ ] char
    - [ ] string
- [ ] Function
  - [ ] Declaration
  - [ ] Use

## Variable

### Declaration

```
let foo: int =s 10;
```
This will declare a variable, with `foo` name, `int` type and a initial value of `10`.

### Change stored value

``` 
foo = 20;
```
This will change the stored value inside the previously created foo variable, to 20.

### Use

```
foo * 8
```
This exression will multiply foo by 8.

If you just want to use the variable you can just type the variable name : 
```
foo
```

### Primitive types

Primitive types are type directly encoded in the interpreter.

Signed numbers works with two's complement
|     Name     |             Description                 |
| ------------ | --------------------------------------- |
|    `byte`    | `byte` is a 8 bits signed integer,      |
|     `int`    | `int` is a 16 bits signed integer       |
|     `long`   | `long` is a 32 bits signed integer      |
|  `long long` | `long long` is a 64 bits signed integer |

You can use the keyword before long or int to unsigned make an unsigned integer :
|          Name        |                  Description                       |
| -------------------- | -------------------------------------------------- |
|   `unsigned byte`    | `unsigned byte` is a 8 bits unsigned integer       |
|   `unsigned int`     | `unsigned int` is a 16 bits unsigned integer       |
|   `unsigned long`    | `unsigned long` is a 32 bits unsigned integer      |
| `unsigned long long` | `unsigned long long` is a 64 bits unsigned integer |

Floating numbers, specified in the [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754) 
|     Name    |                             Description                           |
| ----------- | ----------------------------------------------------------------- |
|    `float`  | `float` is a single precision floating number (32bits)            |
|   `double`  | `double` is a double precision floating number (32bits)           |
| `quadruple` | `quadruple` is a quadruple precision floating number (128bits)    |

Others primitive types,
|   Name   |                           Description                          |
| -------- | -------------------------------------------------------------- |
|  `bool`  | `bool` is a boolean value; either set to true (1) or false (0) |
|  `char`  | `char` is a Unicode scalar value, that is 4 bytes each         |
| `string` | `string` is a UTF-8-encoded, growable string                   | 

## Functions

### Declaration

```
func foo(bar: int) int {
    return 32 * 9;
}
```
This will declare a function with `foo` name, `bar` as integer argument and return an int;

If your function do not return anything, `void` : 
```
func bar(foo: int) void {
    ...
}
```

### Use

```
bar(12);
```
This snippet of code will call the `bar` function with `12` as `foo` argument.




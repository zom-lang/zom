# Syntax of Mona Programming Language

In this file you will find the syntax / grammar of mona.
<Details>
<summary>Todo</summary>

- [ ] Variable
  - [ ] Declaration
  - [ ] Change refered value
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
    - [ ] bool
    - [ ] char
    - [ ] string
- [ ] Control flow
  - [ ] Conditions operators
  - [ ] If statement
- [ ] Function
  - [ ] Declaration
  - [ ] Use
- [ ] Basic Maths
  - [ ] Addition
  - [ ] Soustraction
  - [ ] Multiplication
  - [ ] Division
  - [ ] Modulo
  
</Details>

## Variable

A variable can have a `None` value beacue Mona uses a Reference Counting system and by default, if you don't initialize the variable, the variable will have a `None` value but therefore it's mandatory to specify its type. The `None` value is possible because the variable will refer to nothing.

### Declaration

```
let foo: int = 10;
```
This will declare a variable, with `foo` name, `int` type and a initial value of `10`. but it is not mandatory to set the type of the variable if you initialize with a value that the interpreter know :
``` 
let foo = 10;
```
And that also declare a variable, with name `foo`, an `int` type (you'll see below why) and a stored value of `10`.

```
let foo: float;
let bar: int = None;
```
This will declare a `foo` variable of type float and with a `None` value and `bar` is also a variable of type int and with a `None` value but in a more implicit way.

### Change refered value

``` 
foo = 20;
```
This will change the stored value inside the previously created foo variable, to 20.

### Use

```
foo * 8
```
This exression will multiply `foo` by 8.

If you just want to use the variable you can just type the variable name : 
```
foo
```

### Primitive types

Primitive types are types that are directly encoded in the interpreter.

Signed numbers work with [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement)
|          Name         |            Description                |                           Range                          |                Default?               |
| --------------------- | ------------------------------------- | -------------------------------------------------------- | ------------------------------------- |
|    `(signed) byte`    | `byte` is a 8 bits signed integer,    | -127 to +127                                             |                   No                  |
|    `(signed) short`   | `short` is a 16 bits signed integer   | −32_767 to +32_767                                       |                   No                  |
|     `(signed) int`    | `int` is a 32 bits signed integer     | −2_147_483_647 to +2_147_483_647                         |                   Yes                 |
|     `(signed) long`   | `long` is a 64 bits signed integer    | −9_223_372_036_854_775_807 to +9_223_372_036_854_775_807 | Yes, when number don't fit in 32 bits |
> By default numbers are signed but, you can use the "signed" keyword to explicitly say it's signed (that's why in the table `signed` is in brackets).
> The `Default?` column tell that the interpreter when no type is given in a declaration will choose `signed int` by default or `signed long` if the number do not fit in 32 bits.
You can type a number like that `123456` or like that if the number is big or it's complex to read it `123_456`.

You can use the keyword `unsigned` before a primitive type number and this will remove the two's complement :
|          Name        |                  Description                       |              Range              | Default? |
| -------------------- | -------------------------------------------------- | ------------------------------- | -------- |
|   `unsigned byte`    | `unsigned byte` is a 8 bits unsigned integer       | 0 to 255                        |    No    |
|   `unsigned short`   | `unsigned short` is a 16 bits unsigned integer     | 0 to 65_535                     |    No    |
|    `unsigned int`    | `unsigned int` is a 32 bits unsigned integer       | 0 to 4_294_967_295              |    No    |
|   `unsigned long`    | `unsigned long` is a 64 bits unsigned integer      | 0 to 18_446_744_073_709_551_615 |    No    |
> The `Default?` column tell that the interpreter when no type is given in a declaration will choose `signed int` by default or `signed long` if the number do not fit in 32 bits.

Floating numbers, specified in the [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754) 
|     Name    |                             Description                           |                                                        Range                                                       | Default? |
| ----------- | ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ | -------- |
|    `float`  | `float` is a single precision floating number (32bits)            | [`≈ 1.18 × 10^-38` to `≈ 3.4028235 × 10^38`](https://en.wikipedia.org/wiki/Single-precision_floating-point_format) |    Yes   |
|   `double`  | `double` is a double precision floating number (64bits)           | [`≈ -7.2 × 10^75` to `≈ 7.2 × 10^75`](https://en.wikipedia.org/wiki/Double-precision_floating-point_format)        |    No    |
> The `Default?` column tell that the interpreter when no type is given in a declaration will choose `signed int` by default or `signed long` if the number do not fit in 32 bits.

You can type a float number like that `123456,789012` or like that if the number is too big or it's complex to read it, `123_456,789_012`

Others primitive types,
|   Name   |                           Description                          |
| -------- | -------------------------------------------------------------- |
|  `bool`  | `bool` is a boolean value; either set to true (1) or false (0) |
|  `char`  | `char` is a Unicode scalar value, that is 4 bytes each         |
| `string` | `string` is a UTF-8-encoded, growable string                   | 

Boolean is `true` or `false`.

A char is initialized with an apostrophe and the char in between, like that : `'A'`.

A string is initialized with a quotation mark and the string in between, like that : `"Hello, world!"`.

## Control Flow

The ability to run some code depending on whether a condition is true and to run some code repeatedly while a condition is true are basic building blocks in most programming languages. The most common constructs that let you control the flow of execution of Mona code are if expressions and loops.

### Conditions operators

Before starting, it's important to know condition operators to know how to use control flow.

#### Equal, `==`

The equals operator, of sign `==` is used when you want to check the equality of two objects, and if the objects are equal the operator return true. ex: `a == b`.

#### Different, `!=`

The different operator, of sign `!=` is used when you want to check the non-equality of two objects, and if the objects are not equal the operator return true. e.g.: `a =! b`.

#### Less than, `<`

The `less than` operator, of sign `<` is used when you want to check if the first object is strictly less than the second object, and if the first object is less than the second object, the operator return true. e.g.: `a < b`.

#### More than, `>`

The `more than` operator, of sign `>` is used when you want to check if the first object is strictly bigger than the second object, and if the first object is bigger than the second object, the operator return true. e.g.: `a > b`.

#### Less than or equal to, `=<`

The `less than or equal to` operator, of sign `=<` is used when you want to check if the first object is less than or equal to the second object, and if the first object is less than or equal to the second object, the operator return true. e.g.: `a =< b`.

#### More than or equal to, `=>`

The `more than or equal to` operator, of sign `=>` is used when you want to check if the first object is more than or equal to the second object, and if the first object is less than or equal to the second object, the operator return true. e.g.: `a => b`.


### Multiple conditions

If you want to perform multiple conditions in one condition, you can use the following operators to do it, instead of nesting conditions.

> In the following exaples, `*condition*` refers a condition.

#### Not, `!`

The `not` operator can be used if you want to invert the result of a condition. e.g.: `!*condition*`.

#### And, `&&`

The `and` operator check if two conditions are both `true` and return `true`. e.g.: `*condition* && *condition*`.

#### Or, `||`

The `or` operator check if one or both of the conditions is true conditions and return `true`. e.g.: `*condition* || *condition*`.

### If statement

An `if` expression allows you to branch your code depending on conditions. You can provide a condition and then state, "If this condition is met, run this block of code. If this condition is not met, do not run this block of code but instead this one if it's true ..."

You can state an `if` with the `if` keyword, followed by a condition, and then a block of code to execute like this :

``` 
if *condition* {
  *code*
}
```
`*condition*` refers to a boolean condition and `*code*` refers to the Mona code you want to perform if the condition is true.

e.g.:
```
let foo: int = 14;

if foo == 5472 {
  // code here ...
}
```
#### Else if

If the condition was `false` you can use `else if` after an `if` statement and if the `if` statement after the else was met then the block of code will be executed. 

You can state an `else if` with the `else if` keyword, followed by a condition, and then a block of code to execute like this :

```
if *condition* {
  *code*
}else if *condition* {
  *code*
}
```

You can add as many as you want `else if` statement after an `if` :
```
if *condition* {
  *code*
}elif *condition* {
  *code*
}elif *condition* {
  *code*
}elif *condition* {
  *code*
}
```

#### Else

If the condition wasn't met in an `if` or an `else if`, you can use `else`. You can state an `else` with the `else` keyword, followed by the block of code to execute

You can state an `else` with the `else` keyword, followed by a condition, and then a block of code to execute like this :

```
if *condition* {
  *code*
}else {
  *code*
}
```

or after a `elif` :

```
if *condition* {
  *code*
}elif *condition* {

} else {
  *code*
}
```

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


## Basic maths

You can do simple maths, for integers and floats the behavior of their operands is directly integrated in the interpreter.
Like math, you can "set" priority with parentheses. And by default the priorities are:
- Brackets
- Addition / Subtraction
- Multiplication / Division

Examples :
`a + b`, with priorities to the addition : `(a + b) * c`

### Addition
You can perform addition with the plus character : `+`. e.g.: `a + b`

### Soustraction
You can perform soustractions with the minus character : `-`.e.g.: `a - b`

### Multiplication
You can perform multiplication with the times character : `*`. e.g.: `a * b`

### Division
You can perform division with the divide character : `/`. e.g.: `a / b`

### Modulo
You can perform modulo with the percent character : `%`. e.g.: `a % b`

### Power
You can perform power with the circumflex character : `^`. e.g.: `a ^ b`

- **Feature Name:** `primitive-types` 
- **Zom Issue:** *Number doesn't correspond* [`mona#010`](https://github.com/Larsouille25/mona/issues/10)
- **Status:** `Not implemented yet`

# Primitive types

Primitive types are types that are directly encoded in Zom.

Signed numbers work with [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement)
|     Name     |            Description                |                           Range                          | Default?  |
| ------------ | ------------------------------------- | -------------------------------------------------------- | --------- |
|    `sbyte`   | `sbyte` is a 8 bits signed integer,   | -127 to +127                                             |     No    |
|    `short`   | `short` is a 16 bits signed integer   | −32_767 to +32_767                                       |     No    |
|     `int`    | `int` is a 32 bits signed integer     | −2_147_483_647 to +2_147_483_647                         |    Yes    |
|     `long`   | `long` is a 64 bits signed integer    | −9_223_372_036_854_775_807 to +9_223_372_036_854_775_807 |   Maybe   |
> By default numbers are signed but, you can use the "signed" keyword to explicitly say it's signed (that's why in the table `signed` is in brackets).
> The `Default?` column tell that the interpreter when no type is given in a declaration will choose `signed int` by default or `signed long` if the number do not fit in 32 bits.
You can type a number like that `123456` or like that if the number is big or it's complex to read it `123_456`.

You can use the keyword `unsigned` before a primitive type number and this will remove the two's complement :
|      Name    |            Description                 |              Range              | Default? |
| ------------ | -------------------------------------- | ------------------------------- | -------- |
|   `ubyte`    | `ubyte` is a 8 bits unsigned integer   | 0 to 255                        |    No    |
|   `ushort`   | `ushort` is a 16 bits unsigned integer | 0 to 65_535                     |    No    |
|    `uint`    | `uint` is a 32 bits unsigned integer   | 0 to 4_294_967_295              |    No    |
|   `ulong`    | `ulong` is a 64 bits unsigned integer  | 0 to 18_446_744_073_709_551_615 |    No    |
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

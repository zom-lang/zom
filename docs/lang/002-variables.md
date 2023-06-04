- **Feature Name:** `variables` 
- **Mona Issue:** Not related to an issue 
- **Status:** `Not yet implemented`


# Variables

A variable can have a `None` value beacue Mona uses a garbage collection system and by default, if you don't initialize the variable, the variable will have a `None` value but therefore it's mandatory to specify its type. The `None` value is possible because the variable will refer to nothing.

## Declaration

```
let foo: int = 10;
```
This will declare a variable, with `foo` name, `int` type and a initial value of `10`. but it is not mandatory to set the type of the variable if you initialize with a value that the interpreter know :
```
let foo = 10;
```
And that also declare a variable, with name `foo`, an `int` type and a stored value of `10`.

```
let foo: float;
let bar: int = None;
```
This will declare a `foo` variable of type float and with a `None` value and `bar` is also a variable of type int and with a `None` value but in a more implicit way.

## Change the value

``` 
foo = 20;
```
This will change the stored value inside the previously created foo variable, to 20.

## Use

```
foo * 8
```
This exression will multiply `foo` by 8.

Because a variable can be a primary_expr you can use it every where a primary_expr is needed, you just need to type the variable name identifier : 
```
foo
```

## Syntax
**Declaration :**
```
let $ident(: $ident) = $primary_expr;
```
> Things in parenthesis are sometimes optional, I let you read the `Declaration` documentation for variables.

**Change value :**
```
$ident = $primary_expr;
```

> Please refer to the [grammar](/docs/lang/001-grammar.md)

- **Feature Name:** `functions` 
- **Zom Issue:** Not related to an issue 
- **Status:** `Implemented` since [30d89dc](https://github.com/zom-lang/zom/commit/30d89dc09819236ed75ebd45ae2cab9cd98d41dd)


# Functions

## Declaration

```
func foo(bar: int) int {
    return 32 * 9;
}
```
This will declare a function with `foo` name, `bar` as integer argument and return an int. You must set the type of the variable after the argument identifier

If your function do not return anything, `void` : 
```
func bar(foo: int) void {
    // ...
}
```
but it's not mandatory you can just put nothing, and this will do the same as `void`
```
func bar(foo: int) {
    // ...
}
```

## Function call

```
bar(12);
```
This snippet of code will call the `bar` function with `12` as `foo` argument.

## Syntax
**Declaration :**
```
func $ident( ($ident: $ident ( , ) )* ) $ident {

}
```
> Things in parenthesis are sometimes optional, I let you read the `Declaration` documentation for variables. And things next to a multiply sign (`*`) can be repeated at the infiny but aren't mandatory
Where the first `$ident` is the name of the function.
This `($ident: $ident ( , ) )*` tells that it can have many arguments, the first `$ident` of arguments list is the name identifier of the argument and the second one is type with an `$ident`. 
If there is more than one argument, you need to add an comma between arguments. Like that `... (foo: bar, a: int, b: int) ...`.
And finaly the `$ident` after the arguments is the type that the function return.

**Function Call :**
```
$ident( ($primary_expr ( , ) )* );
```
Function calls are used in `$primary_expr`, so if the function return anything it can be used as a `$primary_expr`.
The first `$ident` is the name identifier of the function.

Then `($primary_expr ( , ) )*`, that function needs `$primary_expr` every time the function needs and the `$primary_expr` will be separated with comma.

> Please refer to the [grammar](/docs/lang/001-grammar.md)

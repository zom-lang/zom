# If / Else

## If statement

An `if` expression allows you to branch your code depending on conditions. You can provide a condition and then state, "If this condition is met, run this block of code. If this condition is not met, do not run this block of code but instead this one if it's true ... or if nothing met run this"

You can state an `if` with the `if` keyword, followed by a condition, and then a block of code to execute like this :

``` 
if *condition* {
  *code*
}
```
`*condition*` refers to a boolean operation and `*code*` refers to the Mona code you want to perform if the condition is true.

e.g.:
```
let foo: int = 14;

if foo == 5472 {
  // code here ...
}
```
## Else if

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
}else if *condition* {
  *code*
}else if *condition* {
  *code*
}else if *condition* {
  *code*
}
```

## Else

If the condition wasn't met in an `if` or an `else if`, you can use `else`. You can state an `else` with the `else` keyword, followed by the block of code to execute, but there can be only one for every `If` statement.

You can state an `else` with the `else` keyword, followed by a condition, and then a block of code to execute like this :

```
if *condition* {
  *code*
}else {
  *code*
}
```

or after a `else if` :

```
if *condition* {
  *code*
}elif *condition* {
  *code*
} else {
  *code*
}
```
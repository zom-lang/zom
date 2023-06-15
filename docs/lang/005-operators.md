- **Feature Name:** `operators` 
- **Zom Issue:** Not related to an issue 
- **Status:** `Implemented` since [30d89dc](https://github.com/zom-lang/zom/commit/30d89dc09819236ed75ebd45ae2cab9cd98d41dd)

# Operators

## Binary operations
A binary operation is an operation that takes to primary expression and perform an operation on it.

### Integers Operation

#### Addition
You can perform addition with the plus character : `+`. e.g.: `a + b`
```
$primary_expr + $primary_expr
```

#### Soustraction
You can perform soustractions with the minus character : `-`.e.g.: `a - b`
```
$primary_expr - $primary_expr
```

#### Multiplication
You can perform multiplication with the times character : `*`. e.g.: `a * b`
```
$primary_expr * $primary_expr
```

#### Division
You can perform division with the divide character : `/`. e.g.: `a / b`
```
$primary_expr / $primary_expr
```

#### Modulo
You can perform modulo with the percent character : `%`. e.g.: `a % b`
```
$primary_expr % $primary_expr
```

#### Power
You can perform power with the circumflex character : `^`. e.g.: `a ^ b`
```
$primary_expr ^ $primary_expr
```

### Boolean Operation

#### And
You can perform `and` with double ampersand character : `&&`. e.g.: `a && b`
```
$primary_expr && $primary_expr
```

#### Or
You can perform `or` with double bar character : `||`. e.g.: `a || b`
```
$primary_expr || $primary_expr
```

## Unary Operations
A unary operation is an operation that takes only one primary expression and perform an operation on it.

### Boolean Operation

#### Not
You can perform `not` with Exclamation mark character : `!`. e.g.: `!a`
```
! $primary_expr
```

## Condition Operation

#### Equal, `==`

The equals operator, of sign `==` is used when you want to check the equality of two objects, and if the objects are equal the operator return true. ex: `a == b`.
```
$primary_expr == $primary_expr
```

#### Different, `!=`

The different operator, of sign `!=` is used when you want to check the non-equality of two objects, and if the objects are not equal the operator return true. e.g.: `a =! b`.
```
$primary_expr != $primary_expr
```

#### Less than, `<`

The `less than` operator, of sign `<` is used when you want to check if the first object is strictly less than the second object, and if the first object is less than the second object, the operator return true. e.g.: `a < b`.
```
$primary_expr < $primary_expr
```

#### Greater than, `>`

The `greater than` operator, of sign `>` is used when you want to check if the first object is strictly greater than the second object, and if the first object is strictly greater than the second object, the operator return true. e.g.: `a > b`.
```
$primary_expr > $primary_expr
```

#### Less than or equal to, `=<`

The `less than or equal to` operator, of sign `=<` is used when you want to check if the first object is less than or equal to the second object, and if the first object is less than or equal to the second object, the operator return true. e.g.: `a =< b`.
```
$primary_expr =< $primary_expr
```

#### Greater than or equal to, `=>`

The `greater than or equal to` operator, of sign `=>` is used when you want to check if the first object is greater than or equal to the second object, and if the first object is gretter than or equal to the second object, the operator return true. e.g.: `a => b`.
```
$primary_expr => $primary_expr
```

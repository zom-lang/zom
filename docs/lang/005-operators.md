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
# Grammar of Mona Programming Language

## Priorities

expr     : term ((Plus|Minus) term)*

term     : factor ((Mul|Div) factor)*

factor   : Int|Float

> `|` means OR and `*` means that the thing before can repeat.
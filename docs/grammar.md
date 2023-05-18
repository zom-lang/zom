# Grammar of Mona Programming Language

## Priorities
```
   expr     -> summand + expr | summand
   summand  -> term * summand | term
   term     -> Int | Float | ( expr )
```

> `|` means OR and `*` means that the thing before can repeat.
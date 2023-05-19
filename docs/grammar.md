# Grammar of Mona Programming Language

## Grammar
The grammar is kinda based of [Recursive Descent Parsing](http://en.wikipedia.org/wiki/Recursive_descent_parser) and [Operator-Precedence Parsing](http://en.wikipedia.org/wiki/Operator-precedence_parser) to produce [the Abstract Syntax Tree](http://en.wikipedia.org/wiki/Abstract_syntax_tree)

```{.ebnf .notation}
   program          : [[statement | expression] Delimiter ? ]*;
   statement        : [declaration | definition];
   declaration      : Extern prototype;
   definition       : Func prototype expression;
   prototype        : Ident OpenParen [Ident Comma ?]* CloseParen;
   expression       : [primary_expr (Op primary_expr)*];
   primary_expr     : [Ident | Number | call_expr | parenthesis_expr];
   call_expr        : Ident OpenParen [expression Comma ?]* CloseParen;
   parenthesis_expr : OpenParen expression CloseParen;
```
> This is written with the the [Extended Backus–Naur form](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form) notation.

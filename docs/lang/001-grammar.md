- **Feature Name:** `grammar` 
- **Zom Issue:** Not related to an issue 
- **Status:** `Implemented` since [8ac7aa7](https://github.com/zom-lang/zom/commit/8ac7aa7d8aa31c3274764db404ef990b527c2908)

# Grammar of Zom Programming Language

## Grammar
The grammar is kinda based of [Recursive Descent Parsing](http://en.wikipedia.org/wiki/Recursive_descent_parser) and [Operator-Precedence Parsing](http://en.wikipedia.org/wiki/Operator-precedence_parser) to produce [the Abstract Syntax Tree](http://en.wikipedia.org/wiki/Abstract_syntax_tree)

```{.ebnf .notation}
   program          : [declaration | definition]*;
   declaration      : Extern prototype;
   definition       : Func prototype block_expr;
   prototype        : $Ident OpenParen [$Ident Comma ?]* CloseParen;
   expression       : [primary_expr (Op primary_expr)*];
   primary_expr     : [$Ident | Number | call_expr | parenthesis_expr | block_expr];
   call_expr        : $Ident OpenParen [expression Comma ?]* CloseParen;
   parenthesis_expr : OpenParen expression CloseParen;
   var_stmt         : var $Ident ( : type )? = expression
   const_stmt       : const $Ident ( : type )? = expression
   statement        : [var_stmt | const_stmt | expression];
   block_expr       : OpenBrace ( statement SemiColon)* (expression)? CloseBrace
   type             : [$ident];
```
> This is written with the the [Extended Backus–Naur form](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form) notation.

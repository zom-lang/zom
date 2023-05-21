use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mona::lexer::Lexer;
use mona::parser::{parse, ParserSettings};
use mona::token::Token::*;

fn simple_lexer_and_parser_benchmark(c: &mut Criterion) {
    c.bench_function("lexer + parser  1 + 1", |b| {
        b.iter(|| {
            let text = black_box(String::from("1 + 1"));
            let mut lexer = Lexer::new(&text, "<benches>".to_string());

            let tokens = lexer
                .make_tokens()
                .expect("An error was occured when benchmarking `simple_lexer_benchmark`.");
            
            let ast = Vec::new();
            let mut parser_settings = ParserSettings::default();
            parse(tokens.as_slice(), ast.as_slice(), &mut parser_settings).expect("Parsing was failed");
        })
    });
}

fn lexer_and_parser_func_benchmark(c: &mut Criterion) {
    c.bench_function(
        "lexer + parser  func(26 args) addition of the 26 args",
        |b| b.iter(|| {
            let text = black_box(String::from("func foo(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z) a + b + c + d + e + f + g + h + i + j + k + l + m + n + o + p + q + r + s + t + u + v + w + x + y + z"));
            let mut lexer = Lexer::new(&text, "<benches>".to_string());

            let tokens = lexer
                .make_tokens()
                .expect("An error was occured when benchmarking `simple_lexer_benchmark`.");
            
            let ast = Vec::new();
            let mut parser_settings = ParserSettings::default();
            parse(tokens.as_slice(), ast.as_slice(), &mut parser_settings).expect("Parsing was failed");
        })
    );
}

fn simple_parser_benchmark(c: &mut Criterion) {
    c.bench_function("parser  1 + 1", |b| {
        b.iter(|| {
            let tokens = black_box(vec![Int(1), Operator("+".to_owned()), Int(1)]);
            
            let ast = Vec::new();
            let mut parser_settings = ParserSettings::default();
            parse(tokens.as_slice(), ast.as_slice(), &mut parser_settings).expect("Parsing was failed");
        })
    });
}

fn parser_func_benchmark(c: &mut Criterion) {
    c.bench_function(
        "parser  func(26 args) addition of the 26 args",
        |b| b.iter(|| {
            let tokens = black_box(vec![
                Func, 
                Ident("foo".to_owned()),
                OpenParen,
                Ident("a".to_owned()),
                Comma,
                Ident("b".to_owned()),
                Comma,
                Ident("c".to_owned()),
                Comma,
                Ident("d".to_owned()),
                Comma,
                Ident("e".to_owned()),
                Comma,
                Ident("f".to_owned()),
                Comma,
                Ident("g".to_owned()),
                Comma,
                Ident("h".to_owned()),
                Comma,
                Ident("i".to_owned()),
                Comma,
                Ident("j".to_owned()),
                Comma,
                Ident("k".to_owned()),
                Comma,
                Ident("l".to_owned()),
                Comma,
                Ident("m".to_owned()),
                Comma,
                Ident("n".to_owned()),
                Comma,
                Ident("o".to_owned()),
                Comma,
                Ident("p".to_owned()),
                Comma,
                Ident("q".to_owned()),
                Comma,
                Ident("r".to_owned()),
                Comma,
                Ident("s".to_owned()),
                Comma,
                Ident("t".to_owned()),
                Comma,
                Ident("u".to_owned()),
                Comma,
                Ident("v".to_owned()),
                Comma,
                Ident("w".to_owned()),
                Comma,
                Ident("x".to_owned()),
                Comma,
                Ident("y".to_owned()),
                Comma,
                Ident("z".to_owned()),
                CloseParen,
                Ident("a".to_owned()),
                Operator("+".to_owned()),
                Ident("b".to_owned()),
                Operator("+".to_owned()),
                Ident("c".to_owned()),
                Operator("+".to_owned()),
                Ident("d".to_owned()),
                Operator("+".to_owned()),
                Ident("e".to_owned()),
                Operator("+".to_owned()),
                Ident("f".to_owned()),
                Operator("+".to_owned()),
                Ident("g".to_owned()),
                Operator("+".to_owned()),
                Ident("h".to_owned()),
                Operator("+".to_owned()),
                Ident("i".to_owned()),
                Operator("+".to_owned()),
                Ident("j".to_owned()),
                Operator("+".to_owned()),
                Ident("k".to_owned()),
                Operator("+".to_owned()),
                Ident("l".to_owned()),
                Operator("+".to_owned()),
                Ident("m".to_owned()),
                Operator("+".to_owned()),
                Ident("n".to_owned()),
                Operator("+".to_owned()),
                Ident("o".to_owned()),
                Operator("+".to_owned()),
                Ident("p".to_owned()),
                Operator("+".to_owned()),
                Ident("q".to_owned()),
                Operator("+".to_owned()),
                Ident("r".to_owned()),
                Operator("+".to_owned()),
                Ident("s".to_owned()),
                Operator("+".to_owned()),
                Ident("t".to_owned()),
                Operator("+".to_owned()),
                Ident("u".to_owned()),
                Operator("+".to_owned()),
                Ident("v".to_owned()),
                Operator("+".to_owned()),
                Ident("w".to_owned()),
                Operator("+".to_owned()),
                Ident("x".to_owned()),
                Operator("+".to_owned()),
                Ident("y".to_owned()),
                Operator("+".to_owned()),
                Ident("z".to_owned()),
                ]);

            let ast = Vec::new();
            let mut parser_settings = ParserSettings::default();
            parse(tokens.as_slice(), ast.as_slice(), &mut parser_settings).expect("Parsing was failed");
        })
    );
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = simple_lexer_and_parser_benchmark, lexer_and_parser_func_benchmark, simple_parser_benchmark, parser_func_benchmark
}
criterion_main!(benches);

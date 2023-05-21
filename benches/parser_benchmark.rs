use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mona::lexer::Lexer;
use mona::parser::{parse, ParserSettings};

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

fn lexxer_and_parser_func_benchmark(c: &mut Criterion) {
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

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = simple_lexer_and_parser_benchmark, lexxer_and_parser_func_benchmark
}
criterion_main!(benches);

use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zom_fe::lexer::Lexer;

fn simple_lexer_benchmark(c: &mut Criterion) {
    c.bench_function("lexer 1 + 1", |b| {
        b.iter(|| {
            let text = black_box("1 + 1\n");
            let mut lexer = Lexer::new(text, Path::new("<benches>.zom"));
            lexer
                .lex()
                .expect("An error was occured when benchmarking `simple_lexer_benchmark`.");
        })
    });
}

fn lex_func_benchmark(c: &mut Criterion) {
    c.bench_function(
        "lexer func(26 args) addition of the 26 args",
        |b| b.iter(|| {
            let text = black_box("func foo(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z) a + b + c + d + e + f + g + h + i + j + k + l + m + n + o + p + q + r + s + t + u + v + w + x + y + z\n");
            let mut lexer = Lexer::new(text, Path::new("<benches>.zom"));
            lexer.lex().expect("An error was occured when benchmarking `simple_lexer_benchmark`.");
        })
    );
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = simple_lexer_benchmark, lex_func_benchmark
}
criterion_main!(benches);

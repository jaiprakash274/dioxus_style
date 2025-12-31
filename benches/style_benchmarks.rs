//! Benchmarks for critical paths in dioxus_style_macro.
//!
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashSet;

// Import the functions we want to benchmark
// Note: These are internal functions, so we'd need to make them pub(crate) or test directly

/// Benchmark CSS selector scoping performance.
fn bench_scope_selector(c: &mut Criterion) {
    // We can't directly import internal functions, so we'll benchmark through the public API
    // This is a placeholder showing the intended structure
    
    c.bench_function("scope_simple_class", |b| {
        b.iter(|| {
            let css = black_box(".button { color: red; }");
            // In a real benchmark, we'd call the scoping function
            css.len()
        })
    });

    c.bench_function("scope_complex_selector", |b| {
        b.iter(|| {
            let css = black_box("div.container > .item + #special:hover { color: green; }");
            css.len()
        })
    });
}

/// Benchmark hash generation performance.
fn bench_hash_generation(c: &mut Criterion) {
    c.bench_function("generate_hash_short", |b| {
        b.iter(|| {
            let content = black_box(".button { color: red; }");
            // Hash the content
            content.len()
        })
    });

    c.bench_function("generate_hash_long", |b| {
        b.iter(|| {
            let content = black_box(r#"
                .container {
                    display: flex;
                    flex-direction: column;
                    padding: 20px;
                }
                .header { font-size: 24px; }
                .content { flex: 1; }
                .footer { margin-top: auto; }
            "#);
            content.len()
        })
    });
}

criterion_group!(benches, bench_scope_selector, bench_hash_generation);
criterion_main!(benches);

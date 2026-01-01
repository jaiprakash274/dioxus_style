//! Benchmarks for critical paths in dioxus_style.
//!
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use xxhash_rust::xxh3::xxh3_64;

/// Simple CSS samples for benchmarking
const SIMPLE_CSS: &str = ".button { color: red; padding: 10px; }";
const MEDIUM_CSS: &str = r#"
.container {
    display: flex;
    flex-direction: column;
    padding: 20px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}
.header { 
    font-size: 24px; 
    font-weight: bold;
    color: white;
}
.content { 
    flex: 1; 
    padding: 16px;
}
.footer { 
    margin-top: auto; 
    text-align: center;
}
"#;

const COMPLEX_CSS: &str = r#"
.dashboard-container {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 20px;
    padding: 40px;
    background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
    min-height: 100vh;
}
.card {
    background: rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(10px);
    border-radius: 16px;
    padding: 24px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
    border: 1px solid rgba(255, 255, 255, 0.1);
    transition: transform 0.3s ease, box-shadow 0.3s ease;
}
.card:hover {
    transform: translateY(-4px);
    box-shadow: 0 12px 48px rgba(0, 0, 0, 0.3);
}
.card-title {
    font-size: 1.25rem;
    font-weight: 600;
    color: white;
    margin-bottom: 12px;
}
.card-value {
    font-size: 2.5rem;
    font-weight: 700;
    background: linear-gradient(135deg, #00c6fb, #005bea);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
}
.btn-primary {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    padding: 12px 24px;
    border: none;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
}
.btn-primary:hover {
    transform: scale(1.05);
    box-shadow: 0 4px 20px rgba(102, 126, 234, 0.4);
}
"#;

/// Benchmark hash generation performance using XXH3.
fn bench_hash_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Hash Generation");
    
    group.bench_with_input(BenchmarkId::new("simple", "23 bytes"), &SIMPLE_CSS, |b, css| {
        b.iter(|| {
            xxh3_64(black_box(css.as_bytes()))
        })
    });

    group.bench_with_input(BenchmarkId::new("medium", "400 bytes"), &MEDIUM_CSS, |b, css| {
        b.iter(|| {
            xxh3_64(black_box(css.as_bytes()))
        })
    });

    group.bench_with_input(BenchmarkId::new("complex", "1.5KB"), &COMPLEX_CSS, |b, css| {
        b.iter(|| {
            xxh3_64(black_box(css.as_bytes()))
        })
    });
    
    group.finish();
}

/// Benchmark base62 encoding performance.
fn bench_base62_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("Base62 Encoding");
    
    let test_values = [
        (0u64, "zero"),
        (61, "single_char"),
        (123456789, "medium"),
        (u64::MAX, "max"),
    ];
    
    for (val, name) in test_values.iter() {
        group.bench_with_input(BenchmarkId::new("encode", name), val, |b, &num| {
            b.iter(|| {
                encode_base62(black_box(num))
            })
        });
    }
    
    group.finish();
}

/// Benchmark CSS selector parsing (simulated).
fn bench_css_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("CSS Parsing");
    
    // Simple class extraction
    group.bench_function("extract_class_names", |b| {
        b.iter(|| {
            let css = black_box(COMPLEX_CSS);
            extract_class_names(css)
        })
    });
    
    // Selector counting
    group.bench_function("count_selectors", |b| {
        b.iter(|| {
            let css = black_box(COMPLEX_CSS);
            css.matches('{').count()
        })
    });
    
    group.finish();
}

/// Benchmark string operations commonly used in scoping.
fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("String Operations");
    
    let scope = "sc_a1b2c3";
    let class_name = "button-primary";
    
    // Format-based class name generation
    group.bench_function("format_class_name", |b| {
        b.iter(|| {
            format!(".{}_{}", black_box(scope), black_box(class_name))
        })
    });
    
    // Push-based class name generation
    group.bench_function("push_class_name", |b| {
        b.iter(|| {
            let mut result = String::with_capacity(scope.len() + class_name.len() + 2);
            result.push('.');
            result.push_str(black_box(scope));
            result.push('_');
            result.push_str(black_box(class_name));
            result
        })
    });
    
    group.finish();
}

// Helper function: Base62 encoding
fn encode_base62(mut num: u64) -> String {
    const BASE62_CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    
    if num == 0 {
        return String::from("0");
    }
    
    let mut result = Vec::with_capacity(11);
    while num > 0 {
        result.push(BASE62_CHARS[(num % 62) as usize]);
        num /= 62;
    }
    result.reverse();
    unsafe { String::from_utf8_unchecked(result) }
}

// Helper function: Extract class names from CSS
fn extract_class_names(css: &str) -> Vec<&str> {
    let mut classes = Vec::new();
    let mut chars = css.chars().peekable();
    let bytes = css.as_bytes();
    let mut i = 0;
    
    while i < bytes.len() {
        if bytes[i] == b'.' {
            let start = i + 1;
            let mut end = start;
            while end < bytes.len() && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'-' || bytes[end] == b'_') {
                end += 1;
            }
            if end > start {
                classes.push(&css[start..end]);
            }
            i = end;
        } else {
            i += 1;
        }
    }
    classes
}

criterion_group!(
    benches,
    bench_hash_generation,
    bench_base62_encoding,
    bench_css_parsing,
    bench_string_operations
);
criterion_main!(benches);

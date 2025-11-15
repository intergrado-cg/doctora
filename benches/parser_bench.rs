//! Comprehensive benchmarks comparing Logos+Chumsky vs Logos+Winnow
//!
//! This benchmark suite measures the performance of two parser combinations:
//! - Logos 0.15 + Chumsky 0.11
//! - Logos 0.15 + Winnow 0.7
//!
//! Both parsers produce the same AST from the same token stream.
//!
//! Test inputs vary in size:
//! - Small: ~100 bytes (simple doc with 1 heading, 1 paragraph)
//! - Medium: ~1KB (multiple sections with formatted text)
//! - Large: ~10KB (complex nested document)
//!
//! Metrics measured:
//! - Parse time (mean, median, std dev)
//! - Throughput (MB/s)
//! - Statistical analysis via Criterion

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use doctora::parser_winnow::parse_document_winnow;
use doctora::token::Token;
use logos::Logos;

// ============================================================================
// Test Input Generation
// ============================================================================

/// Small document: ~100 bytes
fn small_document() -> &'static str {
    r#"= Document Title

This is a paragraph with **bold** and _italic_ text."#
}

/// Medium document: ~1KB
fn medium_document() -> &'static str {
    r#"= Main Document Title

This is the introduction paragraph with some **bold text** and _italic text_.
It spans multiple lines to make it more realistic.

== First Section

This section contains several paragraphs with various formatting options.
We have **bold**, _italic_, and even **bold with _nested italic_** text.

Another paragraph in this section with more content to increase the size.
The parser needs to handle inline formatting correctly across paragraph boundaries.

== Second Section

=== Subsection A

Here we have nested sections to test the parser's ability to handle
hierarchical document structures. This is important for AsciiDoc.

More text with **formatting** to make it interesting.

=== Subsection B

Another subsection with different content. The parser must maintain
proper nesting levels and track section hierarchy correctly.

== Third Section

This section has multiple paragraphs to increase document complexity.

First paragraph with **bold** text.

Second paragraph with _italic_ text.

Third paragraph with **bold _and nested_ formatting**."#
}

/// Large document: ~10KB
fn large_document() -> &'static str {
    r#"= Comprehensive AsciiDoc Document

This is a large, complex document designed to stress-test the parser.
It contains multiple levels of nesting, extensive formatting, and realistic content.

== Introduction

AsciiDoc is a **lightweight markup language** designed for writing documentation.
It provides a _rich set of features_ while maintaining **readable plain text** syntax.

This document demonstrates various AsciiDoc features including:

Text with **bold formatting** and _italic formatting_.
Nested formatting like **bold with _italic inside_** works correctly.

== First Major Section

=== Background and Motivation

The need for a **high-performance AsciiDoc parser** in Rust stems from several requirements:

Performance is crucial for processing large documentation sets.
We need **low latency** and _high throughput_ to handle real-world workloads.

The parser must be **robust** and handle malformed input gracefully.
Error recovery is important for providing **useful feedback** to users.

=== Design Principles

==== Modularity

The architecture separates **parsing from processing**, enabling:

Pluggable output formats without touching the parser.
Independent testing and **optimization** of each component.

==== Performance

We aim for **exceptional performance** through:

Zero-copy parsing where possible.
Efficient **memory allocation** strategies.

==== Correctness

The parser must produce **correct AST** representations that:

Preserve document structure and _semantic meaning_.
Handle edge cases and **malformed input** properly.

== Second Major Section

=== Implementation Details

==== Lexical Analysis

The **Logos library** provides fast, efficient lexing with:

Compile-time regex compilation for **maximum performance**.
Built-in _span tracking_ for error reporting.

Performance characteristics of Logos include:

**Over 1,200 MB/s** throughput on typical workloads.
_Low memory overhead_ due to streaming design.

==== Syntactic Analysis

We compare **two parser combinator libraries**:

Chumsky provides **elegant error recovery** and composable parsers.
Winnow offers **high performance** with zero-copy parsing.

==== Abstract Syntax Tree

The AST design focuses on:

**Minimal memory footprint** while preserving all information.
_Type safety_ through Rust's strong type system.

== Third Major Section

=== Performance Benchmarks

==== Methodology

Our benchmarking approach uses:

**Criterion.rs** for statistical analysis and reliable measurements.
_Multiple input sizes_ (small, medium, large) to test scalability.

==== Results

Initial benchmarks show:

Logos achieves **1,200+ MB/s** lexing throughput.
Parser performance varies by _combinator library_ choice.

==== Analysis

The results indicate:

**Lexing is extremely fast** and not a bottleneck.
Parser efficiency depends on _combinator design_ and error handling.

== Fourth Major Section

=== Future Enhancements

==== Additional Features

Planned enhancements include:

**Table parsing** for structured data representation.
_List support_ for bulleted and numbered lists.

==== Optimization Opportunities

Performance improvements could come from:

**Arena allocation** for AST nodes to reduce allocator pressure.
_Parallel parsing_ of independent document sections.

==== Error Reporting

Better error messages through:

**Rich diagnostics** with source snippets and suggestions.
_Multiple error collection_ instead of fail-fast behavior.

== Conclusion

This document has demonstrated:

**Complex nesting** of sections up to 4 levels deep.
_Extensive inline formatting_ including nested combinations.

The parser must handle all these cases **efficiently** and _correctly_.

Final thoughts on **parser design**:

Choose the right tools for **performance and correctness**.
Test thoroughly with _realistic input_ data.

=== Acknowledgments

Thanks to the **Rust community** for excellent libraries and tools.
Special recognition for _Logos, Chumsky, and Winnow_ developers.

=== References

See the **Rust API Guidelines** for idiomatic design patterns.
Consult _AsciiDoc specification_ for language details.

== Appendix

=== Technical Specifications

==== Performance Targets

Target metrics include:

**Lexing**: > 1,000 MB/s throughput.
**Parsing**: > 500 MB/s throughput.

==== Memory Usage

Acceptable memory characteristics:

**Peak allocation**: < 2x input size.
_Steady state_: < 1.5x input size.

=== Testing Strategy

==== Unit Tests

Cover all parser components:

**Inline formatting** with all combinations.
_Block structures_ including sections and paragraphs.

==== Integration Tests

End-to-end validation:

**Complete documents** parsed correctly.
_Error cases_ handled gracefully.

This completes our large test document with **extensive content** and _varied formatting_."#
}

// ============================================================================
// Lexing Helper (shared by both parsers)
// ============================================================================

fn lex_tokens(input: &str) -> Vec<Token> {
    Token::lexer(input)
        .filter_map(|result| result.ok())
        .collect()
}

// ============================================================================
// Chumsky Parser Benchmarks
// ============================================================================

fn bench_chumsky_small(c: &mut Criterion) {
    let input = small_document();
    let input_bytes = input.len();

    let mut group = c.benchmark_group("chumsky_small");
    group.throughput(Throughput::Bytes(input_bytes as u64));

    group.bench_function("parse", |b| {
        let tokens = lex_tokens(input);
        b.iter(|| {
            use chumsky::prelude::*;
            let result = doctora::parser::document()
                .parse(std::hint::black_box(tokens.as_slice()))
                .into_result();
            std::hint::black_box(result)
        });
    });

    group.finish();
}

fn bench_chumsky_medium(c: &mut Criterion) {
    let input = medium_document();
    let input_bytes = input.len();

    let mut group = c.benchmark_group("chumsky_medium");
    group.throughput(Throughput::Bytes(input_bytes as u64));

    group.bench_function("parse", |b| {
        let tokens = lex_tokens(input);
        b.iter(|| {
            use chumsky::prelude::*;
            let result = doctora::parser::document()
                .parse(std::hint::black_box(tokens.as_slice()))
                .into_result();
            std::hint::black_box(result)
        });
    });

    group.finish();
}

fn bench_chumsky_large(c: &mut Criterion) {
    let input = large_document();
    let input_bytes = input.len();

    let mut group = c.benchmark_group("chumsky_large");
    group.throughput(Throughput::Bytes(input_bytes as u64));

    group.bench_function("parse", |b| {
        let tokens = lex_tokens(input);
        b.iter(|| {
            use chumsky::prelude::*;
            let result = doctora::parser::document()
                .parse(std::hint::black_box(tokens.as_slice()))
                .into_result();
            std::hint::black_box(result)
        });
    });

    group.finish();
}

// ============================================================================
// Winnow Parser Benchmarks
// ============================================================================

fn bench_winnow_small(c: &mut Criterion) {
    let input = small_document();
    let input_bytes = input.len();

    let mut group = c.benchmark_group("winnow_small");
    group.throughput(Throughput::Bytes(input_bytes as u64));

    group.bench_function("parse", |b| {
        let tokens = lex_tokens(input);
        b.iter(|| {
            let result = parse_document_winnow(std::hint::black_box(&tokens));
            std::hint::black_box(result)
        });
    });

    group.finish();
}

fn bench_winnow_medium(c: &mut Criterion) {
    let input = medium_document();
    let input_bytes = input.len();

    let mut group = c.benchmark_group("winnow_medium");
    group.throughput(Throughput::Bytes(input_bytes as u64));

    group.bench_function("parse", |b| {
        let tokens = lex_tokens(input);
        b.iter(|| {
            let result = parse_document_winnow(std::hint::black_box(&tokens));
            std::hint::black_box(result)
        });
    });

    group.finish();
}

fn bench_winnow_large(c: &mut Criterion) {
    let input = large_document();
    let input_bytes = input.len();

    let mut group = c.benchmark_group("winnow_large");
    group.throughput(Throughput::Bytes(input_bytes as u64));

    group.bench_function("parse", |b| {
        let tokens = lex_tokens(input);
        b.iter(|| {
            let result = parse_document_winnow(std::hint::black_box(&tokens));
            std::hint::black_box(result)
        });
    });

    group.finish();
}

// ============================================================================
// Comparison Benchmarks (both parsers on same input)
// ============================================================================

fn bench_comparison(c: &mut Criterion) {
    let inputs = [
        ("small", small_document()),
        ("medium", medium_document()),
        ("large", large_document()),
    ];

    for (name, input) in inputs.iter() {
        let mut group = c.benchmark_group(format!("comparison_{}", name));
        group.throughput(Throughput::Bytes(input.len() as u64));

        let tokens = lex_tokens(input);

        group.bench_with_input(BenchmarkId::new("chumsky", name), &tokens, |b, tokens| {
            b.iter(|| {
                use chumsky::prelude::*;
                let result = doctora::parser::document()
                    .parse(std::hint::black_box(tokens.as_slice()))
                    .into_result();
                std::hint::black_box(result)
            });
        });

        group.bench_with_input(BenchmarkId::new("winnow", name), &tokens, |b, tokens| {
            b.iter(|| {
                let result = parse_document_winnow(std::hint::black_box(tokens));
                std::hint::black_box(result)
            });
        });

        group.finish();
    }
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group!(
    benches,
    bench_chumsky_small,
    bench_chumsky_medium,
    bench_chumsky_large,
    bench_winnow_small,
    bench_winnow_medium,
    bench_winnow_large,
    bench_comparison
);

criterion_main!(benches);

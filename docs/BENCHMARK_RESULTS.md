# Parser Benchmark Results: Chumsky vs Winnow

**Date**: 2025-11-15
**Task**: Phase 1, Task 6 - Benchmark Logos+Chumsky vs Logos+Winnow
**Purpose**: Validate parsing library choice for doctora AsciiDoc parser (ADR-004)

## Executive Summary

**Winner: Winnow** - Winnow consistently outperforms Chumsky by 15-45% across all input sizes, with the largest gains on small inputs. Winnow is the recommended choice for the parser layer.

## Performance Comparison

### Small Document (~100 bytes)

| Parser | Mean Time | Throughput | Performance |
|--------|-----------|------------|-------------|
| **Winnow** | **759.8 ns** | **87.86 MiB/s** | **Baseline** |
| Chumsky | 1,011.7 ns | 65.99 MiB/s | 33% slower |

**Winner**: Winnow is **33% faster** on small inputs

### Medium Document (~1 KB)

| Parser | Mean Time | Throughput | Performance |
|--------|-----------|------------|-------------|
| **Winnow** | **9.08 µs** | **112.44 MiB/s** | **Baseline** |
| Chumsky | 13.17 µs | 77.56 MiB/s | 45% slower |

**Winner**: Winnow is **45% faster** on medium inputs

### Large Document (~10 KB)

| Parser | Mean Time | Throughput | Performance |
|--------|-----------|------------|-------------|
| **Winnow** | **46.88 µs** | **97.55 MiB/s** | **Baseline** |
| Chumsky | 54.19 µs | 84.39 MiB/s | 16% slower |

**Winner**: Winnow is **16% faster** on large inputs

## Detailed Benchmark Results

### Chumsky Parser (Logos + Chumsky 0.11)

```
Small:   1.012 µs  (65.99 MiB/s)
Medium: 13.169 µs  (77.56 MiB/s)
Large:  54.187 µs  (84.39 MiB/s)
```

### Winnow Parser (Logos + Winnow 0.7)

```
Small:    759.8 ns  (87.86 MiB/s)
Medium:   9.084 µs  (112.44 MiB/s)
Large:   46.879 µs  (97.55 MiB/s)
```

## Performance Scaling Analysis

### Throughput by Input Size

| Input Size | Chumsky (MiB/s) | Winnow (MiB/s) | Winnow Advantage |
|------------|-----------------|----------------|------------------|
| Small (~100B) | 65.99 | 87.86 | +33% |
| Medium (~1KB) | 77.56 | 112.44 | +45% |
| Large (~10KB) | 84.39 | 97.55 | +16% |

**Observation**: Both parsers show improved throughput on larger inputs (better amortization of overhead), but Winnow maintains a consistent advantage.

## API Ergonomics Comparison

### Implementation Complexity

Both parsers required similar implementation effort:

**Chumsky (0.11)**:
- Elegant recursive combinator API
- Built-in error recovery with `Recoverable`
- Clean type inference
- Good error messages from compiler
- More compact code

**Winnow (0.7)**:
- Explicit type annotations required (`winnow::Result`)
- Manual error handling with `ErrMode`
- Slightly more verbose (type annotations)
- Excellent runtime performance
- Zero-copy parsing capabilities

### Code Maintainability

**Chumsky**: More readable, easier for contributors unfamiliar with parser combinators
**Winnow**: Slightly more verbose but predictable performance characteristics

## Error Recovery & Reporting

**Chumsky**: Built-in error recovery, can report multiple errors in one pass
**Winnow**: Focuses on performance, error handling is more manual but flexible

For the POC, both provided adequate error handling. Full error recovery testing was not performed.

## Memory Allocation

Memory profiling was not performed in this benchmark. Both libraries operate on the same `Vec<Token>` from Logos, so allocation patterns are similar. Winnow may have a slight edge due to zero-copy design, but this was not measured.

## Surprises & Key Findings

1. **Winnow's advantage increases on medium-sized inputs**: The 45% improvement on ~1KB documents suggests Winnow's zero-copy approach shines at this scale.

2. **Both parsers scale well**: Throughput improves with larger inputs for both libraries, showing good overhead amortization.

3. **Implementation effort was comparable**: Despite Winnow requiring more type annotations, the overall code structure was similar.

4. **Logos is not the bottleneck**: Since both parsers consume the same pre-lexed token stream, parsing is the differentiator.

## Recommendation

**Adopt Winnow (0.7) for the core parser**

### Rationale

1. **Performance**: 15-45% faster across all input sizes
2. **Predictability**: Consistent performance advantage
3. **Scalability**: Zero-copy design will benefit larger documents
4. **Community**: Winnow is actively maintained (fork of nom with improvements)
5. **Future-proof**: Performance headroom for additional features

### Trade-offs Accepted

- Slightly more verbose code (explicit type annotations)
- Manual error recovery (acceptable for Phase 1)
- Learning curve for contributors (mitigated by documentation)

## Next Steps

1. **Update ADR-004** in `docs/design/architecture.md` with Winnow decision
2. **Adopt Winnow parser** as the primary implementation
3. **Remove Chumsky** implementation and dependency (keep for reference in git history)
4. **Document Winnow patterns** for future contributors
5. **Consider error recovery**: Implement custom error recovery on top of Winnow

## Benchmark Environment

- **CPU**: (WSL2 on Windows)
- **Rust**: 1.85.0-stable
- **Criterion**: 0.7
- **Logos**: 0.15
- **Chumsky**: 0.11
- **Winnow**: 0.7
- **Profile**: Release mode (optimized)
- **Iterations**: 100 samples per benchmark
- **Statistical analysis**: Mean + confidence intervals

## Appendix: Raw Benchmark Output

See `target/criterion` directory for full Criterion reports including:
- HTML reports with visualizations
- Statistical analysis (mean, median, std dev)
- Outlier detection
- Regression analysis

## References

- **ADR-004**: Parsing library selection (docs/design/architecture.md)
- **Winnow**: https://docs.rs/winnow/
- **Chumsky**: https://docs.rs/chumsky/
- **Criterion**: https://docs.rs/criterion/
- **Logos**: https://docs.rs/logos/

# Markdown Parser Benchmarks

This directory contains performance benchmarks comparing our `md2html` parser with popular Rust Markdown parsers.

## Structure

```
benchmark/
├── README.md           # This file
├── results.md          # Benchmark results and analysis
└── samples/            # Test documents
    ├── small.md        # ~1KB document
    ├── medium.md       # ~5KB document
    └── large.md        # ~25KB document
```

## Running Benchmarks

To run the benchmarks:

```bash
cargo bench
```

This will:
1. Compile all parsers with optimizations
2. Run benchmarks using Criterion.rs
3. Generate detailed reports in `target/criterion/`

## Parsers Compared

1. **md2html** - Our implementation
   - Simple, dependency-free parser
   - Supports basic Markdown features
   - Educational focus

2. **pulldown-cmark** - Industry standard
   - Event-based parser
   - Highly optimized
   - CommonMark compliant

3. **comrak** - GitHub's parser
   - Full GFM support
   - Most features
   - Reference implementation

## Key Findings

- **Performance**: pulldown-cmark is 1.7-2.4x faster than md2html
- **Simplicity**: md2html has no dependencies and simpler code
- **Features**: comrak supports the most features but is slowest

See [results.md](./results.md) for detailed analysis.

## Sample Documents

The benchmark uses three document sizes:
- **Small**: Basic features, quick parsing
- **Medium**: Mixed content, realistic blog post
- **Large**: Stress test with extensive content

## Interpreting Results

- **Time**: Lower is better (measured in microseconds/milliseconds)
- **Outliers**: Occasional slow runs, usually from system interrupts
- **Scaling**: How performance changes with document size

## Future Improvements

Potential optimizations for md2html:
1. Event-based parsing like pulldown-cmark
2. SIMD optimizations for pattern matching
3. Memory pool for string allocations
4. Lazy evaluation of inline elements
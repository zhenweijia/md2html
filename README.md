# md2html - Fast Markdown to HTML Parser

A high-performance, zero-dependency Markdown to HTML parser written in Rust. This implementation demonstrates how careful optimization can achieve competitive performance while maintaining code simplicity.

## 🚀 Performance

**Outperforms industry-standard parsers:**

- **1.6x faster** than pulldown-cmark
- **5.0x faster** than comrak
- **77-82% improvement** over initial implementation

| Document Size | md2html | pulldown-cmark | comrak   |
| ------------- | ------- | -------------- | -------- |
| Small (1KB)   | 1.10 µs | 1.72 µs        | 5.55 µs  |
| Medium (5KB)  | 8.37 µs | 12.87 µs       | 43.80 µs |
| Large (25KB)  | 1.07 ms | 1.95 ms        | 6.19 ms  |

## ✨ Features

- **Headers** (H1-H6): `# Header 1`, `## Header 2`, etc.
- **Text formatting**: `**bold**`, `*italic*`, `_italic_`
- **Inline code**: `code`
- **Code blocks**: Triple backticks
- **Links**: `[text](url)`
- **Lists**: Unordered (`-`, `*`, `+`) and ordered (`1.`, `2.`, etc.)
- **Paragraphs**: Automatic paragraph detection
- **HTML escaping**: Safe output with proper character escaping

## 🛠 Installation

Clone the repository:

```bash
git clone <repository-url>
cd md2html
cargo build --release
```

## 📖 Usage

### Command Line

```bash
# Convert file to file
cargo run -- input.md output.html

# Convert file to stdout
cargo run -- input.md

# Read from stdin, write to stdout
echo "# Hello World" | cargo run
```

### As a Library

```rust
use md2html::MarkdownParser;

fn main() {
    let markdown = "# Hello World\n\nThis is **bold** text.";
    let parser = MarkdownParser::new(markdown);
    let html = parser.parse();
    println!("{}", html);
}
```

Output:

```html
<h1>Hello World</h1>
<p>This is <strong>bold</strong> text.</p>
```

## 🎯 Examples

### Input Markdown

````markdown
# My Blog Post

This is a paragraph with **bold** and _italic_ text.

## Features

- Fast parsing
- Zero dependencies
- Simple API

### Code Example

```rust
fn main() {
    println!("Hello, world!");
}
```
````

Visit [Rust](https://rust-lang.org) for more info.

````markdown

### Output HTML
```html
<h1>My Blog Post</h1>
<p>This is a paragraph with <strong>bold</strong> and <em>italic</em> text.</p>
<h2>Features</h2>
<ul>
  <li>Fast parsing</li>
  <li>Zero dependencies</li>
  <li>Simple API</li>
</ul>
<h3>Code Example</h3>
<pre><code>fn main() {
    println!(&quot;Hello, world!&quot;);
}</code></pre>
<p>Visit <a href="https://rust-lang.org">Rust</a> for more info.</p>
````

## ⚡ Performance Optimizations

The parser achieves its speed through several key optimizations:

### 1. Zero-Copy Processing

- Uses `&str` instead of `String` for input
- Direct string slice operations
- Eliminates unnecessary allocations

### 2. Byte-Level Parsing

- Pattern matching on bytes instead of chars
- Direct byte comparisons for delimiters
- Optimized ASCII character detection

### 3. Pre-allocated Buffers

- Output buffers sized with capacity hints
- Reduced dynamic memory allocations
- Single-pass HTML generation

### 4. Inline Functions

- Critical path functions marked `#[inline]`
- Reduced function call overhead
- Better compiler optimizations

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

## 📊 Benchmarks

Run performance benchmarks:

```bash
cargo bench
```

This compares md2html against:

- **pulldown-cmark**: Industry standard Rust parser
- **comrak**: GitHub-flavored Markdown parser

See [`benchmark/`](./benchmark/) for detailed results and analysis.

## 🏗 Architecture

### Core Components

- **`MarkdownParser`**: Main parser struct with zero-copy design
- **Block parsing**: Headers, code blocks, lists, paragraphs
- **Inline parsing**: Bold, italic, code, links with single-pass processing
- **HTML generation**: Streaming output with optimized escaping

### Design Principles

1. **Performance**: Optimized for speed without sacrificing readability
2. **Simplicity**: Straightforward implementation, easy to understand
3. **Zero dependencies**: No external crates required
4. **Safety**: Proper HTML escaping prevents XSS vulnerabilities

## 🔬 Technical Details

### Memory Usage

- **Lower allocations**: Pre-sized buffers reduce allocation count
- **Better cache locality**: Sequential memory access patterns
- **String slices**: Avoid copying input data

### Parsing Strategy

- **Single-pass**: Combined parsing and HTML generation
- **Byte-oriented**: Direct byte operations for common patterns
- **Streaming**: Output generated progressively

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes
4. Run tests: `cargo test`
5. Run benchmarks: `cargo bench`
6. Submit a pull request

## 📝 License

This project is open source. See LICENSE file for details.

## 🔗 Related Projects

- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark): Production Rust parser
- [comrak](https://github.com/kivikakk/comrak): GitHub-flavored Markdown parser
- [CommonMark](https://commonmark.org/): Markdown specification

---

**Note**: This parser prioritizes performance and simplicity over feature completeness. For production use requiring full CommonMark compliance, consider pulldown-cmark.

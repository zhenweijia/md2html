use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use md2html::MarkdownParser;
use pulldown_cmark::{Parser, Options, html};
use comrak::{markdown_to_html, ComrakOptions};

fn create_small_markdown() -> String {
    r#"# Hello World

This is a **test** document with *italic* text and `inline code`.

## Features

- Item 1
- Item 2
- Item 3

1. First
2. Second
3. Third

Check out [GitHub](https://github.com) for more.

```rust
fn main() {
    println!("Hello, world!");
}
```
"#.to_string()
}

fn create_medium_markdown() -> String {
    let mut content = String::new();
    
    for i in 0..10 {
        content.push_str(&format!("# Section {}\n\n", i));
        content.push_str("This is a paragraph with **bold** and *italic* text. ");
        content.push_str("It contains `inline code` and [links](https://example.com).\n\n");
        
        content.push_str("## Subsection\n\n");
        for j in 0..5 {
            content.push_str(&format!("- List item {}\n", j));
        }
        content.push('\n');
        
        content.push_str("```rust\n");
        content.push_str("fn example() {\n");
        content.push_str("    println!(\"Example code\");\n");
        content.push_str("}\n");
        content.push_str("```\n\n");
    }
    
    content
}

fn create_large_markdown() -> String {
    let mut content = String::new();
    
    for i in 0..100 {
        content.push_str(&format!("# Chapter {}\n\n", i));
        
        for j in 0..10 {
            content.push_str(&format!("## Section {}.{}\n\n", i, j));
            content.push_str("This is a long paragraph with various **formatting** options. ");
            content.push_str("It includes *italics*, `code`, and [multiple](https://example1.com) ");
            content.push_str("[different](https://example2.com) [links](https://example3.com). ");
            content.push_str("The paragraph continues with more text to simulate real-world content.\n\n");
            
            if j % 2 == 0 {
                content.push_str("### Unordered List\n\n");
                for k in 0..8 {
                    content.push_str(&format!("- Item {}\n", k));
                }
            } else {
                content.push_str("### Ordered List\n\n");
                for k in 0..8 {
                    content.push_str(&format!("{}. Item {}\n", k + 1, k));
                }
            }
            content.push('\n');
        }
        
        content.push_str("```rust\n");
        content.push_str("// Example code block\n");
        content.push_str("fn complex_function() {\n");
        content.push_str("    let data = vec![1, 2, 3, 4, 5];\n");
        content.push_str("    for item in data {\n");
        content.push_str("        println!(\"Processing: {}\", item);\n");
        content.push_str("    }\n");
        content.push_str("}\n");
        content.push_str("```\n\n");
    }
    
    content
}

fn bench_md2html(c: &mut Criterion) {
    let small = create_small_markdown();
    let medium = create_medium_markdown();
    let large = create_large_markdown();
    
    c.bench_with_input(
        BenchmarkId::new("md2html", "small"),
        &small,
        |b, input| {
            b.iter(|| {
                let mut parser = MarkdownParser::new(input.clone());
                black_box(parser.parse())
            });
        },
    );
    
    c.bench_with_input(
        BenchmarkId::new("md2html", "medium"),
        &medium,
        |b, input| {
            b.iter(|| {
                let mut parser = MarkdownParser::new(input.clone());
                black_box(parser.parse())
            });
        },
    );
    
    c.bench_with_input(
        BenchmarkId::new("md2html", "large"),
        &large,
        |b, input| {
            b.iter(|| {
                let mut parser = MarkdownParser::new(input.clone());
                black_box(parser.parse())
            });
        },
    );
}

fn bench_pulldown_cmark(c: &mut Criterion) {
    let small = create_small_markdown();
    let medium = create_medium_markdown();
    let large = create_large_markdown();
    
    c.bench_with_input(
        BenchmarkId::new("pulldown-cmark", "small"),
        &small,
        |b, input| {
            b.iter(|| {
                let parser = Parser::new_ext(input, Options::all());
                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);
                black_box(html_output)
            });
        },
    );
    
    c.bench_with_input(
        BenchmarkId::new("pulldown-cmark", "medium"),
        &medium,
        |b, input| {
            b.iter(|| {
                let parser = Parser::new_ext(input, Options::all());
                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);
                black_box(html_output)
            });
        },
    );
    
    c.bench_with_input(
        BenchmarkId::new("pulldown-cmark", "large"),
        &large,
        |b, input| {
            b.iter(|| {
                let parser = Parser::new_ext(input, Options::all());
                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);
                black_box(html_output)
            });
        },
    );
}

fn bench_comrak(c: &mut Criterion) {
    let small = create_small_markdown();
    let medium = create_medium_markdown();
    let large = create_large_markdown();
    
    let options = ComrakOptions::default();
    
    c.bench_with_input(
        BenchmarkId::new("comrak", "small"),
        &small,
        |b, input| {
            b.iter(|| {
                black_box(markdown_to_html(input, &options))
            });
        },
    );
    
    c.bench_with_input(
        BenchmarkId::new("comrak", "medium"),
        &medium,
        |b, input| {
            b.iter(|| {
                black_box(markdown_to_html(input, &options))
            });
        },
    );
    
    c.bench_with_input(
        BenchmarkId::new("comrak", "large"),
        &large,
        |b, input| {
            b.iter(|| {
                black_box(markdown_to_html(input, &options))
            });
        },
    );
}

criterion_group!(benches, bench_md2html, bench_pulldown_cmark, bench_comrak);
criterion_main!(benches);
# Medium-Sized Markdown Document

This document contains a moderate amount of content suitable for performance testing.

## Introduction

Markdown is a lightweight markup language that you can use to add formatting elements to plaintext text documents. Created by John Gruber in 2004, Markdown is now one of the world's most popular markup languages.

### Why Use Markdown?

- **Simplicity**: Easy to read and write
- **Portability**: Plain text files work everywhere
- **Flexibility**: Convert to many output formats
- **Version Control**: Works great with Git

## Syntax Examples

### Text Formatting

You can make text **bold** or *italic* with ease. You can also combine them for ***bold and italic*** text. Don't forget about `inline code` formatting!

### Headers at Different Levels

#### Level 4 Header
##### Level 5 Header
###### Level 6 Header

### Extended Lists

1. First ordered item
   - Nested unordered item
   - Another nested item
2. Second ordered item
   1. Nested ordered item
   2. Another nested ordered item
3. Third ordered item

### Multiple Code Blocks

```python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

# Example usage
for i in range(10):
    print(f"F({i}) = {fibonacci(i)}")
```

```javascript
// JavaScript example
class Calculator {
    constructor() {
        this.result = 0;
    }
    
    add(value) {
        this.result += value;
        return this;
    }
    
    multiply(value) {
        this.result *= value;
        return this;
    }
    
    getResult() {
        return this.result;
    }
}

const calc = new Calculator();
console.log(calc.add(5).multiply(3).getResult()); // 15
```

```rust
// Rust example
use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }
    
    counts
}

fn main() {
    let text = "The quick brown fox jumps over the lazy dog";
    let counts = word_count(text);
    
    for (word, count) in counts {
        println!("{}: {}", word, count);
    }
}
```

## Links and References

Here are some useful links:
- [GitHub](https://github.com)
- [Stack Overflow](https://stackoverflow.com)
- [Mozilla Developer Network](https://developer.mozilla.org)
- [Rust Documentation](https://doc.rust-lang.org)

## Conclusion

This medium-sized document demonstrates various Markdown features and provides a good balance of content for benchmarking purposes. The mix of different elements helps test the parser's performance across different scenarios.
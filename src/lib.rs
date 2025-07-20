use std::fmt::Write;

mod simd;
use simd::{detect_line_type_simd, find_delimiter_simd, html_escape_simd_into, LineType};

pub struct MarkdownParser<'a> {
    input: &'a str,
}

impl<'a> MarkdownParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn parse(&self) -> String {
        let lines: Vec<&str> = self.input.lines().collect();
        let estimated_capacity = self.input.len() * 2;
        let mut output = String::with_capacity(estimated_capacity);
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];

            // Use SIMD-accelerated line type detection
            match detect_line_type_simd(line) {
                LineType::Empty => {
                    i += 1;
                    continue;
                }
                LineType::Header(level) => {
                    let content = line.trim_start().get((level + 1)..).unwrap_or("").trim();
                    let mut result = String::with_capacity(content.len() + 20);
                    write!(result, "<h{level}>").unwrap();
                    self.process_inline_elements_into(content, &mut result);
                    write!(result, "</h{level}>").unwrap();
                    output.push_str(&result);
                    output.push('\n');
                }
                LineType::CodeBlock => {
                    let (code_block, lines_consumed) = self.parse_code_block(&lines[i..]);
                    output.push_str(&code_block);
                    output.push('\n');
                    i += lines_consumed - 1;
                }
                LineType::UnorderedList => {
                    let (list, lines_consumed) = self.parse_unordered_list(&lines[i..]);
                    output.push_str(&list);
                    output.push('\n');
                    i += lines_consumed - 1;
                }
                LineType::OrderedList => {
                    let (list, lines_consumed) = self.parse_ordered_list(&lines[i..]);
                    output.push_str(&list);
                    output.push('\n');
                    i += lines_consumed - 1;
                }
                LineType::Paragraph => {
                    let (paragraph, lines_consumed) = self.parse_paragraph(&lines[i..]);
                    output.push_str(&paragraph);
                    output.push('\n');
                    i += lines_consumed - 1;
                }
            }

            i += 1;
        }

        output
    }

    fn parse_header(&self, line: &str) -> Option<String> {
        let trimmed = line.trim_start();
        let level = trimmed.bytes().take_while(|&b| b == b'#').count();

        if level > 0 && level <= 6 && trimmed.as_bytes().get(level) == Some(&b' ') {
            let content = trimmed[(level + 1)..].trim();
            let mut result = String::with_capacity(content.len() + 20);
            write!(result, "<h{level}>").unwrap();
            self.process_inline_elements_into(content, &mut result);
            write!(result, "</h{level}>").unwrap();
            Some(result)
        } else {
            None
        }
    }

    #[inline]
    fn is_code_block_start(&self, line: &str) -> bool {
        line.trim_start().starts_with("```")
    }

    fn parse_code_block(&self, lines: &[&str]) -> (String, usize) {
        if lines.is_empty() || !self.is_code_block_start(lines[0]) {
            return (String::new(), 0);
        }

        let mut result = String::with_capacity(512);
        result.push_str("<pre><code>");

        let mut i = 1;
        while i < lines.len() && !lines[i].trim_start().starts_with("```") {
            if i > 1 {
                result.push('\n');
            }
            html_escape_simd_into(lines[i], &mut result);
            i += 1;
        }

        result.push_str("</code></pre>");
        (result, i + 1)
    }

    #[inline]
    fn is_unordered_list_item(&self, line: &str) -> bool {
        let trimmed = line.trim_start();
        trimmed.len() >= 2 && matches!(trimmed.as_bytes(), [b'-' | b'*' | b'+', b' ', ..])
    }

    fn parse_unordered_list(&self, lines: &[&str]) -> (String, usize) {
        let mut result = String::with_capacity(256);
        result.push_str("<ul>\n");
        let mut i = 0;

        while i < lines.len() && self.is_unordered_list_item(lines[i]) {
            let content = lines[i].trim_start().get(2..).unwrap_or("").trim();
            result.push_str("  <li>");
            self.process_inline_elements_into(content, &mut result);
            result.push_str("</li>\n");
            i += 1;
        }

        result.push_str("</ul>");
        (result, i)
    }

    #[inline]
    fn is_ordered_list_item(&self, line: &str) -> bool {
        let trimmed = line.trim_start();
        if let Some(dot_pos) = trimmed.find(". ") {
            dot_pos > 0 && trimmed[..dot_pos].bytes().all(|b| b.is_ascii_digit())
        } else {
            false
        }
    }

    fn parse_ordered_list(&self, lines: &[&str]) -> (String, usize) {
        let mut result = String::with_capacity(256);
        result.push_str("<ol>\n");
        let mut i = 0;

        while i < lines.len() && self.is_ordered_list_item(lines[i]) {
            let trimmed = lines[i].trim_start();
            if let Some(dot_pos) = trimmed.find(". ") {
                let content = trimmed[(dot_pos + 2)..].trim();
                result.push_str("  <li>");
                self.process_inline_elements_into(content, &mut result);
                result.push_str("</li>\n");
            }
            i += 1;
        }

        result.push_str("</ol>");
        (result, i)
    }

    fn parse_paragraph(&self, lines: &[&str]) -> (String, usize) {
        let mut paragraph_content = String::with_capacity(256);
        let mut i = 0;
        let mut first_line = true;

        while i < lines.len() {
            let line = lines[i];
            if line.trim().is_empty()
                || self.parse_header(line).is_some()
                || self.is_code_block_start(line)
                || self.is_unordered_list_item(line)
                || self.is_ordered_list_item(line)
            {
                break;
            }

            if !first_line {
                paragraph_content.push(' ');
            }
            paragraph_content.push_str(line);
            first_line = false;
            i += 1;
        }

        let mut result = String::with_capacity(paragraph_content.len() + 20);
        result.push_str("<p>");
        self.process_inline_elements_into(&paragraph_content, &mut result);
        result.push_str("</p>");
        (result, i)
    }

    fn process_inline_elements_into(&self, text: &str, output: &mut String) {
        let bytes = text.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            match bytes[i] {
                b'*' => {
                    if i + 1 < bytes.len() && bytes[i + 1] == b'*' {
                        if let Some(consumed) = self.try_parse_bold(text, i, output) {
                            i += consumed;
                            continue;
                        }
                    }
                    if let Some(consumed) = self.try_parse_italic(text, i, output) {
                        i += consumed;
                        continue;
                    }
                    output.push('*');
                }
                b'_' => {
                    if let Some(consumed) = self.try_parse_italic(text, i, output) {
                        i += consumed;
                        continue;
                    }
                    output.push('_');
                }
                b'`' => {
                    if let Some(consumed) = self.try_parse_inline_code(text, i, output) {
                        i += consumed;
                        continue;
                    }
                    output.push('`');
                }
                b'[' => {
                    if let Some(consumed) = self.try_parse_link(text, i, output) {
                        i += consumed;
                        continue;
                    }
                    output.push('[');
                }
                b'&' => {
                    output.push_str("&amp;");
                }
                b'<' => {
                    output.push_str("&lt;");
                }
                b'>' => {
                    output.push_str("&gt;");
                }
                b'"' => {
                    output.push_str("&quot;");
                }
                b'\'' => {
                    output.push_str("&#39;");
                }
                _ => {
                    if let Some(ch) = text[i..].chars().next() {
                        output.push(ch);
                        i += ch.len_utf8() - 1;
                    }
                }
            }
            i += 1;
        }
    }

    fn try_parse_bold(&self, text: &str, start: usize, output: &mut String) -> Option<usize> {
        let bytes = text.as_bytes();
        if start + 3 >= bytes.len() || bytes[start] != b'*' || bytes[start + 1] != b'*' {
            return None;
        }

        // Use SIMD to find closing **
        if let Some(end_pos) = find_delimiter_simd(text, b'*', start + 2) {
            if end_pos + 1 < bytes.len() && bytes[end_pos + 1] == b'*' {
                output.push_str("<strong>");
                let content = &text[(start + 2)..end_pos];
                self.process_inline_elements_into(content, output);
                output.push_str("</strong>");
                return Some(end_pos - start + 2);
            }
        }
        None
    }

    fn try_parse_italic(&self, text: &str, start: usize, output: &mut String) -> Option<usize> {
        let bytes = text.as_bytes();
        if start >= bytes.len() {
            return None;
        }

        let delimiter = bytes[start];
        if delimiter != b'*' && delimiter != b'_' {
            return None;
        }

        // Use SIMD to find closing delimiter
        if let Some(end_pos) = find_delimiter_simd(text, delimiter, start + 1) {
            output.push_str("<em>");
            let content = &text[(start + 1)..end_pos];
            output.push_str(content);
            output.push_str("</em>");
            return Some(end_pos - start + 1);
        }
        None
    }

    fn try_parse_inline_code(
        &self,
        text: &str,
        start: usize,
        output: &mut String,
    ) -> Option<usize> {
        let bytes = text.as_bytes();
        if start >= bytes.len() || bytes[start] != b'`' {
            return None;
        }

        // Use SIMD to find closing backtick
        if let Some(end_pos) = find_delimiter_simd(text, b'`', start + 1) {
            output.push_str("<code>");
            let content = &text[(start + 1)..end_pos];
            html_escape_simd_into(content, output);
            output.push_str("</code>");
            return Some(end_pos - start + 1);
        }
        None
    }

    fn try_parse_link(&self, text: &str, start: usize, output: &mut String) -> Option<usize> {
        let bytes = text.as_bytes();
        if start >= bytes.len() || bytes[start] != b'[' {
            return None;
        }

        // Use SIMD to find closing bracket and parenthesis
        let bracket_end = find_delimiter_simd(text, b']', start + 1)?;

        if bracket_end + 1 >= bytes.len() || bytes[bracket_end + 1] != b'(' {
            return None;
        }

        let paren_end = find_delimiter_simd(text, b')', bracket_end + 2)?;

        let link_text = &text[(start + 1)..bracket_end];
        let url = &text[(bracket_end + 2)..paren_end];

        output.push_str("<a href=\"");
        html_escape_simd_into(url, output);
        output.push_str("\">");
        html_escape_simd_into(link_text, output);
        output.push_str("</a>");

        Some(paren_end - start + 1)
    }
}

// Legacy function removed - all code now uses html_escape_simd_into directly

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headers() {
        let parser = MarkdownParser::new("# Header 1\n## Header 2\n### Header 3");
        let result = parser.parse();
        assert!(result.contains("<h1>Header 1</h1>"));
        assert!(result.contains("<h2>Header 2</h2>"));
        assert!(result.contains("<h3>Header 3</h3>"));
    }

    #[test]
    fn test_bold_and_italic() {
        let parser = MarkdownParser::new("This is **bold** and *italic* text");
        let result = parser.parse();
        assert!(result.contains("<strong>bold</strong>"));
        assert!(result.contains("<em>italic</em>"));
    }

    #[test]
    fn test_inline_code() {
        let parser = MarkdownParser::new("Use `code` for inline code");
        let result = parser.parse();
        assert!(result.contains("<code>code</code>"));
    }

    #[test]
    fn test_links() {
        let parser = MarkdownParser::new("This is a [link](https://example.com)");
        let result = parser.parse();
        assert!(result.contains("<a href=\"https://example.com\">link</a>"));
    }

    #[test]
    fn test_unordered_list() {
        let parser = MarkdownParser::new("- Item 1\n- Item 2\n- Item 3");
        let result = parser.parse();
        assert!(result.contains("<ul>"));
        assert!(result.contains("<li>Item 1</li>"));
        assert!(result.contains("<li>Item 2</li>"));
        assert!(result.contains("<li>Item 3</li>"));
        assert!(result.contains("</ul>"));
    }

    #[test]
    fn test_ordered_list() {
        let parser = MarkdownParser::new("1. First\n2. Second\n3. Third");
        let result = parser.parse();
        assert!(result.contains("<ol>"));
        assert!(result.contains("<li>First</li>"));
        assert!(result.contains("<li>Second</li>"));
        assert!(result.contains("<li>Third</li>"));
        assert!(result.contains("</ol>"));
    }

    #[test]
    fn test_code_block() {
        let parser = MarkdownParser::new("```\ncode block\nwith multiple lines\n```");
        let result = parser.parse();
        assert!(result.contains("<pre><code>"));
        assert!(result.contains("code block\nwith multiple lines"));
        assert!(result.contains("</code></pre>"));
    }

    #[test]
    fn test_paragraph() {
        let parser = MarkdownParser::new("This is a paragraph\nwith multiple lines.");
        let result = parser.parse();
        assert!(result.contains("<p>This is a paragraph with multiple lines.</p>"));
    }
}

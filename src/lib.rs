use std::fmt::Write;

pub struct MarkdownParser {
    input: String,
}

impl MarkdownParser {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn parse(&mut self) -> String {
        let mut output = String::new();
        let lines: Vec<&str> = self.input.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];
            
            if line.trim().is_empty() {
                i += 1;
                continue;
            }

            if let Some(header) = self.parse_header(line) {
                output.push_str(&header);
                output.push('\n');
            } else if self.is_code_block_start(line) {
                let (code_block, lines_consumed) = self.parse_code_block(&lines[i..]);
                output.push_str(&code_block);
                output.push('\n');
                i += lines_consumed - 1;
            } else if self.is_unordered_list_item(line) {
                let (list, lines_consumed) = self.parse_unordered_list(&lines[i..]);
                output.push_str(&list);
                output.push('\n');
                i += lines_consumed - 1;
            } else if self.is_ordered_list_item(line) {
                let (list, lines_consumed) = self.parse_ordered_list(&lines[i..]);
                output.push_str(&list);
                output.push('\n');
                i += lines_consumed - 1;
            } else {
                let (paragraph, lines_consumed) = self.parse_paragraph(&lines[i..]);
                output.push_str(&paragraph);
                output.push('\n');
                i += lines_consumed - 1;
            }
            
            i += 1;
        }

        output
    }

    fn parse_header(&self, line: &str) -> Option<String> {
        let trimmed = line.trim_start();
        let level = trimmed.chars().take_while(|&c| c == '#').count();
        
        if level > 0 && level <= 6 && trimmed.chars().nth(level) == Some(' ') {
            let content = trimmed[(level + 1)..].trim();
            let processed_content = self.process_inline_elements(content);
            Some(format!("<h{level}>{processed_content}</h{level}>"))
        } else {
            None
        }
    }

    fn is_code_block_start(&self, line: &str) -> bool {
        line.trim().starts_with("```")
    }

    fn parse_code_block(&self, lines: &[&str]) -> (String, usize) {
        if lines.is_empty() || !self.is_code_block_start(lines[0]) {
            return (String::new(), 0);
        }

        let mut result = String::from("<pre><code>");
        let _language = lines[0].trim().strip_prefix("```").unwrap_or("").trim();
        
        let mut i = 1;
        while i < lines.len() && !lines[i].trim().starts_with("```") {
            if i > 1 {
                result.push('\n');
            }
            result.push_str(&html_escape(lines[i]));
            i += 1;
        }
        
        result.push_str("</code></pre>");
        (result, i + 1)
    }

    fn is_unordered_list_item(&self, line: &str) -> bool {
        let trimmed = line.trim_start();
        trimmed.starts_with("- ") || trimmed.starts_with("* ") || trimmed.starts_with("+ ")
    }

    fn parse_unordered_list(&self, lines: &[&str]) -> (String, usize) {
        let mut result = String::from("<ul>\n");
        let mut i = 0;

        while i < lines.len() && self.is_unordered_list_item(lines[i]) {
            let content = lines[i].trim_start().get(2..).unwrap_or("").trim();
            let processed_content = self.process_inline_elements(content);
            writeln!(result, "  <li>{processed_content}</li>").unwrap();
            i += 1;
        }

        result.push_str("</ul>");
        (result, i)
    }

    fn is_ordered_list_item(&self, line: &str) -> bool {
        let trimmed = line.trim_start();
        if let Some(dot_pos) = trimmed.find(". ") {
            trimmed[..dot_pos].chars().all(|c| c.is_numeric())
        } else {
            false
        }
    }

    fn parse_ordered_list(&self, lines: &[&str]) -> (String, usize) {
        let mut result = String::from("<ol>\n");
        let mut i = 0;

        while i < lines.len() && self.is_ordered_list_item(lines[i]) {
            let trimmed = lines[i].trim_start();
            if let Some(dot_pos) = trimmed.find(". ") {
                let content = trimmed[(dot_pos + 2)..].trim();
                let processed_content = self.process_inline_elements(content);
                writeln!(result, "  <li>{processed_content}</li>").unwrap();
            }
            i += 1;
        }

        result.push_str("</ol>");
        (result, i)
    }

    fn parse_paragraph(&self, lines: &[&str]) -> (String, usize) {
        let mut paragraph_lines = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];
            if line.trim().is_empty() 
                || self.parse_header(line).is_some()
                || self.is_code_block_start(line)
                || self.is_unordered_list_item(line)
                || self.is_ordered_list_item(line) {
                break;
            }
            paragraph_lines.push(line);
            i += 1;
        }

        let content = paragraph_lines.join(" ");
        let processed_content = self.process_inline_elements(&content);
        (format!("<p>{processed_content}</p>"), i)
    }

    fn process_inline_elements(&self, text: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
                let (bold_text, consumed) = self.parse_bold(&chars[i..]);
                if consumed > 0 {
                    result.push_str(&bold_text);
                    i += consumed;
                    continue;
                }
            }

            if chars[i] == '*' || chars[i] == '_' {
                let (italic_text, consumed) = self.parse_italic(&chars[i..]);
                if consumed > 0 {
                    result.push_str(&italic_text);
                    i += consumed;
                    continue;
                }
            }

            if chars[i] == '`' {
                let (code_text, consumed) = self.parse_inline_code(&chars[i..]);
                if consumed > 0 {
                    result.push_str(&code_text);
                    i += consumed;
                    continue;
                }
            }

            if chars[i] == '[' {
                let (link_text, consumed) = self.parse_link(&chars[i..]);
                if consumed > 0 {
                    result.push_str(&link_text);
                    i += consumed;
                    continue;
                }
            }

            result.push(chars[i]);
            i += 1;
        }

        result
    }

    fn parse_bold(&self, chars: &[char]) -> (String, usize) {
        if chars.len() < 4 || chars[0] != '*' || chars[1] != '*' {
            return (String::new(), 0);
        }

        let mut end = 2;
        while end + 1 < chars.len() {
            if chars[end] == '*' && chars[end + 1] == '*' {
                let content: String = chars[2..end].iter().collect();
                return (format!("<strong>{content}</strong>"), end + 2);
            }
            end += 1;
        }

        (String::new(), 0)
    }

    fn parse_italic(&self, chars: &[char]) -> (String, usize) {
        if chars.is_empty() || (chars[0] != '*' && chars[0] != '_') {
            return (String::new(), 0);
        }

        let delimiter = chars[0];
        let mut end = 1;
        
        while end < chars.len() {
            if chars[end] == delimiter {
                let content: String = chars[1..end].iter().collect();
                return (format!("<em>{content}</em>"), end + 1);
            }
            end += 1;
        }

        (String::new(), 0)
    }

    fn parse_inline_code(&self, chars: &[char]) -> (String, usize) {
        if chars.is_empty() || chars[0] != '`' {
            return (String::new(), 0);
        }

        let mut end = 1;
        while end < chars.len() {
            if chars[end] == '`' {
                let content: String = chars[1..end].iter().collect();
                return (format!("<code>{}</code>", html_escape(&content)), end + 1);
            }
            end += 1;
        }

        (String::new(), 0)
    }

    fn parse_link(&self, chars: &[char]) -> (String, usize) {
        if chars.is_empty() || chars[0] != '[' {
            return (String::new(), 0);
        }

        let mut bracket_end = 1;
        while bracket_end < chars.len() && chars[bracket_end] != ']' {
            bracket_end += 1;
        }

        if bracket_end >= chars.len() || bracket_end + 1 >= chars.len() || chars[bracket_end + 1] != '(' {
            return (String::new(), 0);
        }

        let mut paren_end = bracket_end + 2;
        while paren_end < chars.len() && chars[paren_end] != ')' {
            paren_end += 1;
        }

        if paren_end >= chars.len() {
            return (String::new(), 0);
        }

        let link_text: String = chars[1..bracket_end].iter().collect();
        let url: String = chars[(bracket_end + 2)..paren_end].iter().collect();
        
        (format!("<a href=\"{}\">{}</a>", html_escape(&url), html_escape(&link_text)), paren_end + 1)
    }
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_headers() {
        let mut parser = MarkdownParser::new("# Header 1\n## Header 2\n### Header 3".to_string());
        let result = parser.parse();
        assert!(result.contains("<h1>Header 1</h1>"));
        assert!(result.contains("<h2>Header 2</h2>"));
        assert!(result.contains("<h3>Header 3</h3>"));
    }

    #[test]
    fn test_bold_and_italic() {
        let mut parser = MarkdownParser::new("This is **bold** and *italic* text".to_string());
        let result = parser.parse();
        assert!(result.contains("<strong>bold</strong>"));
        assert!(result.contains("<em>italic</em>"));
    }

    #[test]
    fn test_inline_code() {
        let mut parser = MarkdownParser::new("Use `code` for inline code".to_string());
        let result = parser.parse();
        assert!(result.contains("<code>code</code>"));
    }

    #[test]
    fn test_links() {
        let mut parser = MarkdownParser::new("This is a [link](https://example.com)".to_string());
        let result = parser.parse();
        assert!(result.contains("<a href=\"https://example.com\">link</a>"));
    }

    #[test]
    fn test_unordered_list() {
        let mut parser = MarkdownParser::new("- Item 1\n- Item 2\n- Item 3".to_string());
        let result = parser.parse();
        assert!(result.contains("<ul>"));
        assert!(result.contains("<li>Item 1</li>"));
        assert!(result.contains("<li>Item 2</li>"));
        assert!(result.contains("<li>Item 3</li>"));
        assert!(result.contains("</ul>"));
    }

    #[test]
    fn test_ordered_list() {
        let mut parser = MarkdownParser::new("1. First\n2. Second\n3. Third".to_string());
        let result = parser.parse();
        assert!(result.contains("<ol>"));
        assert!(result.contains("<li>First</li>"));
        assert!(result.contains("<li>Second</li>"));
        assert!(result.contains("<li>Third</li>"));
        assert!(result.contains("</ol>"));
    }

    #[test]
    fn test_code_block() {
        let mut parser = MarkdownParser::new("```\ncode block\nwith multiple lines\n```".to_string());
        let result = parser.parse();
        assert!(result.contains("<pre><code>"));
        assert!(result.contains("code block\nwith multiple lines"));
        assert!(result.contains("</code></pre>"));
    }

    #[test]
    fn test_paragraph() {
        let mut parser = MarkdownParser::new("This is a paragraph\nwith multiple lines.".to_string());
        let result = parser.parse();
        assert!(result.contains("<p>This is a paragraph with multiple lines.</p>"));
    }
}
use anyhow::Result;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use serde_json::Value;
use std::io::stdout;

// Add this helper function before the StreamResponse implementation
pub fn strip_markdown_code_blocks(s: &str) -> String {
    let s = s.trim();

    // Check if string starts with ```json and ends with ```
    if s.starts_with("```json") && s.ends_with("```") {
        // Extract the content between the markers
        let without_start = s.strip_prefix("```json").unwrap_or(s);
        let content = without_start.strip_suffix("```").unwrap_or(without_start);
        return content.trim().to_string();
    }
    // s.replace("```json", "").replace("```", "")
    // If not a full code block, just return the original string
    s.to_string()
}

// Define our rendering states with associated colors
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum MarkdownState {
    Normal,
    Heading(usize), // Level 1-6
    Bold,
    Italic,
    BoldItalic,
    CodeBlock(String), // Language
    InlineCode,
    UnorderedList(usize),      // Nesting level
    OrderedList(usize, usize), // Nesting level, current number
    Link,
    LinkUrl,
    Blockquote(usize), // Nesting level
}

pub struct MarkdownStreamRenderer {
    buffer: String,
    in_response: bool,
    depth: i32,
    extracted: String,
    state_stack: Vec<MarkdownState>,
    current_line: String,
}

impl MarkdownStreamRenderer {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            in_response: false,
            depth: 0,
            extracted: String::new(),
            state_stack: vec![MarkdownState::Normal],
            current_line: String::new(),
        }
    }

    pub fn process_chunk(&mut self, chunk: &str) -> String {
        self.buffer.push_str(chunk);
        let mut output = String::new();

        // Try to find response field
        if !self.in_response && self.buffer.contains(r#""response":"#) {
            if let Some(pos) = self.buffer.find(r#""response":"#) {
                // Safely get the part after the response field
                let safe_start = pos + r#""response":"#.len();
                let chars: Vec<char> = self.buffer.chars().collect();
                if safe_start < self.buffer.len() {
                    // Use character iterator to ensure we slice at character boundaries
                    let new_buffer: String = chars.into_iter().skip(safe_start).collect();
                    self.buffer = new_buffer;
                } else {
                    self.buffer.clear();
                }
                self.in_response = true;
                self.depth = 0;
            }
        }

        // If we've found the response field, start extracting its content
        if self.in_response {
            let chars: Vec<char> = self.buffer.chars().collect();
            let mut i = 0;

            while i < chars.len() {
                let c = chars[i];

                // Check for quote characters
                let is_escape_quote = i > 0 && chars[i - 1] == '\\' && c == '"';

                if c == '"' && !is_escape_quote {
                    if self.depth == 0 {
                        // Found start quote - start capturing but don't include the quote itself
                        self.depth = 1;
                        i += 1; // Skip the opening quote
                        continue;
                    } else {
                        // Found end quote - stop capturing
                        self.depth = 0;
                        self.in_response = false;
                        break;
                    }
                }

                if self.depth == 1 {
                    if c == '\\' && i + 1 < chars.len() {
                        // Normal case: both backslash and escaped char are in the same chunk
                        let next_char = chars[i + 1];
                        if next_char == 'n' {
                            output.push('\n');
                        } else if next_char == '"' {
                            output.push('"');
                        } else if next_char == '\\' {
                            output.push('\\');
                        } else {
                            output.push(c);
                            i -= 1; // Unknown escape, don't skip next character
                        }
                        i += 1; // Skip the next character in the escape sequence
                    } else if self.extracted.chars().last() == Some('\\') {
                        // Split case: backslash was at end of previous chunk
                        if c == 'n' {
                            output.push('\n');
                            // Remove the backslash from extracted to prevent double-handling
                            self.extracted.pop();
                        } else if c == '"' {
                            output.push('"');
                            self.extracted.pop();
                        } else if c == '\\' {
                            output.push('\\');
                            self.extracted.pop();
                        } else {
                            // Not an escape sequence we recognize, just add the original backslash and this char
                            output.push(c);
                        }
                    } else {
                        output.push(c);
                    }
                }

                i += 1;
            }

            // Update buffer, using character indices instead of byte indices
            if !self.in_response {
                self.buffer.clear();
            } else if i < chars.len() {
                self.buffer = chars.into_iter().skip(i).collect();
            } else {
                self.buffer.clear();
            }
        }
        self.extracted.push_str(&output);
        let output = if output.chars().last() == Some('\\') {
            output.pop();
            output
        } else {
            output
        };
        // Render markdown if we have content
        if !output.is_empty() {
            let _ = self.render_increment(&output);
            return String::new(); // Return empty since we're handling rendering
        }

        output
    }

    fn current_state(&self) -> &MarkdownState {
        self.state_stack.last().unwrap_or(&MarkdownState::Normal)
    }

    fn push_state(&mut self, state: MarkdownState) {
        self.state_stack.push(state);
    }

    fn pop_state(&mut self) -> Option<MarkdownState> {
        self.state_stack.pop()
    }

    fn render_increment(&mut self, text: &str) -> Result<()> {
        // Don't clear previous render - just render the new text incrementally
        // Process and render the new text
        self.process_text(text)?;

        // We don't need to track render height since we're not clearing anything

        Ok(())
    }

    fn process_text(&mut self, text: &str) -> Result<()> {
        let mut chars = text.chars().peekable();

        while let Some(c) = chars.next() {
            // Add character to current line buffer
            self.current_line.push(c);

            // Apply styling based on current character and state
            match c {
                '#' => {
                    // Check for heading at start of line
                    if self.current_line.trim() == "#" {
                        // First # of a potential heading
                        let mut level = 1;

                        // Look ahead to count consecutive # characters
                        let mut lookahead = chars.clone();
                        while lookahead.next_if_eq(&'#').is_some() {
                            level += 1;
                        }

                        // Set heading color based on level
                        let color = match level {
                            1 => Color::Magenta,
                            2 => Color::DarkMagenta,
                            3 => Color::Cyan,
                            _ => Color::White,
                        };

                        // Print the # with appropriate color
                        execute!(stdout(), SetForegroundColor(color), Print("#"))?;

                        // Push heading state
                        self.push_state(MarkdownState::Heading(level));
                    } else {
                        // Just a regular # character, not at start of line
                        self.print_with_current_style("#")?;
                    }
                }
                '*' => {
                    // Handle asterisks for bold/italic or list items
                    if self.current_line.trim() == "*" && chars.peek() == Some(&' ') {
                        // Unordered list item
                        execute!(stdout(), SetForegroundColor(Color::Green), Print("*"))?;
                        self.push_state(MarkdownState::UnorderedList(0));
                    } else if self.current_line.ends_with("**") {
                        // Bold marker
                        execute!(stdout(), SetForegroundColor(Color::Yellow), Print("**"))?;
                        self.current_line.pop(); // Remove the last * we just printed
                        self.current_line.pop(); // Remove the second-to-last *

                        // Toggle bold state
                        match self.current_state() {
                            MarkdownState::Bold => {
                                let _ = self.pop_state();
                                execute!(stdout(), ResetColor)?;
                            }
                            _ => self.push_state(MarkdownState::Bold),
                        }
                    } else if self.current_line.ends_with("*") && !self.current_line.ends_with("**")
                    {
                        // Italic marker
                        execute!(stdout(), SetForegroundColor(Color::Blue), Print("*"))?;
                        self.current_line.pop(); // Remove the * we just printed

                        // Toggle italic state
                        match self.current_state() {
                            MarkdownState::Italic => {
                                let _ = self.pop_state();
                                execute!(stdout(), ResetColor)?;
                            }
                            _ => self.push_state(MarkdownState::Italic),
                        }
                    } else {
                        // Just a regular asterisk
                        self.print_with_current_style("*")?;
                    }
                }
                '`' => {
                    // Handle backticks for code
                    if self.current_line.ends_with("```") {
                        // Code block marker
                        execute!(stdout(), SetForegroundColor(Color::Yellow), Print("```"))?;
                        self.current_line.pop(); // Remove the last `
                        self.current_line.pop(); // Remove the second `
                        self.current_line.pop(); // Remove the third `

                        // Toggle code block state
                        match self.current_state() {
                            MarkdownState::CodeBlock(_) => {
                                let _ = self.pop_state();
                                execute!(stdout(), ResetColor)?;
                            }
                            _ => self.push_state(MarkdownState::CodeBlock(String::new())),
                        }
                    } else if self.current_line.ends_with("`") {
                        // Inline code marker
                        execute!(stdout(), SetForegroundColor(Color::Yellow), Print("`"))?;
                        self.current_line.pop(); // Remove the ` we just printed

                        // Toggle inline code state
                        match self.current_state() {
                            MarkdownState::InlineCode => {
                                let _ = self.pop_state();
                                execute!(stdout(), ResetColor)?;
                            }
                            _ => self.push_state(MarkdownState::InlineCode),
                        }
                    } else {
                        // Just a regular backtick
                        self.print_with_current_style("`")?;
                    }
                }
                '[' => {
                    // Link opening bracket
                    execute!(stdout(), SetForegroundColor(Color::Blue), Print("["))?;
                    self.push_state(MarkdownState::Link);
                }
                ']' => {
                    // Link closing bracket
                    execute!(stdout(), SetForegroundColor(Color::Blue), Print("]"))?;

                    // Check if we're in a link state
                    if matches!(self.current_state(), MarkdownState::Link) {
                        self.pop_state();

                        // Check for opening parenthesis for URL
                        if chars.peek() == Some(&'(') {
                            self.push_state(MarkdownState::LinkUrl);
                        }
                    }
                }
                '(' => {
                    if matches!(self.current_state(), MarkdownState::LinkUrl) {
                        // URL opening parenthesis
                        execute!(stdout(), SetForegroundColor(Color::DarkBlue), Print("("))?;
                    } else {
                        // Regular parenthesis
                        self.print_with_current_style("(")?;
                    }
                }
                ')' => {
                    if matches!(self.current_state(), MarkdownState::LinkUrl) {
                        // URL closing parenthesis
                        execute!(stdout(), SetForegroundColor(Color::DarkBlue), Print(")"))?;
                        self.pop_state();
                    } else {
                        // Regular parenthesis
                        self.print_with_current_style(")")?;
                    }
                }
                '1'..='9' => {
                    // Check for ordered list at start of line
                    if self.current_line.trim().len() == 1 && chars.peek() == Some(&'.') {
                        // Potential ordered list item
                        execute!(stdout(), SetForegroundColor(Color::Green), Print(c))?;
                    } else {
                        // Regular digit
                        self.print_with_current_style(c.to_string().as_str())?;
                    }
                }
                '.' => {
                    if self.current_line.trim().len() >= 1
                        && self
                            .current_line
                            .trim()
                            .chars()
                            .next()
                            .unwrap()
                            .is_digit(10)
                        && self.current_line.trim().ends_with('.')
                        && chars.peek() == Some(&' ')
                    {
                        // Ordered list dot
                        execute!(stdout(), SetForegroundColor(Color::Green), Print("."))?;
                        self.push_state(MarkdownState::OrderedList(0, 0));
                    } else {
                        // Regular dot
                        self.print_with_current_style(".")?;
                    }
                }
                '>' => {
                    // Blockquote
                    if self.current_line.trim() == ">" {
                        execute!(stdout(), SetForegroundColor(Color::Cyan), Print(">"))?;
                        self.push_state(MarkdownState::Blockquote(1));
                    } else {
                        // Regular > character
                        self.print_with_current_style(">")?;
                    }
                }
                '\n' => {
                    // End of line
                    execute!(stdout(), Print("\n"))?;
                    self.current_line.clear();

                    // Reset line-specific states
                    self.reset_line_states()?;
                }
                _ => {
                    // Regular character
                    self.print_with_current_style(c.to_string().as_str())?;
                }
            }
        }

        Ok(())
    }

    // Helper method to print text with current style
    fn print_with_current_style(&self, text: &str) -> Result<()> {
        match self.current_state() {
            MarkdownState::Normal => {
                execute!(stdout(), ResetColor, Print(text))?;
            }
            MarkdownState::Heading(level) => {
                let color = match level {
                    1 => Color::Magenta,
                    2 => Color::DarkMagenta,
                    3 => Color::Cyan,
                    _ => Color::White,
                };
                execute!(stdout(), SetForegroundColor(color), Print(text))?;
            }
            MarkdownState::Bold => {
                execute!(stdout(), SetForegroundColor(Color::Yellow), Print(text))?;
            }
            MarkdownState::Italic => {
                execute!(stdout(), SetForegroundColor(Color::Blue), Print(text))?;
            }
            MarkdownState::BoldItalic => {
                execute!(stdout(), SetForegroundColor(Color::Magenta), Print(text))?;
            }
            MarkdownState::CodeBlock(_) => {
                execute!(stdout(), SetForegroundColor(Color::Yellow), Print(text))?;
            }
            MarkdownState::InlineCode => {
                execute!(stdout(), SetForegroundColor(Color::Yellow), Print(text))?;
            }
            MarkdownState::Link => {
                execute!(stdout(), SetForegroundColor(Color::Blue), Print(text))?;
            }
            MarkdownState::LinkUrl => {
                execute!(stdout(), SetForegroundColor(Color::DarkBlue), Print(text))?;
            }
            MarkdownState::UnorderedList(_) | MarkdownState::OrderedList(_, _) => {
                execute!(stdout(), SetForegroundColor(Color::Green), Print(text))?;
            }
            MarkdownState::Blockquote(_) => {
                execute!(stdout(), SetForegroundColor(Color::Cyan), Print(text))?;
            }
        }

        Ok(())
    }

    fn reset_line_states(&mut self) -> Result<()> {
        // Reset states that shouldn't persist across lines
        match self.current_state() {
            MarkdownState::Heading(_) => {
                self.pop_state();
                execute!(stdout(), ResetColor)?;
            }
            MarkdownState::UnorderedList(_) => {
                self.pop_state();
            }
            MarkdownState::OrderedList(_, _) => {
                self.pop_state();
            }
            MarkdownState::Blockquote(_) => {
                self.pop_state();
            }
            _ => {}
        }

        Ok(())
    }
}

// Helper function to extract response from JSON
pub fn extract_response(partial: &str) -> Option<&str> {
    // Find the start of the response field
    let response_marker = r#""response": ""#;
    let start_pos = partial.find(response_marker)?;
    let start_pos = start_pos + response_marker.len();

    // Get substring starting from the response value
    let response_substring = &partial[start_pos..];

    // Track the JSON string boundaries more carefully
    let mut depth = 0;
    let mut is_escaped = false;
    let mut end_pos = 0;

    for (i, c) in response_substring.char_indices() {
        if is_escaped {
            is_escaped = false;
            continue;
        }

        if c == '\\' {
            is_escaped = true;
            continue;
        }

        // Only consider a quote as the end if we're at the top level
        if c == '"' && depth == 0 {
            end_pos = i;
            break;
        }

        // Track balanced pairs for proper parsing
        match c {
            '[' => depth += 1,
            ']' => {
                if depth > 0 {
                    depth -= 1
                }
            }
            '{' => depth += 1,
            '}' => {
                if depth > 0 {
                    depth -= 1
                }
            }
            _ => {}
        }
    }

    // If we didn't find a closing quote, the JSON might be incomplete
    if end_pos == 0 {
        // Return what we have so far
        Some(response_substring)
    } else {
        // Extract just the response value
        let response = &response_substring[..end_pos];

        // We'll handle unescaping in the renderer
        Some(response)
    }
}

// JSON parsing utility functions
pub fn contains_end_tag(content: &str) -> bool {
    // Strip markdown code blocks first
    let clean_content = strip_markdown_code_blocks(content);

    // Try JSON
    match serde_json::from_str::<Value>(&clean_content) {
        Ok(json) => {
            if json
                .get("finished")
                .and_then(Value::as_bool)
                .unwrap_or(false)
            {
                return true;
            }
        }
        Err(_) => {}
    }
    return false;
}

pub fn contains_tool_call(content: &str) -> Option<(String, String)> {
    // Strip markdown code blocks first
    let clean_content = strip_markdown_code_blocks(content);

    // Try JSON
    match serde_json::from_str::<Value>(&clean_content) {
        Ok(json) => {
            if let Some(tool) = json.get("tool") {
                if tool.is_null() {
                    return None;
                }

                let tool_name = tool.get("name")?.as_str()?;
                let tool_content = tool.get("content")?.as_str()?;

                return Some((tool_name.to_string(), tool_content.to_string()));
            }
        }
        Err(_) => {}
    }

    None
}

pub fn extract_tool_content(content: &str) -> Option<String> {
    // Try JSON parsing
    if let Some(tool_content) = extract_tool_content_json(content) {
        return Some(tool_content);
    }
    None
}

fn extract_tool_content_json(content: &str) -> Option<String> {
    // Strip markdown code blocks first
    let clean_content = strip_markdown_code_blocks(content);

    match serde_json::from_str::<Value>(&clean_content) {
        Ok(json) => {
            if let Some(tool) = json.get("tool") {
                if tool.is_object() && tool.get("name")? == "cli" {
                    return tool.get("content")?.as_str().map(String::from);
                }
            }
            None
        }
        Err(_) => None,
    }
}

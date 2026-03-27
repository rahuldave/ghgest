use std::io;

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use yansi::Paint;

use super::theme::Theme;

/// Tracking state for the markdown renderer.
struct RenderState {
  /// Current column position (for word wrapping).
  col: usize,
  /// Whether we've written any content at all.
  has_content: bool,
  /// Current heading level (1-6).
  heading_level: u8,
  /// Whether we're inside a blockquote.
  in_blockquote: bool,
  /// Whether we're inside bold text.
  in_bold: bool,
  /// Whether we're inside a code block (no word wrapping).
  in_code_block: bool,
  /// Whether we're inside a heading.
  in_heading: bool,
  /// Whether we're inside italic text.
  in_italic: bool,
  /// Whether we're inside a link.
  in_link: bool,
  /// Current line buffer (unstyled text for word-wrap measurement).
  line_buf: String,
  /// The URL of the current link.
  link_url: String,
  /// Ordered list counter stack (None = unordered, Some(n) = ordered starting at n).
  list_stack: Vec<Option<u64>>,
  /// Whether we need a blank line before the next block element.
  needs_blank_line: bool,
  /// Maximum line width for word wrapping.
  width: usize,
}

impl RenderState {
  fn new(width: usize) -> Self {
    Self {
      col: 0,
      has_content: false,
      heading_level: 1,
      in_blockquote: false,
      in_bold: false,
      in_code_block: false,
      in_heading: false,
      in_italic: false,
      in_link: false,
      line_buf: String::new(),
      link_url: String::new(),
      list_stack: Vec::new(),
      needs_blank_line: false,
      width,
    }
  }

  fn effective_width(&self) -> usize {
    let indent = self.indent();
    let bq = if self.in_blockquote { 2 } else { 0 };
    self.width.saturating_sub(indent + bq)
  }

  fn indent(&self) -> usize {
    if self.list_stack.len() > 1 {
      (self.list_stack.len() - 1) * 2
    } else {
      0
    }
  }
}

/// Render markdown text to the given writer using ANSI styling from the theme.
///
/// When yansi color is globally disabled (e.g. piped output), the output is
/// plain text with no ANSI escape sequences.
///
/// Accepts `&mut impl io::Write` consistent with ADR-0006 component pattern.
pub fn render(w: &mut impl io::Write, markdown: &str, theme: &Theme) -> io::Result<()> {
  let width = terminal_width();
  render_with_width(w, markdown, theme, width)
}

fn ensure_blank_line(w: &mut impl io::Write, state: &mut RenderState) -> io::Result<()> {
  if state.needs_blank_line && state.has_content {
    writeln!(w)?;
    state.needs_blank_line = false;
  }
  Ok(())
}

fn flush_line(w: &mut impl io::Write, state: &mut RenderState) -> io::Result<()> {
  if !state.line_buf.is_empty() {
    let text = std::mem::take(&mut state.line_buf);
    write!(w, "{text}")?;
  }
  if state.col > 0 {
    writeln!(w)?;
    state.col = 0;
  }
  Ok(())
}

fn handle_end(w: &mut impl io::Write, tag_end: &TagEnd, theme: &Theme, state: &mut RenderState) -> io::Result<()> {
  match tag_end {
    TagEnd::Heading(_) => {
      // Flush heading text
      let text = std::mem::take(&mut state.line_buf);
      let prefix = "#".repeat(state.heading_level as usize);
      let line = format!("{prefix} {text}");
      writeln!(w, "{}", line.paint(theme.md_heading))?;
      state.in_heading = false;
      state.col = 0;
      state.needs_blank_line = true;
      state.has_content = true;
    }
    TagEnd::Paragraph => {
      flush_line(w, state)?;
      writeln!(w)?;
      state.needs_blank_line = false;
      state.has_content = true;
    }
    TagEnd::BlockQuote(_) => {
      state.in_blockquote = false;
      state.needs_blank_line = true;
      state.has_content = true;
    }
    TagEnd::CodeBlock => {
      state.in_code_block = false;
      state.needs_blank_line = true;
      state.has_content = true;
    }
    TagEnd::List(_) => {
      state.list_stack.pop();
      if state.list_stack.is_empty() {
        state.needs_blank_line = true;
      }
      state.has_content = true;
    }
    TagEnd::Item => {
      flush_line(w, state)?;
    }
    TagEnd::Emphasis => {
      state.in_italic = false;
    }
    TagEnd::Strong => {
      state.in_bold = false;
    }
    TagEnd::Link => {
      // After link text, append URL in parentheses
      let url = std::mem::take(&mut state.link_url);
      let url_display = format!(" ({url})");
      state.line_buf.push_str(&url_display);
      state.in_link = false;
    }
    _ => {}
  }
  Ok(())
}

fn handle_hard_break(w: &mut impl io::Write, state: &mut RenderState) -> io::Result<()> {
  flush_line(w, state)?;
  writeln!(w)?;
  Ok(())
}

fn handle_inline_code(w: &mut impl io::Write, code: &str, theme: &Theme, state: &mut RenderState) -> io::Result<()> {
  if state.in_heading {
    state.line_buf.push('`');
    state.line_buf.push_str(code);
    state.line_buf.push('`');
    return Ok(());
  }

  let styled = format!("`{code}`").paint(theme.md_code).to_string();
  let plain = format!("`{code}`");

  // Treat inline code as a single "word" for wrapping
  let word_len = plain.len();
  let effective_width = state.effective_width();

  if state.col > 0 && state.col + 1 + word_len > effective_width {
    writeln!(w)?;
    write_line_prefix(w, state)?;
    state.col = 0;
  } else if state.col > 0 {
    write!(w, " ")?;
    state.col += 1;
  }

  write!(w, "{styled}")?;
  state.col += word_len;
  state.has_content = true;

  Ok(())
}

fn handle_rule(w: &mut impl io::Write, theme: &Theme, state: &mut RenderState) -> io::Result<()> {
  ensure_blank_line(w, state)?;
  let rule = "\u{2500}".repeat(state.width.min(40));
  writeln!(w, "{}", rule.paint(theme.md_rule))?;
  state.needs_blank_line = true;
  state.has_content = true;
  Ok(())
}

fn handle_soft_break(w: &mut impl io::Write, state: &mut RenderState) -> io::Result<()> {
  // Treat soft break as a space for word wrapping
  if !state.line_buf.is_empty() {
    flush_line(w, state)?;
  }
  Ok(())
}

fn handle_start(w: &mut impl io::Write, tag: &Tag<'_>, _theme: &Theme, state: &mut RenderState) -> io::Result<()> {
  match tag {
    Tag::Heading {
      level, ..
    } => {
      ensure_blank_line(w, state)?;
      state.in_heading = true;
      state.heading_level = heading_level_to_u8(level);
    }
    Tag::Paragraph => {
      ensure_blank_line(w, state)?;
    }
    Tag::BlockQuote(_) => {
      ensure_blank_line(w, state)?;
      state.in_blockquote = true;
    }
    Tag::CodeBlock(_) => {
      ensure_blank_line(w, state)?;
      state.in_code_block = true;
    }
    Tag::List(start) => {
      ensure_blank_line(w, state)?;
      state.list_stack.push(*start);
    }
    Tag::Item => {
      let marker = match state.list_stack.last_mut() {
        Some(Some(n)) => {
          let s = format!("{n}.");
          *n += 1;
          s
        }
        Some(None) => "-".to_string(),
        None => "-".to_string(),
      };
      write_indent(w, state)?;
      write!(w, "{marker}")?;
      state.col = state.indent() + marker.len();
    }
    Tag::Emphasis => {
      state.in_italic = true;
    }
    Tag::Strong => {
      state.in_bold = true;
    }
    Tag::Link {
      dest_url, ..
    } => {
      state.in_link = true;
      state.link_url = dest_url.to_string();
    }
    _ => {}
  }
  Ok(())
}

fn handle_text(w: &mut impl io::Write, text: &str, theme: &Theme, state: &mut RenderState) -> io::Result<()> {
  if state.in_code_block {
    // Code blocks: write verbatim with theme styling, no word wrap
    for line in text.lines() {
      write!(w, "    {}", line.paint(theme.md_code_block))?;
      writeln!(w)?;
    }
    return Ok(());
  }

  if state.in_heading {
    // Accumulate heading text in buffer
    state.line_buf.push_str(text);
    return Ok(());
  }

  // Word-wrap and apply inline styles per word
  write_word_wrapped(w, text, theme, state)
}

fn heading_level_to_u8(level: &HeadingLevel) -> u8 {
  match level {
    HeadingLevel::H1 => 1,
    HeadingLevel::H2 => 2,
    HeadingLevel::H3 => 3,
    HeadingLevel::H4 => 4,
    HeadingLevel::H5 => 5,
    HeadingLevel::H6 => 6,
  }
}

/// Render markdown with a specific width (useful for testing).
fn render_with_width(w: &mut impl io::Write, markdown: &str, theme: &Theme, width: usize) -> io::Result<()> {
  let options = Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES;
  let parser = Parser::new_ext(markdown, options);

  let mut state = RenderState::new(width);

  for event in parser {
    match event {
      Event::Start(tag) => handle_start(w, &tag, theme, &mut state)?,
      Event::End(tag_end) => handle_end(w, &tag_end, theme, &mut state)?,
      Event::Text(text) => handle_text(w, &text, theme, &mut state)?,
      Event::Code(code) => handle_inline_code(w, &code, theme, &mut state)?,
      Event::SoftBreak => handle_soft_break(w, &mut state)?,
      Event::HardBreak => handle_hard_break(w, &mut state)?,
      Event::Rule => handle_rule(w, theme, &mut state)?,
      _ => {}
    }
  }

  // Flush any remaining text
  flush_line(w, &mut state)?;

  Ok(())
}

fn style_inline_text(text: &str, theme: &Theme, state: &RenderState) -> String {
  if state.in_bold && state.in_italic {
    text.paint(theme.md_bold).italic().to_string()
  } else if state.in_bold {
    text.paint(theme.md_bold).to_string()
  } else if state.in_italic {
    text.paint(theme.md_italic).to_string()
  } else if state.in_link {
    text.paint(theme.md_link).to_string()
  } else if state.in_blockquote {
    text.paint(theme.md_blockquote).to_string()
  } else {
    text.to_string()
  }
}

/// Get the terminal width, falling back to 80 columns.
fn terminal_width() -> usize {
  terminal_size::terminal_size().map(|(w, _)| w.0 as usize).unwrap_or(80)
}

fn write_indent(w: &mut impl io::Write, state: &RenderState) -> io::Result<()> {
  let indent = state.indent();
  if indent > 0 {
    write!(w, "{}", " ".repeat(indent))?;
  }
  Ok(())
}

fn write_line_prefix(w: &mut impl io::Write, state: &RenderState) -> io::Result<()> {
  write_indent(w, state)?;
  if state.in_blockquote {
    write!(w, "> ")?;
  }
  Ok(())
}

fn write_word_wrapped(w: &mut impl io::Write, plain: &str, theme: &Theme, state: &mut RenderState) -> io::Result<()> {
  let effective_width = state.effective_width();

  for word in plain.split_whitespace() {
    let word_len = word.len();

    if state.col > 0 && state.col + 1 + word_len > effective_width {
      // Wrap to next line
      writeln!(w)?;
      write_line_prefix(w, state)?;
      state.col = 0;
    } else if state.col > 0 {
      write!(w, " ")?;
      state.col += 1;
    } else if state.col == 0 && state.list_stack.is_empty() {
      write_line_prefix(w, state)?;
    }

    // Re-style the individual word
    let styled_word = style_inline_text(word, theme, state);
    write!(w, "{styled_word}")?;
    state.col += word_len;
  }

  state.has_content = true;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  /// Helper: render markdown with colors disabled so output is plain text.
  fn render_plain(markdown: &str, width: usize) -> String {
    // Use a guard to disable colors for this test
    let _guard = yansi::disable();
    let theme = Theme::default();
    let mut buf = Vec::new();
    render_with_width(&mut buf, markdown, &theme, width).unwrap();
    String::from_utf8(buf).unwrap()
  }

  mod blockquotes {
    use super::*;

    #[test]
    fn it_renders_blockquote() {
      let output = render_plain("> This is a quote", 80);
      assert!(
        output.contains("> This is a quote"),
        "Should contain '> This is a quote', got: {output}"
      );
    }
  }

  mod bold {
    use super::*;

    #[test]
    fn it_renders_bold_in_paragraph() {
      let output = render_plain("Hello **world** goodbye", 80);
      assert!(output.contains("Hello"), "Should contain 'Hello', got: {output}");
      assert!(output.contains("world"), "Should contain 'world', got: {output}");
      assert!(output.contains("goodbye"), "Should contain 'goodbye', got: {output}");
    }

    #[test]
    fn it_renders_bold_text() {
      let output = render_plain("This is **bold** text", 80);
      assert!(output.contains("bold"), "Should contain 'bold', got: {output}");
    }
  }

  mod code_blocks {
    use super::*;

    #[test]
    fn it_renders_code_block_with_language() {
      let input = "```rust\nlet x = 42;\n```";
      let output = render_plain(input, 80);
      assert!(
        output.contains("let x = 42"),
        "Should contain code content, got: {output}"
      );
    }

    #[test]
    fn it_renders_fenced_code_block() {
      let input = "```\nfn main() {\n    println!(\"hello\");\n}\n```";
      let output = render_plain(input, 80);
      assert!(
        output.contains("fn main()"),
        "Should contain code block content, got: {output}"
      );
      assert!(
        output.contains("    fn main()"),
        "Should indent code block, got: {output}"
      );
    }
  }

  mod combined {
    use super::*;

    #[test]
    fn it_renders_mixed_markdown() {
      let input = "# Title\n\nSome **bold** and *italic* text.\n\n- Item 1\n- Item 2\n\n> A quote\n\n---";
      let output = render_plain(input, 80);
      assert!(output.contains("# Title"), "Should contain heading");
      assert!(output.contains("bold"), "Should contain bold text");
      assert!(output.contains("italic"), "Should contain italic text");
      assert!(output.contains("- Item 1"), "Should contain list item");
      assert!(output.contains("> A quote"), "Should contain blockquote");
      assert!(output.contains("\u{2500}"), "Should contain rule");
    }
  }

  mod headings {
    use super::*;

    #[test]
    fn it_renders_h1() {
      let output = render_plain("# Hello", 80);
      assert!(output.contains("# Hello"), "Should contain '# Hello', got: {output}");
    }

    #[test]
    fn it_renders_h2() {
      let output = render_plain("## World", 80);
      assert!(output.contains("## World"), "Should contain '## World', got: {output}");
    }

    #[test]
    fn it_renders_h3() {
      let output = render_plain("### Level 3", 80);
      assert!(
        output.contains("### Level 3"),
        "Should contain '### Level 3', got: {output}"
      );
    }

    #[test]
    fn it_renders_h4() {
      let output = render_plain("#### Level 4", 80);
      assert!(
        output.contains("#### Level 4"),
        "Should contain '#### Level 4', got: {output}"
      );
    }

    #[test]
    fn it_renders_h5() {
      let output = render_plain("##### Level 5", 80);
      assert!(
        output.contains("##### Level 5"),
        "Should contain '##### Level 5', got: {output}"
      );
    }

    #[test]
    fn it_renders_h6() {
      let output = render_plain("###### Level 6", 80);
      assert!(
        output.contains("###### Level 6"),
        "Should contain '###### Level 6', got: {output}"
      );
    }
  }

  mod horizontal_rules {
    use super::*;

    #[test]
    fn it_renders_horizontal_rule() {
      let output = render_plain("---", 80);
      assert!(
        output.contains("\u{2500}"),
        "Should contain horizontal rule character, got: {output}"
      );
    }

    #[test]
    fn it_renders_rule_with_correct_length() {
      let output = render_plain("---", 80);
      let rule_line = output.lines().find(|l| l.contains('\u{2500}')).unwrap();
      let count = rule_line.chars().filter(|c| *c == '\u{2500}').count();
      assert_eq!(count, 40, "Rule should be 40 chars wide (min of width and 40)");
    }
  }

  mod inline_code {
    use super::*;

    #[test]
    fn it_renders_inline_code() {
      let output = render_plain("Use `cargo run` to build", 80);
      assert!(
        output.contains("`cargo run`"),
        "Should contain '`cargo run`', got: {output}"
      );
    }
  }

  mod italic {
    use super::*;

    #[test]
    fn it_renders_italic_text() {
      let output = render_plain("This is *italic* text", 80);
      assert!(output.contains("italic"), "Should contain 'italic', got: {output}");
    }
  }

  mod links {
    use super::*;

    #[test]
    fn it_renders_link_with_url() {
      let output = render_plain("[click here](https://example.com)", 80);
      assert!(output.contains("click here"), "Should contain link text, got: {output}");
      assert!(
        output.contains("(https://example.com)"),
        "Should contain URL in parens, got: {output}"
      );
    }
  }

  mod ordered_lists {
    use super::*;

    #[test]
    fn it_renders_ordered_list() {
      let input = "1. First\n2. Second\n3. Third";
      let output = render_plain(input, 80);
      assert!(output.contains("1. First"), "Should contain '1. First', got: {output}");
      assert!(
        output.contains("2. Second"),
        "Should contain '2. Second', got: {output}"
      );
      assert!(output.contains("3. Third"), "Should contain '3. Third', got: {output}");
    }
  }

  mod plain_output {
    use super::*;

    #[test]
    fn it_produces_no_ansi_escapes_when_disabled() {
      let _guard = yansi::disable();
      let theme = Theme::default();
      let mut buf = Vec::new();
      render_with_width(&mut buf, "# Hello **world**", &theme, 80).unwrap();
      let output = String::from_utf8(buf).unwrap();
      assert!(
        !output.contains("\x1b["),
        "Should not contain ANSI escapes when disabled, got: {output}"
      );
    }
  }

  mod unordered_lists {
    use super::*;

    #[test]
    fn it_renders_nested_unordered_list() {
      let input = "- Outer\n  - Inner";
      let output = render_plain(input, 80);
      assert!(output.contains("- Outer"), "Should contain '- Outer', got: {output}");
      assert!(
        output.contains("  - Inner"),
        "Should contain nested '  - Inner', got: {output}"
      );
    }

    #[test]
    fn it_renders_unordered_list() {
      let input = "- Item one\n- Item two\n- Item three";
      let output = render_plain(input, 80);
      assert!(
        output.contains("- Item one"),
        "Should contain '- Item one', got: {output}"
      );
      assert!(
        output.contains("- Item two"),
        "Should contain '- Item two', got: {output}"
      );
      assert!(
        output.contains("- Item three"),
        "Should contain '- Item three', got: {output}"
      );
    }
  }

  mod word_wrapping {
    use super::*;

    #[test]
    fn it_preserves_short_lines() {
      let output = render_plain("Short line", 80);
      let content_lines: Vec<&str> = output.lines().filter(|l| !l.is_empty()).collect();
      assert_eq!(content_lines.len(), 1, "Short text should be one line, got: {output}");
    }

    #[test]
    fn it_wraps_at_terminal_width() {
      let input = "This is a long paragraph that should be wrapped at the specified width for readability";
      let output = render_plain(input, 30);
      let lines: Vec<&str> = output.lines().collect();
      assert!(lines.len() > 1, "Should wrap into multiple lines, got: {output}");
      for line in &lines {
        assert!(
          line.len() <= 30,
          "Each line should be at most 30 chars, got {} chars: '{line}'",
          line.len()
        );
      }
    }
  }

  mod write_to_trait {
    use super::*;

    #[test]
    fn it_accepts_vec_writer() {
      let _guard = yansi::disable();
      let theme = Theme::default();
      let mut buf: Vec<u8> = Vec::new();
      render(&mut buf, "Hello", &theme).unwrap();
      let output = String::from_utf8(buf).unwrap();
      assert!(output.contains("Hello"), "Should write to Vec<u8>, got: {output}");
    }
  }
}

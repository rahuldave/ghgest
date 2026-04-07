//! Terminal-width-aware content region with optional padding.

use std::fmt::{self, Display, Formatter};

/// A content region that is aware of terminal width and can apply padding.
/// Used by detail views, board columns, and any full-width component.
pub struct Component {
  content: String,
  padding: usize,
  width: Option<usize>,
}

impl Component {
  /// Create a block with the given content.
  pub fn new(content: impl Display) -> Self {
    Self {
      content: content.to_string(),
      padding: 0,
      width: None,
    }
  }

  /// Set horizontal padding (applied to both left and right).
  pub fn padding(mut self, padding: usize) -> Self {
    self.padding = padding;
    self
  }

  /// Set an explicit width. If not set, uses terminal width.
  pub fn width(mut self, width: usize) -> Self {
    self.width = Some(width);
    self
  }

  fn effective_width(&self) -> usize {
    let w = self
      .width
      .unwrap_or_else(|| crate::ui::components::molecules::row::terminal_width() as usize);
    w.saturating_sub(self.padding * 2)
  }
}

impl Display for Component {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let pad = " ".repeat(self.padding);
    let _max_width = self.effective_width();

    for (i, line) in self.content.lines().enumerate() {
      if i > 0 {
        writeln!(f)?;
      }
      write!(f, "{pad}{line}")?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_renders_content_with_no_padding() {
    let block = Component::new("hello world");

    assert_eq!(block.to_string(), "hello world");
  }

  #[test]
  fn it_applies_padding_to_each_line() {
    let block = Component::new("line one\nline two").padding(2);

    assert_eq!(block.to_string(), "  line one\n  line two");
  }

  #[test]
  fn it_computes_effective_width() {
    let block = Component::new("test").width(80).padding(4);

    assert_eq!(block.effective_width(), 72);
  }
}

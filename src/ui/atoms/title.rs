use std::fmt;

use yansi::{Paint, Style};

/// A styled heading that supports truncation with ellipsis and right-padding.
pub struct Title {
  max_width: Option<usize>,
  pad_to: Option<usize>,
  style: Style,
  text: String,
}

impl Title {
  /// Create a title with the given text and style.
  pub fn new(text: impl Into<String>, style: Style) -> Self {
    Self {
      text: text.into(),
      style,
      max_width: None,
      pad_to: None,
    }
  }

  /// Truncate text exceeding `width` characters, appending an ellipsis.
  pub fn max_width(mut self, width: usize) -> Self {
    self.max_width = Some(width);
    self
  }

  /// Right-pad with spaces so the title occupies at least `width` visible columns.
  pub fn pad_to(mut self, width: usize) -> Self {
    self.pad_to = Some(width);
    self
  }
}

impl fmt::Display for Title {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut display_text = self.text.clone();

    if let Some(max) = self.max_width
      && max > 0
      && display_text.chars().count() > max
    {
      display_text = display_text.chars().take(max - 1).collect::<String>() + "…";
    }

    let visible_len = display_text.chars().count();

    let padding = if let Some(pad) = self.pad_to {
      pad.saturating_sub(visible_len)
    } else {
      0
    };

    write!(f, "{}{}", display_text.paint(self.style), " ".repeat(padding))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn plain() -> Style {
    Style::default()
  }

  mod display {
    use super::*;

    #[test]
    fn it_renders_basic_text() {
      let title = Title::new("Hello", plain());
      let output = title.to_string();
      assert!(output.contains("Hello"));
    }

    #[test]
    fn it_renders_empty_title() {
      let title = Title::new("", plain());
      let output = title.to_string();
      assert!(output.is_empty() || output.trim().is_empty());
    }
  }

  mod max_width {
    use super::*;

    #[test]
    fn it_does_not_truncate_at_exact_width() {
      let title = Title::new("ABCDE", plain()).max_width(5);
      let output = title.to_string();
      assert!(output.contains("ABCDE"));
      assert!(!output.contains("\u{2026}"));
    }

    #[test]
    fn it_truncates_one_char_over() {
      let title = Title::new("ABCDEF", plain()).max_width(5);
      let output = title.to_string();
      assert!(output.contains("ABCD\u{2026}"));
      assert!(!output.contains("ABCDE"));
    }

    #[test]
    fn it_truncates_with_ellipsis() {
      let title = Title::new("Hello World", plain()).max_width(6);
      let output = title.to_string();
      assert!(output.contains("Hello\u{2026}"));
      assert!(!output.contains("World"));
    }
  }

  mod pad_to {
    use super::*;

    #[test]
    fn it_pads_to_fixed_width() {
      let title = Title::new("Hi", plain()).pad_to(10);
      let output = title.to_string();
      assert!(output.ends_with("        "));
    }
  }
}

use std::fmt;

use yansi::{Paint, Style};

/// A styled text label with optional right-padding for column alignment.
pub struct Label {
  pad_to: usize,
  style: Style,
  text: String,
}

impl Label {
  /// Create a label with the given text and style, initially unpadded.
  pub fn new(text: impl Into<String>, style: Style) -> Self {
    Self {
      text: text.into(),
      style,
      pad_to: 0,
    }
  }

  /// Right-pad with spaces so the label occupies at least `width` visible columns.
  pub fn pad_to(mut self, width: usize) -> Self {
    self.pad_to = width;
    self
  }
}

impl fmt::Display for Label {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let padding = self.pad_to.saturating_sub(self.text.len());
    write!(f, "{}{}", self.text.paint(self.style), " ".repeat(padding))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod display {
    use super::*;

    #[test]
    fn it_pads_to_smaller_than_text_without_truncating() {
      let label = Label::new("Priority", Style::default()).pad_to(3);
      let rendered = format!("{label}");
      assert!(rendered.contains("Priority"));
    }

    #[test]
    fn it_renders_with_padding() {
      let label = Label::new("Status", Style::default()).pad_to(12);
      let rendered = format!("{label}");
      assert!(rendered.contains("Status"));
      assert!(rendered.ends_with("      "));
    }

    #[test]
    fn it_renders_without_padding() {
      let label = Label::new("Title", Style::default());
      let rendered = format!("{label}");
      assert!(rendered.contains("Title"));
      assert!(!rendered.ends_with(' '));
    }
  }
}

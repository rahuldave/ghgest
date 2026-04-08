//! Styled text label with optional right-padding for column alignment.

use std::fmt::{self, Display, Formatter};

use yansi::{Paint, Style};

/// A styled text label, typically used as the key in a key-value display.
///
/// Call [`pad_to`](Component::pad_to) to right-pad with spaces for column alignment.
pub struct Component {
  pad_to: usize,
  style: Style,
  text: String,
}

impl Component {
  /// Create a label with the given text and style, initially unpadded.
  pub fn new(text: impl Into<String>, style: Style) -> Self {
    Self {
      pad_to: 0,
      style,
      text: text.into(),
    }
  }

  /// Right-pad with spaces so the label occupies at least `width` visible columns.
  pub fn pad_to(mut self, width: usize) -> Self {
    self.pad_to = width;
    self
  }
}

impl Display for Component {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let padding = self.pad_to.saturating_sub(self.text.len());
    write!(f, "{}{}", self.text.paint(self.style), " ".repeat(padding))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn plain() -> Style {
    Style::default()
  }

  mod fmt {
    use super::*;

    #[test]
    fn it_does_not_truncate_when_pad_is_smaller_than_text() {
      let component = Component::new("Priority", plain()).pad_to(3);

      let output = component.to_string();

      assert!(output.contains("Priority"));
    }

    #[test]
    fn it_pads_to_requested_width() {
      let component = Component::new("Status", plain()).pad_to(12);

      let output = component.to_string();

      assert!(output.contains("Status"));
      assert!(output.ends_with("      "));
    }
  }
}

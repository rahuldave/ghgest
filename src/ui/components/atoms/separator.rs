use std::fmt::{self, Display, Formatter};

use yansi::{Paint, Style};

use crate::ui::components::molecules::row;

/// A horizontal separator line with optional centered label.
pub struct Component {
  ch: char,
  label: Option<String>,
  label_style: Option<Style>,
  style: Style,
  width: Option<usize>,
}

impl Component {
  /// Create a solid rule separator.
  pub fn rule(style: Style) -> Self {
    Self {
      ch: '─',
      label: None,
      label_style: None,
      style,
      width: None,
    }
  }

  /// Create a solid separator with a centered label.
  pub fn labeled(label: impl Into<String>, style: Style) -> Self {
    Self {
      ch: '─',
      label: Some(label.into()),
      label_style: None,
      style,
      width: None,
    }
  }

  /// Create a dashed separator with a centered label.
  pub fn dashed(label: impl Into<String>, style: Style) -> Self {
    Self {
      ch: '╌',
      label: Some(label.into()),
      label_style: None,
      style,
      width: None,
    }
  }

  /// Set an explicit style for the label text (defaults to the separator style).
  pub fn label_style(mut self, style: Style) -> Self {
    self.label_style = Some(style);
    self
  }

  /// Override the separator width (defaults to terminal width minus indent).
  pub fn width(mut self, width: usize) -> Self {
    self.width = Some(width);
    self
  }
}

impl Display for Component {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let width = self.width.unwrap_or_else(|| row::terminal_width() as usize);
    let ch = self.ch.to_string();

    match &self.label {
      Some(label) => {
        let label_len = label.len() + 2; // " label "
        let left = (width.saturating_sub(label_len)) / 2;
        let right = width.saturating_sub(label_len + left);
        let left_line: String = ch.repeat(left);
        let right_line: String = ch.repeat(right);
        let ls = self.label_style.unwrap_or(self.style);
        write!(
          f,
          "{} {} {}",
          left_line.paint(self.style),
          label.paint(ls),
          right_line.paint(self.style),
        )
      }
      None => {
        let line: String = ch.repeat(width);
        write!(f, "{}", line.paint(self.style))
      }
    }
  }
}

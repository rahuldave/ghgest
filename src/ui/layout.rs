//! Row and column layout containers for terminal output.

use std::fmt::{self, Display, Formatter};

use unicode_width::UnicodeWidthChar;

use super::utils;

/// A vertical stack of display items, separated by newlines.
pub struct Column {
  items: Vec<String>,
}

impl Column {
  pub fn new() -> Self {
    Self {
      items: Vec::new(),
    }
  }

  /// Append an item as the next row in the column.
  pub fn row(mut self, item: impl Display) -> Self {
    self.items.push(item.to_string());
    self
  }
}

impl Default for Column {
  fn default() -> Self {
    Self::new()
  }
}

impl Display for Column {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    for (i, item) in self.items.iter().enumerate() {
      if i > 0 {
        writeln!(f)?;
      }
      write!(f, "{item}")?;
    }
    Ok(())
  }
}

/// A horizontal sequence of display items with configurable spacing.
///
/// Columns that would exceed `max_width` are truncated with an ellipsis.
pub struct Row {
  items: Vec<String>,
  max_width: Option<u16>,
  spacing: usize,
}

impl Row {
  pub fn new() -> Self {
    Self {
      items: Vec::new(),
      spacing: 2,
      max_width: None,
    }
  }

  /// Append an item as the next column in the row.
  pub fn col(mut self, item: impl Display) -> Self {
    self.items.push(item.to_string());
    self
  }

  /// Set the maximum visible width (defaults to terminal width).
  pub fn max_width(mut self, width: u16) -> Self {
    self.max_width = Some(width);
    self
  }

  /// Set the number of space characters between columns (default 2).
  pub fn spacing(mut self, spacing: usize) -> Self {
    self.spacing = spacing;
    self
  }
}

impl Default for Row {
  fn default() -> Self {
    Self::new()
  }
}

impl Display for Row {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let max_width = self.max_width.unwrap_or_else(utils::terminal_width) as usize;
    let spacer = " ".repeat(self.spacing);

    let mut current_width: usize = 0;
    let mut first = true;

    for item in &self.items {
      let item_width = utils::display_width(item);

      if !first {
        let needed = self.spacing + item_width;
        if current_width + needed > max_width {
          let remaining = max_width.saturating_sub(current_width + self.spacing);
          if remaining > 1 {
            write!(f, "{spacer}")?;
            truncate_visible(f, item, remaining)?;
          }
          break;
        }
        write!(f, "{spacer}")?;
        current_width += self.spacing;
      }

      write!(f, "{item}")?;
      current_width += item_width;
      first = false;
    }

    Ok(())
  }
}

/// Write `s` into `f`, truncating with an ellipsis once visible width exceeds `budget`.
fn truncate_visible(f: &mut Formatter<'_>, s: &str, budget: usize) -> fmt::Result {
  let mut visible = 0;
  let mut chars = s.chars().peekable();
  let mut truncated = false;

  while let Some(c) = chars.next() {
    if c == '\x1b' {
      write!(f, "{c}")?;
      if chars.peek() == Some(&'[') {
        write!(f, "{}", chars.next().unwrap())?;
        for inner in chars.by_ref() {
          write!(f, "{inner}")?;
          if inner.is_ascii_alphabetic() {
            break;
          }
        }
      }
      continue;
    }

    let w = UnicodeWidthChar::width(c).unwrap_or(0);
    if visible + w > budget.saturating_sub(1) {
      write!(f, "\u{2026}")?;
      truncated = true;
      break;
    }
    write!(f, "{c}")?;
    visible += w;
  }

  let _ = truncated;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  mod column {
    use super::*;

    #[test]
    fn it_renders_empty() {
      let col = Column::new();
      assert_eq!(col.to_string(), "");
    }

    #[test]
    fn it_renders_newlines_between_rows() {
      let col = Column::new().row("a").row("b").row("c");
      assert_eq!(col.to_string(), "a\nb\nc");
    }

    #[test]
    fn it_renders_single_item() {
      let col = Column::new().row("only");
      assert_eq!(col.to_string(), "only");
    }
  }

  mod row {
    use super::*;

    #[test]
    fn it_renders_custom_spacing() {
      let row = Row::new().spacing(1).max_width(80).col("a").col("b").col("c");
      assert_eq!(row.to_string(), "a b c");
    }

    #[test]
    fn it_renders_empty() {
      let row = Row::new().max_width(80);
      assert_eq!(row.to_string(), "");
    }

    #[test]
    fn it_renders_single_item() {
      let row = Row::new().max_width(80).col("hello");
      assert_eq!(row.to_string(), "hello");
    }

    #[test]
    fn it_renders_three_items_with_default_spacing() {
      let row = Row::new().max_width(80).col("a").col("b").col("c");
      assert_eq!(row.to_string(), "a  b  c");
    }

    #[test]
    fn it_truncates_on_overflow() {
      let row = Row::new().max_width(10).col("aaaa").col("bbbbbb");
      let rendered = row.to_string();
      assert!(rendered.starts_with("aaaa  "));
      assert!(rendered.contains('\u{2026}'));
      let visible_width = utils::display_width(&rendered);
      assert!(visible_width <= 10, "visible width {visible_width} exceeds 10");
    }
  }
}

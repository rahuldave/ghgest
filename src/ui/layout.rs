//! Row and column layout containers for terminal output.

use std::fmt::{self, Display, Formatter};

use unicode_width::UnicodeWidthChar;

use super::utils;

/// Builder returned by [`Row::col_with`] to configure column sizing before adding the next column.
pub struct ColBuilder<'a> {
  row: &'a mut Row,
  content: String,
}

impl<'a> ColBuilder<'a> {
  /// Size to content width (default). This is a no-op that makes intent explicit.
  pub fn fixed(self) -> &'a mut Row {
    self.row.entries.push(ColEntry {
      content: self.content,
      sizing: Sizing::Fixed,
    });
    self.row
  }

  /// Always occupy exactly `width` display columns, right-padded with spaces if shorter.
  pub fn fixed_width(self, width: usize) -> &'a mut Row {
    self.row.entries.push(ColEntry {
      content: self.content,
      sizing: Sizing::FixedWidth(width),
    });
    self.row
  }

  /// Grow to fill remaining terminal width. Truncates with `…` on overflow.
  pub fn flex(self) -> &'a mut Row {
    self.row.entries.push(ColEntry {
      content: self.content,
      sizing: Sizing::Flex(None),
    });
    self.row
  }

  /// Grow to fill remaining width, but never shrink below `min` columns.
  pub fn flex_min(self, min: usize) -> &'a mut Row {
    self.row.entries.push(ColEntry {
      content: self.content,
      sizing: Sizing::Flex(Some(min)),
    });
    self.row
  }
}

/// A vertical stack of display items, separated by newlines.
pub struct Column {
  items: Vec<String>,
  max_width: Option<u16>,
}

impl Column {
  pub fn new() -> Self {
    Self {
      items: Vec::new(),
      max_width: None,
    }
  }

  /// Set the maximum width propagated to nested [`Row`]s.
  pub fn max_width(mut self, width: u16) -> Self {
    self.max_width = Some(width);
    self
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

/// A horizontal sequence of display items with configurable spacing and flex layout.
///
/// Columns that exceed available width are truncated with an ellipsis. Flex columns
/// grow to fill remaining terminal width after fixed columns are measured.
pub struct Row {
  entries: Vec<ColEntry>,
  max_width: Option<u16>,
  spacing: usize,
}

impl Row {
  pub fn new() -> Self {
    Self {
      entries: Vec::new(),
      spacing: 2,
      max_width: None,
    }
  }

  /// Append an item as the next column in the row with default (fixed) sizing.
  ///
  /// This is the simple API — the column sizes to its content width.
  /// For flex or fixed-width sizing, use [`col_with`].
  pub fn col(mut self, item: impl Display) -> Self {
    self.entries.push(ColEntry {
      content: item.to_string(),
      sizing: Sizing::Fixed,
    });
    self
  }

  /// Append a column and return a builder to configure its sizing.
  ///
  /// # Examples
  ///
  /// ```ignore
  /// let mut row = Row::new();
  /// row.col_with("title").flex();
  /// row.col_with("badge").fixed_width(6);
  /// ```
  pub fn col_with(&mut self, item: impl Display) -> ColBuilder<'_> {
    ColBuilder {
      row: self,
      content: item.to_string(),
    }
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

  fn render_flex(&self, f: &mut Formatter<'_>, max_width: usize, spacer: &str) -> fmt::Result {
    // Calculate total spacing between columns.
    let total_spacing = if self.entries.len() > 1 {
      self.spacing * (self.entries.len() - 1)
    } else {
      0
    };

    // Measure fixed columns and count flex columns.
    let mut fixed_total: usize = 0;
    let mut flex_count: usize = 0;

    for entry in &self.entries {
      match &entry.sizing {
        Sizing::Fixed => {
          fixed_total += utils::display_width(&entry.content);
        }
        Sizing::FixedWidth(w) => {
          fixed_total += w;
        }
        Sizing::Flex(_min) => {
          flex_count += 1;
        }
      }
    }

    // Distribute remaining space to flex columns.
    let used = fixed_total + total_spacing;
    let remaining = max_width.saturating_sub(used);
    let flex_each = if flex_count > 0 { remaining / flex_count } else { 0 };

    let mut first = true;
    for entry in &self.entries {
      if !first {
        write!(f, "{spacer}")?;
      }
      first = false;

      match &entry.sizing {
        Sizing::Fixed => {
          write!(f, "{}", entry.content)?;
        }
        Sizing::FixedWidth(w) => {
          let content_width = utils::display_width(&entry.content);
          write!(f, "{}", entry.content)?;
          if content_width < *w {
            write!(f, "{}", " ".repeat(w - content_width))?;
          }
        }
        Sizing::Flex(min) => {
          let budget = flex_each.max(min.unwrap_or(0));
          let content_width = utils::display_width(&entry.content);
          if content_width <= budget {
            write!(f, "{}", entry.content)?;
            if content_width < budget {
              write!(f, "{}", " ".repeat(budget - content_width))?;
            }
          } else {
            truncate_visible(f, &entry.content, budget)?;
          }
        }
      }
    }

    Ok(())
  }

  fn render_sequential(&self, f: &mut Formatter<'_>, max_width: usize, spacer: &str) -> fmt::Result {
    let mut current_width: usize = 0;
    let mut first = true;

    for entry in &self.entries {
      let col_width = match &entry.sizing {
        Sizing::Fixed => utils::display_width(&entry.content),
        Sizing::FixedWidth(w) => *w,
        Sizing::Flex(_) => unreachable!("flex columns should use render_flex"),
      };

      if !first {
        let needed = self.spacing + col_width;
        if current_width + needed > max_width {
          let remaining = max_width.saturating_sub(current_width + self.spacing);
          if remaining > 1 {
            write!(f, "{spacer}")?;
            truncate_visible(f, &entry.content, remaining)?;
          }
          break;
        }
        write!(f, "{spacer}")?;
        current_width += self.spacing;
      }

      match &entry.sizing {
        Sizing::Fixed => {
          write!(f, "{}", entry.content)?;
        }
        Sizing::FixedWidth(w) => {
          let content_width = utils::display_width(&entry.content);
          write!(f, "{}", entry.content)?;
          if content_width < *w {
            write!(f, "{}", " ".repeat(w - content_width))?;
          }
        }
        Sizing::Flex(_) => unreachable!(),
      }

      current_width += col_width;
      first = false;
    }

    Ok(())
  }
}

impl Default for Row {
  fn default() -> Self {
    Self::new()
  }
}

impl Display for Row {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    if self.entries.is_empty() {
      return Ok(());
    }

    let max_width = self.max_width.unwrap_or_else(utils::terminal_width) as usize;
    let spacer = " ".repeat(self.spacing);

    let has_flex = self.entries.iter().any(|e| matches!(e.sizing, Sizing::Flex(_)));

    if has_flex {
      self.render_flex(f, max_width, &spacer)
    } else {
      self.render_sequential(f, max_width, &spacer)
    }
  }
}

/// A single column entry within a [`Row`], pairing content with its sizing mode.
struct ColEntry {
  content: String,
  sizing: Sizing,
}

/// Sizing mode for a column within a [`Row`].
#[derive(Clone, Debug)]
enum Sizing {
  /// Occupy exactly the content's display width.
  Fixed,
  /// Always occupy exactly `n` display columns, right-padded with spaces.
  FixedWidth(usize),
  /// Grow to fill remaining terminal width. Truncates with `…` on overflow.
  /// Holds an optional minimum width.
  Flex(Option<usize>),
}

/// Write `s` into `f`, truncating with an ellipsis once visible width exceeds `budget`.
fn truncate_visible(f: &mut Formatter<'_>, s: &str, budget: usize) -> fmt::Result {
  if budget == 0 {
    return Ok(());
  }

  let mut visible = 0;
  let mut chars = s.chars().peekable();

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
      break;
    }
    write!(f, "{c}")?;
    visible += w;
  }

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

  mod flex {
    use super::*;

    #[test]
    fn it_distributes_space_to_multiple_flex_columns() {
      let mut row = Row::new().max_width(40);
      row.col_with("left content").flex();
      row.col_with("right content").flex();
      let rendered = row.to_string();
      let width = utils::display_width(&rendered);
      assert_eq!(width, 40, "should fill to max_width; got: '{rendered}'");
    }

    #[test]
    fn it_fills_remaining_space() {
      let mut row = Row::new().max_width(30).col("ID");
      row.col_with("title text").flex();
      row.col_with("badge").fixed();
      let rendered = row.to_string();
      let width = utils::display_width(&rendered);
      assert_eq!(width, 30, "should fill to max_width; got: '{rendered}'");
    }

    #[test]
    fn it_handles_empty_fixed_width_column() {
      let mut row = Row::new().max_width(80);
      row.col_with("").fixed_width(6);
      row.col_with("title").fixed();
      let rendered = row.to_string();
      assert_eq!(rendered, "        title");
    }

    #[test]
    fn it_pads_fixed_width_columns() {
      let mut row = Row::new().max_width(80);
      row.col_with("[P0]").fixed_width(6);
      row.col_with("title").fixed();
      let rendered = row.to_string();
      assert_eq!(rendered, "[P0]    title");
    }

    #[test]
    fn it_respects_flex_min_width() {
      let mut row = Row::new().max_width(15);
      row.col_with("aaaaaaaaaa").fixed();
      row.col_with("title").flex_min(10);
      let rendered = row.to_string();
      let parts: Vec<&str> = rendered.splitn(2, "  ").collect();
      assert_eq!(parts.len(), 2);
      let flex_part_width = utils::display_width(parts[1]);
      assert!(
        flex_part_width >= 10,
        "flex column width {flex_part_width} should be at least 10"
      );
    }

    #[test]
    fn it_truncates_flex_column_when_content_exceeds_budget() {
      let mut row = Row::new().max_width(20).col("ID");
      row.col_with("this is a very long title that overflows").flex();
      let rendered = row.to_string();
      assert!(rendered.contains('\u{2026}'), "should truncate with ellipsis");
      let width = utils::display_width(&rendered);
      assert!(width <= 20, "visible width {width} exceeds 20");
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

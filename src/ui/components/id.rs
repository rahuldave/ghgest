use std::fmt;

use yansi::Paint;

use crate::{model, ui::theme::Theme};

const MIN_DISPLAY_LEN: usize = 8;

/// Atomic component for rendering an entity ID.
///
/// Enforces display invariants:
/// - Always renders at least [`MIN_DISPLAY_LEN`] visible characters
/// - Shortest unique prefix colored with `theme.id_prefix`
/// - Remainder colored with `theme.id_rest`
pub struct Id<'a> {
  id: &'a model::Id,
  prefix_len: usize,
  theme: &'a Theme,
}

impl<'a> Id<'a> {
  pub fn new(id: &'a model::Id, prefix_len: usize, theme: &'a Theme) -> Self {
    Self {
      id,
      prefix_len,
      theme,
    }
  }
}

impl fmt::Display for Id<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let full = self.id.to_string();
    let display_len = MIN_DISPLAY_LEN.min(full.len());
    let s = &full[..display_len];
    let split = self.prefix_len.min(s.len());
    let (prefix, rest) = s.split_at(split);
    write!(
      f,
      "{}{}",
      prefix.paint(self.theme.id_prefix),
      rest.paint(self.theme.id_rest)
    )
  }
}

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;

  use super::*;
  use crate::ui::utils::display_width;

  fn test_id() -> model::Id {
    "zyxwvutsrqponmlkzyxwvutsrqponmlk".parse().unwrap()
  }

  #[test]
  fn it_always_renders_minimum_8_characters() {
    let id = test_id();
    let theme = Theme::default();
    let component = Id::new(&id, 3, &theme);
    let rendered = component.to_string();
    assert_eq!(display_width(&rendered), 8);
  }

  #[test]
  fn it_highlights_shortest_prefix() {
    let id = test_id();
    let theme = Theme::default();
    let component = Id::new(&id, 3, &theme);
    let rendered = component.to_string();
    assert!(rendered.contains("zyx"), "Should contain the prefix text");
    assert!(rendered.contains("wvuts"), "Should contain the remainder text");
  }

  #[test]
  fn it_handles_prefix_len_zero() {
    let id = test_id();
    let theme = Theme::default();
    let component = Id::new(&id, 0, &theme);
    let rendered = component.to_string();
    assert_eq!(display_width(&rendered), 8);
    assert!(
      rendered.contains("zyxwvuts"),
      "Should contain 8-char ID with all chars as rest"
    );
  }

  #[test]
  fn it_handles_prefix_len_exceeding_display() {
    let id = test_id();
    let theme = Theme::default();
    let component = Id::new(&id, 100, &theme);
    let rendered = component.to_string();
    assert_eq!(display_width(&rendered), 8);
    assert!(
      rendered.contains("zyxwvuts"),
      "Should contain 8-char ID with all chars as prefix"
    );
  }

  #[test]
  fn it_handles_prefix_len_equal_to_display() {
    let id = test_id();
    let theme = Theme::default();
    let component = Id::new(&id, 8, &theme);
    let rendered = component.to_string();
    assert_eq!(display_width(&rendered), 8);
  }
}

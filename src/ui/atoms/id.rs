use std::fmt;

use yansi::Paint;

use crate::ui::theme::Theme;

/// Displays a truncated, two-tone entity identifier (highlighted prefix + dimmed suffix).
pub struct Id<'a> {
  prefix_len: usize,
  theme: &'a Theme,
  value: &'a str,
}

impl<'a> Id<'a> {
  /// Create an id display, showing at most 8 characters with a 2-char highlighted prefix.
  pub fn new(value: &'a str, theme: &'a Theme) -> Self {
    Self {
      value,
      prefix_len: 2,
      theme,
    }
  }

  /// Override the number of highlighted prefix characters (clamped to value length).
  pub fn prefix_len(mut self, len: usize) -> Self {
    self.prefix_len = len;
    self
  }
}

impl fmt::Display for Id<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let display: String = self.value.chars().take(8).collect();
    let prefix_len = self.prefix_len.min(display.len());
    let (prefix, rest) = display.split_at(prefix_len);

    write!(
      f,
      "{}{}",
      prefix.paint(self.theme.id_prefix),
      rest.paint(self.theme.id_rest),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn theme() -> Theme {
    Theme::default()
  }

  mod display {
    use super::*;

    #[test]
    fn it_contains_correct_characters() {
      let theme = theme();
      let id = Id::new("xqnuktro", &theme);
      let rendered = format!("{id}");
      assert!(rendered.contains("xq"));
      assert!(rendered.contains("nuktro"));
    }

    #[test]
    fn it_renders_8_char_id_with_default_prefix() {
      let theme = theme();
      let id = Id::new("abcdefghijklmnop", &theme);
      let rendered = format!("{id}");

      assert!(rendered.contains("ab"));
      assert!(rendered.contains("cdefgh"));
      assert!(!rendered.contains("ijklmnop"));
    }

    #[test]
    fn it_renders_short_id_without_padding() {
      let theme = theme();
      let id = Id::new("abc", &theme);
      let rendered = format!("{id}");
      assert!(rendered.contains("ab"));
      assert!(rendered.contains("c"));
      assert!(!rendered.contains(' '));
    }
  }

  mod prefix_len {
    use super::*;

    #[test]
    fn it_clamps_to_value_length() {
      let theme = theme();
      let id = Id::new("ab", &theme).prefix_len(10);
      let rendered = format!("{id}");
      assert!(rendered.contains("ab"));
    }

    #[test]
    fn it_uses_custom_prefix_length() {
      let theme = theme();
      let id = Id::new("abcdefgh", &theme).prefix_len(4);
      let rendered = format!("{id}");
      assert!(rendered.contains("abcd"));
      assert!(rendered.contains("efgh"));
    }
  }
}

use std::fmt;

use yansi::Paint;

use crate::ui::theme::Theme;

/// Atomic component for rendering tags with `@` prefix in theme style.
///
/// Each tag is rendered as `@tag` with the theme's tag color, separated by spaces.
pub struct Tags<'a> {
  tags: &'a [String],
  theme: &'a Theme,
}

impl<'a> Tags<'a> {
  pub fn new(tags: &'a [String], theme: &'a Theme) -> Self {
    Self {
      tags,
      theme,
    }
  }
}

impl fmt::Display for Tags<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let formatted: Vec<String> = self.tags.iter().map(|t| format!("@{t}").paint(self.theme.tag).to_string()).collect();
    write!(f, "{}", formatted.join(" "))
  }
}

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;

  use super::*;

  #[test]
  fn it_renders_empty_when_no_tags() {
    let tags: Vec<String> = vec![];
    let theme = Theme::default();
    let component = Tags::new(&tags, &theme);
    assert_eq!(component.to_string(), "");
  }

  #[test]
  fn it_renders_single_tag_with_at_prefix() {
    yansi::disable();
    let tags = vec!["rust".to_string()];
    let theme = Theme::default();
    let component = Tags::new(&tags, &theme);
    assert_eq!(component.to_string(), "@rust");
  }

  #[test]
  fn it_renders_multiple_tags_separated_by_spaces() {
    yansi::disable();
    let tags = vec!["rust".to_string(), "cli".to_string()];
    let theme = Theme::default();
    let component = Tags::new(&tags, &theme);
    assert_eq!(component.to_string(), "@rust @cli");
  }
}

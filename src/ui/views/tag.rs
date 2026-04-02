use std::fmt::{self, Display, Formatter};

use crate::ui::{atoms::tag::Tag, composites::grouped_list::GroupedList, theming::theme::Theme};

/// Renders a themed list of tags with a summary header.
pub struct TagListView<'a> {
  tags: Vec<String>,
  theme: &'a Theme,
}

impl<'a> TagListView<'a> {
  pub fn new(tags: Vec<String>, theme: &'a Theme) -> Self {
    Self {
      tags,
      theme,
    }
  }
}

impl Display for TagListView<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let count = self.tags.len();
    let summary = format!("{count} tag{}", if count == 1 { "" } else { "s" });

    let rows: Vec<String> = self
      .tags
      .iter()
      .map(|name| Tag::new(name, self.theme.tag).to_string())
      .collect();

    let list = GroupedList::new("tags", summary, self.theme).rows(rows);
    write!(f, "{list}")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn theme() -> Theme {
    Theme::default()
  }

  #[test]
  fn it_renders_heading_and_summary() {
    yansi::disable();
    let theme = theme();
    let view = TagListView::new(vec!["bug".into(), "ui".into()], &theme);
    let output = view.to_string();

    assert!(output.contains("tags"), "should contain heading");
    assert!(output.contains("2 tags"), "should contain count");
  }

  #[test]
  fn it_renders_singular_tag_count() {
    yansi::disable();
    let theme = theme();
    let view = TagListView::new(vec!["solo".into()], &theme);
    let output = view.to_string();

    assert!(output.contains("1 tag"), "should use singular");
    assert!(!output.contains("1 tags"), "should not use plural for count of 1");
  }

  #[test]
  fn it_renders_tags_with_hash_prefix() {
    yansi::disable();
    let theme = theme();
    let view = TagListView::new(vec!["bug".into(), "ui".into()], &theme);
    let output = view.to_string();

    assert!(output.contains("#bug"), "should contain #bug");
    assert!(output.contains("#ui"), "should contain #ui");
  }
}

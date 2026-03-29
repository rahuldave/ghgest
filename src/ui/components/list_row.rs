use super::{Id, Tags, Title};
use crate::{model, ui::theme::Theme};

/// Composite component that builds a table row from atomic components.
///
/// Composes [`Id`], [`Title`], and [`Tags`] into a `Vec<String>` suitable
/// for table rendering. Enforces column order: `[id, title, tags, ...extras]`.
/// The tags column is always present (empty string when no tags).
pub struct ListRow<'a> {
  id: &'a model::Id,
  prefix_len: usize,
  title: &'a str,
  tags: &'a [String],
  theme: &'a Theme,
  extras: Vec<String>,
}

impl<'a> ListRow<'a> {
  pub fn new(id: &'a model::Id, prefix_len: usize, title: &'a str, tags: &'a [String], theme: &'a Theme) -> Self {
    Self {
      id,
      prefix_len,
      title,
      tags,
      theme,
      extras: Vec::new(),
    }
  }

  /// Append an extra column value after the tags column.
  pub fn extra(mut self, value: String) -> Self {
    self.extras.push(value);
    self
  }

  /// Build the row as a `Vec<String>` with column order: id, title, tags, then extras.
  pub fn build(&self) -> Vec<String> {
    let id_cell = Id::new(self.id, self.prefix_len, self.theme).to_string();
    let title_cell = Title::new(self.title).to_string();
    let tags_cell = Tags::new(self.tags, self.theme).to_string();

    let mut row = vec![id_cell, title_cell, tags_cell];
    row.extend(self.extras.iter().cloned());
    row
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
  fn it_builds_a_basic_row() {
    let id = test_id();
    let theme = Theme::default();
    let row = ListRow::new(&id, 3, "My Task", &[], &theme).build();

    assert_eq!(row.len(), 3);
    assert_eq!(display_width(&row[0]), 8);
    assert_eq!(row[1], "My Task");
    assert_eq!(row[2], "");
  }

  #[test]
  fn it_builds_a_row_with_tags() {
    yansi::disable();
    let id = test_id();
    let theme = Theme::default();
    let tags = vec!["bug".to_string(), "urgent".to_string()];
    let row = ListRow::new(&id, 3, "Tagged Task", &tags, &theme).build();

    assert_eq!(row.len(), 3);
    assert_eq!(row[1], "Tagged Task");
    assert_eq!(row[2], "@bug @urgent");
  }

  #[test]
  fn it_builds_a_row_without_tags_as_empty_column() {
    let id = test_id();
    let theme = Theme::default();
    let row = ListRow::new(&id, 3, "No Tags", &[], &theme).build();

    assert_eq!(row.len(), 3);
    assert_eq!(row[2], "", "Tags column should be empty string when no tags");
  }

  #[test]
  fn it_builds_a_row_with_extras() {
    let id = test_id();
    let theme = Theme::default();
    let row = ListRow::new(&id, 3, "With Extras", &[], &theme)
      .extra("indicator1".to_string())
      .extra("5 tasks".to_string())
      .build();

    assert_eq!(row.len(), 5);
    assert_eq!(row[1], "With Extras");
    assert_eq!(row[2], "");
    assert_eq!(row[3], "indicator1");
    assert_eq!(row[4], "5 tasks");
  }

  #[test]
  fn it_enforces_column_order_id_title_tags_extras() {
    let id = test_id();
    let theme = Theme::default();
    let tags = vec!["cli".to_string()];
    let row = ListRow::new(&id, 3, "Ordered", &tags, &theme)
      .extra("extra1".to_string())
      .build();

    assert_eq!(row.len(), 4);
    // Column 0: id (has ANSI, check width)
    assert_eq!(display_width(&row[0]), 8);
    // Column 1: title
    assert_eq!(row[1], "Ordered");
    // Column 2: tags (has ANSI content)
    assert!(row[2].contains("@cli"));
    // Column 3: extra
    assert_eq!(row[3], "extra1");
  }
}

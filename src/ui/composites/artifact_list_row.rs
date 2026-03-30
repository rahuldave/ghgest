use std::fmt;

use crate::ui::{
  atoms::{badge::Badge, id::Id, tag::Tags, title::Title},
  theme::Theme,
};

/// A single row in an artifact list, showing id, title, tags, and optional archived badge.
pub struct ArtifactListRow<'a> {
  id: &'a str,
  is_archived: bool,
  tags: &'a [String],
  theme: &'a Theme,
  title_text: &'a str,
}

impl<'a> ArtifactListRow<'a> {
  pub fn new(id: &'a str, title_text: &'a str, tags: &'a [String], theme: &'a Theme) -> Self {
    Self {
      id,
      title_text,
      tags,
      is_archived: false,
      theme,
    }
  }

  /// Marks this row as archived, applying dimmed styles and appending an `[archived]` badge.
  pub fn archived(mut self, v: bool) -> Self {
    self.is_archived = v;
    self
  }
}

impl fmt::Display for ArtifactListRow<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let id = Id::new(self.id, self.theme);

    let title_style = if self.is_archived {
      self.theme.artifact_list_title_archived
    } else {
      self.theme.artifact_list_title
    };
    let title = Title::new(self.title_text, title_style).pad_to(23);

    let tag_style = if self.is_archived {
      self.theme.artifact_list_tag_archived
    } else {
      self.theme.tag
    };
    let tags = Tags::new(self.tags, tag_style);

    write!(f, "{id}  {title}")?;

    if !self.tags.is_empty() {
      write!(f, "  {tags}")?;
    }

    if self.is_archived {
      let badge = Badge::new("[archived]", self.theme.artifact_list_archived_badge);
      write!(f, "  {badge}")?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn theme() -> Theme {
    Theme::default()
  }

  #[test]
  fn it_renders_archived_row_with_badge() {
    yansi::disable();
    let theme = theme();
    let tags = vec!["spec".to_string(), "backend".to_string()];
    let row = ArtifactListRow::new("pkzqwrnd", "auth-spec", &tags, &theme).archived(true);
    let rendered = row.to_string();

    assert!(rendered.contains("pkzqwrnd"));
    assert!(rendered.contains("auth-spec"));
    assert!(rendered.contains("#spec"));
    assert!(rendered.contains("#backend"));
    assert!(rendered.contains("[archived]"));
  }

  #[test]
  fn it_renders_row_with_multiple_tags() {
    yansi::disable();
    let theme = theme();
    let tags = vec!["spec".to_string(), "backend".to_string(), "v2".to_string()];
    let row = ArtifactListRow::new("abcdefgh", "my-artifact", &tags, &theme);
    let rendered = row.to_string();

    assert!(rendered.contains("#spec"));
    assert!(rendered.contains("#backend"));
    assert!(rendered.contains("#v2"));
    assert!(rendered.contains("#spec  #backend  #v2"));
  }

  #[test]
  fn it_renders_row_with_no_tags() {
    yansi::disable();
    let theme = theme();
    let row = ArtifactListRow::new("abcdefgh", "bare-artifact", &[], &theme);
    let rendered = row.to_string();

    assert!(rendered.contains("abcdefgh"));
    assert!(rendered.contains("bare-artifact"));
    assert!(!rendered.contains('#'));
  }

  #[test]
  fn it_renders_normal_row_with_id_title_and_tags() {
    yansi::disable();
    let theme = theme();
    let tags = vec!["schema".to_string()];
    let row = ArtifactListRow::new("fsahdqlt", "probe-schema-v2", &tags, &theme);
    let rendered = row.to_string();

    assert!(rendered.contains("fsahdqlt"));
    assert!(rendered.contains("probe-schema-v2"));
    assert!(rendered.contains("#schema"));
    assert!(!rendered.contains("[archived]"));
  }
}

use std::fmt::{self, Display, Formatter};

use crate::ui::components::{
  atoms::{Badge, Column, Icon, Id, Tag, Title},
  molecules::{Indicators, Row, StatusBadge, row},
};

/// Max display width for task titles in list rows.
const TITLE_PAD: usize = 35;

/// A single row in a task list, showing status icon, id, priority, title, status badge, and indicators.
pub struct Component<'a> {
  blocked_by: Option<&'a str>,
  blocking: bool,
  blocking_pad: usize,
  id: &'a str,
  id_prefix_len: usize,
  priority: Option<u8>,
  priority_pad: usize,
  status: &'a str,
  status_pad: usize,
  tags: &'a [String],
  title_text: &'a str,
}

impl<'a> Component<'a> {
  pub fn new(status: &'a str, id: &'a str, title_text: &'a str) -> Self {
    Self {
      blocked_by: None,
      blocking: false,
      blocking_pad: 0,
      id,
      id_prefix_len: 2,
      priority: None,
      priority_pad: 0,
      status,
      status_pad: 0,
      tags: &[],
      title_text,
    }
  }

  /// Sets the ID of the task blocking this one.
  pub fn blocked_by(mut self, id: Option<&'a str>) -> Self {
    self.blocked_by = id;
    self
  }

  /// Marks this task as blocking other tasks.
  pub fn blocking(mut self, b: bool) -> Self {
    self.blocking = b;
    self
  }

  /// Returns the rendered blocking info string for this row.
  ///
  /// The indicator IDs reuse this row's `id_prefix_len` so the highlighted
  /// prefix matches the task's own pool (active vs all). Blocked-by IDs may
  /// belong to a different pool, but for simplicity we use the row task's
  /// pool — see `task::show` for the rationale.
  pub fn blocking_info_string(&self) -> String {
    let blocked_by = self.blocked_by.into_iter().collect();
    Indicators::new()
      .blocking(self.blocking)
      .blocked_by(blocked_by)
      .prefix_len(self.id_prefix_len)
      .to_string()
  }

  /// Returns the display width of the blocking info for this row.
  pub fn blocking_info_width(&self) -> usize {
    row::display_width(&self.blocking_info_string())
  }

  /// Sets minimum display width for the blocking info column.
  pub fn blocking_pad(mut self, w: usize) -> Self {
    self.blocking_pad = w;
    self
  }

  /// Sets the number of highlighted prefix characters in the ID.
  pub fn id_prefix_len(mut self, len: usize) -> Self {
    self.id_prefix_len = len;
    self
  }

  /// Sets the task priority level for the badge.
  pub fn priority(mut self, p: Option<u8>) -> Self {
    self.priority = p;
    self
  }

  /// Returns the rendered priority badge string for this row.
  pub fn priority_badge_string(&self) -> String {
    match self.priority {
      Some(p) => Badge::new(format!("[P{p}]"), *crate::ui::style::global().task_list_priority()).to_string(),
      None => String::new(),
    }
  }

  /// Returns the display width of the priority badge for this row.
  pub fn priority_badge_width(&self) -> usize {
    row::display_width(&self.priority_badge_string())
  }

  /// Sets minimum display width for the priority badge column.
  pub fn priority_pad(mut self, w: usize) -> Self {
    self.priority_pad = w;
    self
  }

  /// Returns the rendered status badge string for this row.
  pub fn status_badge_string(&self) -> String {
    self.status_badge().to_string()
  }

  /// Returns the display width of the status badge for this row.
  pub fn status_badge_width(&self) -> usize {
    row::display_width(&self.status_badge_string())
  }

  /// Sets minimum display width for the status badge column.
  pub fn status_pad(mut self, w: usize) -> Self {
    self.status_pad = w;
    self
  }

  /// Sets the tags to append after the status badge.
  pub fn tags(mut self, t: &'a [String]) -> Self {
    self.tags = t;
    self
  }

  fn leading_icon(&self) -> Icon {
    if self.blocked_by.is_some() {
      Icon::blocked()
    } else {
      Icon::status(self.status)
    }
  }

  fn status_badge(&self) -> StatusBadge<'_> {
    let status = if self.blocked_by.is_some() {
      "blocked"
    } else {
      self.status
    };
    StatusBadge::new(status)
  }

  fn title(&self) -> Title {
    let theme = crate::ui::style::global();
    let style = if self.status == "cancelled" {
      *theme.task_list_title_cancelled()
    } else {
      *theme.task_list_title()
    };
    Title::new(self.title_text, style)
      .max_width(TITLE_PAD)
      .pad_to(TITLE_PAD)
  }
}

impl Display for Component<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let mut row = Row::new().spacing(2);

    row = row.col(Column::natural(self.leading_icon()));
    row = row.col(Column::natural(Id::new(self.id).prefix_len(self.id_prefix_len)));

    let priority_str = self.priority_badge_string();
    if self.priority_pad > 0 {
      row = row.col(Column::fixed(self.priority_pad, priority_str));
    } else if !priority_str.is_empty() {
      row = row.col(Column::natural(priority_str));
    }

    row = row.col(Column::natural(self.title()));

    let status_str = self.status_badge_string();
    if self.status_pad > 0 {
      row = row.col(Column::fixed(self.status_pad, status_str));
    } else {
      row = row.col(Column::natural(status_str));
    }

    let blocking_str = self.blocking_info_string();
    if self.blocking_pad > 0 {
      row = row.col(Column::fixed(self.blocking_pad, blocking_str));
    } else if !blocking_str.is_empty() {
      row = row.col(Column::natural(blocking_str));
    }

    if !self.tags.is_empty() {
      row = row.col(Column::natural(Tag::new(
        self.tags.to_vec(),
        *crate::ui::style::global().tag(),
      )));
    }

    write!(f, "{row}")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn render(row: &Component) -> String {
    yansi::disable();
    let out = row.to_string();
    yansi::enable();
    out
  }

  #[test]
  fn it_appends_tags_when_present() {
    let tags = vec!["urgent".to_string(), "backend".to_string()];
    let row = Component::new("open", "qtsdwcaz", "probe dedup").tags(&tags);
    let output = render(&row);

    assert!(output.contains("#urgent"), "should contain first tag");
    assert!(output.contains("#backend"), "should contain second tag");
  }

  #[test]
  fn it_omits_priority_badge_when_none() {
    let row = Component::new("open", "qtsdwcaz", "probe dedup by content hash");
    let output = render(&row);

    assert!(!output.contains("[P"), "should not contain priority badge");
  }

  #[test]
  fn it_omits_priority_column_when_pad_is_zero() {
    let row = Component::new("open", "aaaaaaaa", "no priority");
    let output = render(&row);

    assert_eq!(row.priority_badge_width(), 0, "width should be zero with no priority");
    assert!(!output.contains("  [P"), "should not contain priority column");
  }

  #[test]
  fn it_renders_blocked_row_with_blocked_by() {
    let row = Component::new("open", "mxdtqrbn", "ctx window").blocked_by(Some("hpvrlbme"));
    let output = render(&row);

    assert!(output.contains("blocked"), "should show blocked status");
    assert!(output.contains("blocked-by"), "should show blocked-by label");
    assert!(output.contains("hpvrlbme"), "should show blocking task id");
    assert!(output.contains('\u{2297}'), "should use blocked icon");
  }

  #[test]
  fn it_renders_cancelled_row_with_dim_style() {
    let row = Component::new("cancelled", "bsylatpq", "redis cache layer");
    let output = render(&row);

    assert!(output.contains("bsylatpq"));
    assert!(output.contains("redis cache layer"));
    assert!(output.contains("cancelled"));
  }

  #[test]
  fn it_renders_done_row_with_id_and_status() {
    let row = Component::new("done", "cdrzjvwk", "sqlite storage backend").priority(Some(0));
    let output = render(&row);

    assert!(output.contains("cdrzjvwk"), "should contain the task id");
    assert!(output.contains("[P0]"), "should contain priority badge");
    assert!(output.contains("sqlite storage backend"), "should contain title");
    assert!(output.contains("done"), "should contain status text");
  }

  #[test]
  fn it_renders_in_progress_row() {
    let row = Component::new("in-progress", "nfkbqmrx", "openai streaming adapter").priority(Some(1));
    let output = render(&row);

    assert!(output.contains("nfkbqmrx"));
    assert!(output.contains("[P1]"));
    assert!(output.contains("in progress"));
  }

  #[test]
  fn it_reserves_priority_slot_when_padded() {
    let with_priority = Component::new("open", "aaaaaaaa", "has priority")
      .priority(Some(1))
      .priority_pad(4);
    let without_priority = Component::new("open", "bbbbbbbb", "no priority").priority_pad(4);

    let out_with = render(&with_priority);
    let out_without = render(&without_priority);

    assert!(out_with.contains("[P1]"), "should contain priority badge");
    assert!(!out_without.contains("[P"), "should not contain priority badge");

    let title_pos_with = out_with.find("has priority").unwrap();
    let title_pos_without = out_without.find("no priority").unwrap();
    assert_eq!(
      title_pos_with, title_pos_without,
      "titles should align when priority_pad is set"
    );
  }

  #[test]
  fn it_shows_blocking_indicator() {
    let row = Component::new("done", "hpvrlbme", "finalize probe schema v2")
      .priority(Some(0))
      .blocking(true);
    let output = render(&row);

    assert!(output.contains("blocking"), "should show blocking badge");
    assert!(output.contains('!'), "should contain blocking icon");
  }
}

//! Full task list view with Grid-aligned columns and count summary.

use std::fmt::{self, Display, Formatter};

use crate::ui::components::{
  atoms::{Badge, Column, Icon, Id, Tag, Title},
  molecules::{EmptyList, Grid, GroupedList, Row, StatusBadge},
};

/// A single task entry for the list view.
pub struct TaskEntry {
  pub blocked_by: Option<String>,
  pub blocking: bool,
  pub id: String,
  pub priority: Option<u8>,
  pub status: String,
  pub tags: Vec<String>,
  pub title: String,
}

/// Full task list view using Grid for column alignment.
pub struct Component {
  entries: Vec<TaskEntry>,
  prefix_len: usize,
}

impl Component {
  pub fn new(entries: Vec<TaskEntry>, prefix_len: usize) -> Self {
    Self {
      entries,
      prefix_len,
    }
  }
}

impl Display for Component {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    if self.entries.is_empty() {
      return write!(f, "{}", EmptyList::new("tasks"));
    }

    let theme = crate::ui::style::global();
    let count = self.entries.len();
    let summary = format!("{count} {}", if count == 1 { "task" } else { "tasks" });
    let mut grid = Grid::new().spacing(2);

    for entry in &self.entries {
      let icon = if entry.blocked_by.is_some() {
        Icon::blocked()
      } else {
        Icon::status(&entry.status)
      };

      let id = Id::new(&entry.id).prefix_len(self.prefix_len);

      let priority_str = match entry.priority {
        Some(p) => Badge::new(format!("[P{p}]"), *theme.task_list_priority()).to_string(),
        None => String::new(),
      };

      let title_style = if entry.status == "cancelled" {
        *theme.task_list_title_cancelled()
      } else {
        *theme.task_list_title()
      };
      let title = Title::new(&entry.title, title_style);

      let badge_status = if entry.blocked_by.is_some() {
        "blocked"
      } else {
        &entry.status
      };
      let status_badge = StatusBadge::new(badge_status);

      let tag_str = if !entry.tags.is_empty() {
        Tag::new(entry.tags.clone(), *theme.tag()).to_string()
      } else {
        String::new()
      };

      let mut row = Row::new()
        .col(Column::natural(icon))
        .col(Column::natural(id))
        .col(Column::natural(priority_str))
        .col(Column::natural(title))
        .col(Column::natural(status_badge));

      if !tag_str.is_empty() {
        row = row.col(Column::natural(tag_str));
      }

      grid.push(row);
    }

    let list = GroupedList::new("tasks", summary).row(grid.to_string());
    write!(f, "{list}")
  }
}

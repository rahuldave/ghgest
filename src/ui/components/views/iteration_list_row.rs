use std::fmt::{self, Display, Formatter};

use yansi::Paint;

use super::super::atoms::{Id, Title};
use crate::ui::style;

/// A single row in an iteration list, showing id, title, and phase/task counts.
pub struct Component {
  id: String,
  id_prefix_len: usize,
  phase_count: usize,
  task_count: usize,
  title: String,
}

impl Component {
  pub fn new(id: impl Into<String>, title: impl Into<String>, phase_count: usize, task_count: usize) -> Self {
    Self {
      id: id.into(),
      id_prefix_len: 2,
      phase_count,
      task_count,
      title: title.into(),
    }
  }

  /// Sets the number of highlighted prefix characters in the ID.
  pub fn id_prefix_len(mut self, len: usize) -> Self {
    self.id_prefix_len = len;
    self
  }
}

impl Display for Component {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let theme = style::global();
    let id = Id::new(&self.id).prefix_len(self.id_prefix_len);
    let title = Title::new(&self.title, *theme.iteration_list_title()).pad_to(30);
    let summary = format!(
      "{} {} \u{00b7} {} {}",
      self.phase_count,
      if self.phase_count == 1 { "phase" } else { "phases" },
      self.task_count,
      if self.task_count == 1 { "task" } else { "tasks" },
    );

    write!(
      f,
      "{}  {}  {}",
      id,
      title,
      summary.paint(*theme.iteration_list_summary()),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_renders_id() {
    yansi::disable();
    let row = Component::new("q1ebvmxp", "Q1 LLM Benchmark", 3, 7);
    let rendered = format!("{row}");

    assert!(rendered.contains("q1"));
    assert!(rendered.contains("ebvmxp"));
  }

  #[test]
  fn it_renders_summary_with_plural_forms() {
    yansi::disable();
    let row = Component::new("q1ebvmxp", "Q1 LLM Benchmark", 3, 7);
    let rendered = format!("{row}");

    assert!(rendered.contains("3 phases"));
    assert!(rendered.contains("7 tasks"));
    assert!(rendered.contains("\u{00b7}"));
  }

  #[test]
  fn it_renders_summary_with_singular_forms() {
    yansi::disable();
    let row = Component::new("abcdefgh", "Solo Run", 1, 1);
    let rendered = format!("{row}");

    assert!(rendered.contains("1 phase"));
    assert!(rendered.contains("1 task"));
    assert!(!rendered.contains("phases"));
    assert!(!rendered.contains("tasks"));
  }

  #[test]
  fn it_renders_title() {
    yansi::disable();
    let row = Component::new("q1ebvmxp", "Q1 LLM Benchmark", 3, 7);
    let rendered = format!("{row}");

    assert!(rendered.contains("Q1 LLM Benchmark"));
  }

  #[test]
  fn it_renders_zero_counts() {
    yansi::disable();
    let row = Component::new("zerotest", "Empty Iteration", 0, 0);
    let rendered = format!("{row}");

    assert!(rendered.contains("0 phases"));
    assert!(rendered.contains("0 tasks"));
  }
}

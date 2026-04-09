//! Phased dependency graph with box-drawing connectors.
//!
//! Composes the [`PhaseHeader`] and [`TaskRow`] molecules with inline
//! branch-connector helpers to render an iteration as a phased task graph.

use std::{
  collections::BTreeMap,
  fmt::{self, Display, Formatter},
};

use yansi::Paint;

use crate::ui::components::molecules::{PhaseHeader, TaskRow, priority_pad_width};

const BRANCH_CLOSE_LAST: &str = "\u{2570}";
const BRANCH_CLOSE_MID: &str = "\u{251C}";
const BRANCH_DASH: &str = "\u{2500}";
const BRANCH_OPEN: &str = "\u{251C}";
const BRANCH_OPEN_TIP: &str = "\u{256E}";
const CLOSE_TIP: &str = "\u{256F}";
const CONTINUATION: &str = "\u{2502}";

/// Phased dependency graph with box-drawing connectors.
pub struct Component {
  iteration_title: String,
  prefix_len: usize,
  tasks: Vec<GraphTask>,
}

/// A task entry for the graph.
pub struct GraphTask {
  /// Short ids of tasks that block this task.
  pub blocked_by: Vec<String>,
  /// Short ID used to render the task's highlighted two-tone identifier.
  pub id_short: String,
  /// True when this task blocks at least one other task.
  pub is_blocking: bool,
  /// Phase number used to group tasks under phase headers.
  pub phase: u32,
  /// Task priority used to render the `[Pn]` badge.
  pub priority: Option<u8>,
  /// Task status, used to select the row icon and color.
  pub status: String,
  /// Task title rendered inside the row.
  pub title: String,
}

impl Component {
  /// Create a graph view for the given iteration title and task list.
  pub fn new(iteration_title: impl Into<String>, tasks: Vec<GraphTask>) -> Self {
    Self {
      iteration_title: iteration_title.into(),
      prefix_len: 2,
      tasks,
    }
  }

  /// Sets the highlighted prefix length passed to rendered task IDs.
  pub fn prefix_len(mut self, len: usize) -> Self {
    self.prefix_len = len;
    self
  }
}

impl Display for Component {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let theme = crate::ui::style::global();

    writeln!(f)?;
    writeln!(f, "  {}", self.iteration_title.paint(*theme.iteration_graph_title()))?;

    if self.tasks.is_empty() {
      writeln!(f)?;
      writeln!(f, "  no tasks in this iteration")?;
      return Ok(());
    }

    let mut phases: BTreeMap<u32, Vec<&GraphTask>> = BTreeMap::new();
    for task in &self.tasks {
      phases.entry(task.phase).or_default().push(task);
    }

    let phase_count = phases.len();
    let task_count = self.tasks.len();
    let phase_word = if phase_count == 1 { "phase" } else { "phases" };
    let task_word = if task_count == 1 { "task" } else { "tasks" };
    writeln!(
      f,
      "  {}",
      format!("{phase_count} {phase_word} \u{00B7} {task_count} {task_word}").paint(*theme.list_summary())
    )?;
    writeln!(f)?;

    for (i, (phase, phase_tasks)) in phases.iter().enumerate() {
      let is_last_phase = i == phase_count - 1;
      let col_count = phase_tasks.len();

      writeln!(f, "  {}", PhaseHeader::new(*phase))?;

      if col_count > 1 {
        write!(f, "  ")?;
        write_branch_open(f, col_count)?;
        writeln!(f)?;
      }

      let priority_pad = priority_pad_width(phase_tasks.iter().map(|t| t.priority));

      for (ti, task) in phase_tasks.iter().enumerate() {
        let row = TaskRow::new(
          ti,
          col_count,
          &task.id_short,
          &task.title,
          &task.status,
          task.priority,
          priority_pad,
          self.prefix_len,
          &task.blocked_by,
          task.is_blocking,
        );
        writeln!(f, "  {row}")?;
      }

      if col_count > 1 {
        write!(f, "  ")?;
        write_branch_close(f, col_count, is_last_phase)?;
        writeln!(f)?;
      }

      if !is_last_phase {
        writeln!(f, "  {}", CONTINUATION.paint(*theme.iteration_graph_branch()))?;
      }
    }

    Ok(())
  }
}

fn write_branch_close(f: &mut Formatter<'_>, cols: usize, is_last_phase: bool) -> fmt::Result {
  let theme = crate::ui::style::global();
  let branch = *theme.iteration_graph_branch();
  let start = if is_last_phase {
    BRANCH_CLOSE_LAST
  } else {
    BRANCH_CLOSE_MID
  };
  write!(f, "{}", start.paint(branch))?;
  for _ in 1..cols {
    write!(f, "{}{}", BRANCH_DASH.paint(branch), CLOSE_TIP.paint(branch))?;
  }
  Ok(())
}

fn write_branch_open(f: &mut Formatter<'_>, cols: usize) -> fmt::Result {
  let theme = crate::ui::style::global();
  let branch = *theme.iteration_graph_branch();
  write!(f, "{}", BRANCH_OPEN.paint(branch))?;
  for _ in 1..cols {
    write!(f, "{}{}", BRANCH_DASH.paint(branch), BRANCH_OPEN_TIP.paint(branch))?;
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  fn render(c: &Component) -> String {
    yansi::disable();
    let out = c.to_string();
    yansi::enable();
    out
  }

  fn task(id: &str, phase: u32, title: &str, status: &str) -> GraphTask {
    GraphTask {
      blocked_by: Vec::new(),
      id_short: id.into(),
      is_blocking: false,
      phase,
      priority: None,
      status: status.into(),
      title: title.into(),
    }
  }

  #[test]
  fn it_renders_a_blocked_task_with_the_blocked_icon() {
    let mut t = task("bbbbbbbb", 1, "blocked task", "open");
    t.blocked_by = vec!["aaaaaaaa".into()];
    let graph = Component::new("Iter", vec![t]);

    let out = render(&graph);

    insta::assert_snapshot!(out);
  }

  #[test]
  fn it_renders_a_blocking_task_with_the_blocking_indicator() {
    let mut t = task("cccccccc", 1, "blocker", "in-progress");
    t.is_blocking = true;
    let graph = Component::new("Iter", vec![t]);

    let out = render(&graph);

    insta::assert_snapshot!(out);
  }

  #[test]
  fn it_renders_a_long_title_truncated_with_ellipsis() {
    let t = task(
      "dddddddd",
      1,
      "this title is long enough that it should be truncated with an ellipsis",
      "open",
    );
    let graph = Component::new("Iter", vec![t]);

    let out = render(&graph);

    insta::assert_snapshot!(out);
  }

  #[test]
  fn it_renders_a_multi_task_phase_with_branch_connectors() {
    let graph = Component::new(
      "Iter",
      vec![
        task("aaaaaaaa", 1, "first task", "done"),
        task("bbbbbbbb", 1, "second task", "in-progress"),
        task("cccccccc", 1, "third task", "open"),
      ],
    );

    let out = render(&graph);

    insta::assert_snapshot!(out);
  }

  #[test]
  fn it_renders_a_single_task_phase_without_branches() {
    let graph = Component::new("Iter", vec![task("aaaaaaaa", 1, "solo task", "open")]);

    let out = render(&graph);

    insta::assert_snapshot!(out);
  }

  #[test]
  fn it_renders_multiple_phases_with_continuation_lines() {
    let graph = Component::new(
      "Iter",
      vec![
        task("aaaaaaaa", 1, "phase one task", "done"),
        task("bbbbbbbb", 2, "phase two task", "in-progress"),
        task("cccccccc", 3, "phase three task", "open"),
      ],
    );

    let out = render(&graph);

    insta::assert_snapshot!(out);
  }

  #[test]
  fn it_renders_priority_badges_when_priority_is_present() {
    let mut t = task("aaaaaaaa", 1, "priority task", "open");
    t.priority = Some(1);
    let graph = Component::new("Iter", vec![t]);

    let out = render(&graph);

    insta::assert_snapshot!(out);
  }

  #[test]
  fn it_renders_without_priority_column_when_absent() {
    let graph = Component::new("Iter", vec![task("aaaaaaaa", 1, "no priority", "open")]);

    let out = render(&graph);

    insta::assert_snapshot!(out);
  }
}

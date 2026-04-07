//! Phased dependency graph with box-drawing connectors.

use std::{
  collections::BTreeMap,
  fmt::{self, Display, Formatter},
};

use yansi::Paint;

/// A task entry for the graph.
pub struct GraphTask {
  pub id_short: String,
  pub phase: u32,
  pub status: String,
  pub title: String,
}

/// Phased dependency graph with box-drawing connectors.
pub struct Component {
  iteration_title: String,
  tasks: Vec<GraphTask>,
}

impl Component {
  pub fn new(iteration_title: impl Into<String>, tasks: Vec<GraphTask>) -> Self {
    Self {
      iteration_title: iteration_title.into(),
      tasks,
    }
  }
}

impl Display for Component {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let theme = crate::ui::style::global();

    writeln!(f)?;
    writeln!(f, "  {}", self.iteration_title.paint(*theme.iteration_graph_title()))?;
    writeln!(f)?;

    if self.tasks.is_empty() {
      writeln!(f, "  no tasks in this iteration")?;
      return Ok(());
    }

    let mut phases: BTreeMap<u32, Vec<&GraphTask>> = BTreeMap::new();
    for task in &self.tasks {
      phases.entry(task.phase).or_default().push(task);
    }

    let phase_count = phases.len();
    for (i, (phase, phase_tasks)) in phases.iter().enumerate() {
      let is_last = i == phase_count - 1;
      let connector = if i == 0 { "╭" } else { "├" };
      writeln!(
        f,
        "  {} ── phase {} ──",
        connector.paint(*theme.iteration_graph_branch()),
        phase
      )?;

      for task in phase_tasks {
        let icon = match task.status.as_str() {
          "done" => "●".paint(*theme.status_done()),
          "in-progress" => "◐".paint(*theme.status_in_progress()),
          "cancelled" => "⊘".paint(*theme.status_cancelled()),
          _ => "●".paint(*theme.status_open()),
        };
        writeln!(
          f,
          "  {}   {} {} {}",
          "│".paint(*theme.iteration_graph_separator()),
          icon,
          task.id_short.paint(*theme.id_rest()),
          task.title,
        )?;
      }

      if is_last {
        writeln!(f, "  {}", "╰".paint(*theme.iteration_graph_branch()))?;
      }
    }

    Ok(())
  }
}

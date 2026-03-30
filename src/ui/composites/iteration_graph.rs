use std::fmt;

use yansi::Paint;

use crate::ui::{
  atoms::{badge::Badge, icon::Icon, id::Id, title::Title},
  layout::Row,
  theme::Theme,
};

/// Max display width for task titles in graph rows.
const TITLE_PAD: usize = 35;

/// Data for a single task rendered inside the iteration graph.
pub struct TaskData<'a> {
  pub blocked_by: Option<&'a str>,
  pub id: &'a str,
  pub is_blocking: bool,
  pub priority: Option<u8>,
  pub status: &'a str,
  pub tags: &'a [String],
  pub title: &'a str,
}

/// Data for a single phase containing its tasks.
pub struct PhaseData<'a> {
  pub name: Option<&'a str>,
  pub number: u32,
  pub tasks: Vec<TaskData<'a>>,
}

/// Renders an iteration as a phased dependency graph with branching box-drawing connectors.
pub struct IterationGraph<'a> {
  pub phases: Vec<PhaseData<'a>>,
  pub theme: &'a Theme,
  pub title: &'a str,
}

impl<'a> IterationGraph<'a> {
  fn fmt_branch_close(&self, task_count: usize, is_last_phase: bool, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if task_count <= 1 {
      return Ok(());
    }
    let branch = self.theme.iteration_graph_branch;
    let start = if is_last_phase { "\u{2570}" } else { "\u{251C}" };
    write!(f, "{}", start.paint(branch))?;
    for _ in 1..task_count {
      write!(f, "{}{}", "\u{2500}".paint(branch), "\u{256F}".paint(branch))?;
    }
    Ok(())
  }

  fn fmt_branch_open(&self, task_count: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if task_count <= 1 {
      return Ok(());
    }
    let branch = self.theme.iteration_graph_branch;
    write!(f, "{}", "\u{251C}".paint(branch))?;
    for _ in 1..task_count {
      write!(f, "{}{}", "\u{2500}".paint(branch), "\u{256E}".paint(branch))?;
    }
    Ok(())
  }

  fn fmt_continuation(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", "\u{2502}".paint(self.theme.iteration_graph_branch))
  }

  fn fmt_phase_header(&self, phase: &PhaseData<'_>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let icon = Icon::phase(self.theme);
    let label = format!("Phase {}", phase.number);
    let sep = "\u{2500}\u{2500}";

    write!(
      f,
      "{}  {}  {}",
      icon,
      Row::new()
        .spacing(2)
        .col(label.paint(self.theme.iteration_graph_phase_label))
        .col(sep.paint(self.theme.iteration_graph_separator)),
      match phase.name {
        Some(name) => format!("{}", name.paint(self.theme.iteration_graph_phase_name)),
        None => String::new(),
      }
    )
  }

  fn fmt_summary(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let np = self.phases.len();
    let nt = self.total_tasks();
    let phase_word = if np == 1 { "phase" } else { "phases" };
    let task_word = if nt == 1 { "task" } else { "tasks" };
    write!(
      f,
      "  {}",
      format!("{np} {phase_word} \u{00B7} {nt} {task_word}").paint(self.theme.list_summary)
    )
  }

  fn fmt_task_row(
    &self,
    task: &TaskData<'_>,
    task_idx: usize,
    total_tasks: usize,
    f: &mut fmt::Formatter<'_>,
  ) -> fmt::Result {
    let branch = self.theme.iteration_graph_branch;

    for col in 0..total_tasks {
      if col == task_idx {
        let icon = if task.blocked_by.is_some() {
          Icon::blocked(self.theme)
        } else {
          Icon::status(task.status, self.theme)
        };
        write!(f, "{icon}")?;
      } else {
        write!(f, "{}", "\u{2502}".paint(branch))?;
      }
      if col < total_tasks - 1 {
        write!(f, " ")?;
      }
    }

    let mut row = Row::new().spacing(2);

    row = row.col(Id::new(task.id, self.theme));

    if let Some(p) = task.priority {
      row = row.col(Badge::new(format!("[P{p}]"), self.theme.task_list_priority));
    }

    let title_style = if task.status == "cancelled" {
      self.theme.task_list_title_cancelled
    } else {
      self.theme.task_list_title
    };
    row = row.col(
      Title::new(task.title, title_style)
        .max_width(TITLE_PAD)
        .pad_to(TITLE_PAD),
    );

    row = row.col(self.status_badge(task));

    write!(f, "  {row}")
  }

  fn fmt_title(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.title.paint(self.theme.iteration_graph_title))
  }

  fn status_badge(&self, task: &TaskData<'_>) -> Badge {
    if task.blocked_by.is_some() {
      let icon = Icon::blocked(self.theme);
      return Badge::new(format!("{icon} blocked"), self.theme.indicator_blocked);
    }

    let icon = Icon::status(task.status, self.theme);
    let (label, style) = match task.status {
      "open" => ("open", self.theme.status_open),
      "in-progress" => ("in progress", self.theme.status_in_progress),
      "done" => ("done", self.theme.status_done),
      "cancelled" => ("cancelled", self.theme.status_cancelled),
      other => (other, self.theme.status_open),
    };
    Badge::new(format!("{icon} {label}"), style)
  }

  fn total_tasks(&self) -> usize {
    self.phases.iter().map(|p| p.tasks.len()).sum()
  }
}

impl fmt::Display for IterationGraph<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.fmt_title(f)?;
    writeln!(f)?;

    self.fmt_summary(f)?;
    writeln!(f)?;

    writeln!(f)?;

    let phase_count = self.phases.len();
    for (pi, phase) in self.phases.iter().enumerate() {
      let is_last = pi == phase_count - 1;
      let task_count = phase.tasks.len();

      self.fmt_phase_header(phase, f)?;
      writeln!(f)?;

      if task_count > 1 {
        self.fmt_branch_open(task_count, f)?;
        writeln!(f)?;
      }

      for (ti, task) in phase.tasks.iter().enumerate() {
        self.fmt_task_row(task, ti, task_count, f)?;
        writeln!(f)?;
      }

      if task_count > 1 {
        self.fmt_branch_close(task_count, is_last, f)?;
        writeln!(f)?;
      }

      if !is_last {
        self.fmt_continuation(f)?;
        writeln!(f)?;
      }
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

  fn render(graph: &IterationGraph) -> String {
    yansi::disable();
    let out = graph.to_string();
    yansi::enable();
    out
  }

  fn sample_graph(theme: &Theme) -> IterationGraph<'_> {
    IterationGraph {
      title: "Q1 LLM Benchmark Evaluation",
      phases: vec![
        PhaseData {
          number: 1,
          name: Some("foundation"),
          tasks: vec![
            TaskData {
              status: "done",
              id: "cdrzjvwk",
              title: "sqlite storage backend",
              priority: Some(0),
              tags: &[],
              is_blocking: false,
              blocked_by: None,
            },
            TaskData {
              status: "done",
              id: "hpvrlbme",
              title: "finalize probe schema v2",
              priority: Some(0),
              tags: &[],
              is_blocking: false,
              blocked_by: None,
            },
          ],
        },
        PhaseData {
          number: 2,
          name: Some("core implementation"),
          tasks: vec![
            TaskData {
              status: "in-progress",
              id: "nfkbqmrx",
              title: "openai streaming adapter",
              priority: Some(1),
              tags: &[],
              is_blocking: false,
              blocked_by: None,
            },
            TaskData {
              status: "open",
              id: "mxdtqrbn",
              title: "context window handling",
              priority: Some(1),
              tags: &[],
              is_blocking: false,
              blocked_by: Some("hpvrlbme"),
            },
            TaskData {
              status: "open",
              id: "qtsdwcaz",
              title: "probe dedup by content hash",
              priority: Some(2),
              tags: &[],
              is_blocking: false,
              blocked_by: None,
            },
          ],
        },
        PhaseData {
          number: 3,
          name: Some("delivery"),
          tasks: vec![
            TaskData {
              status: "open",
              id: "rwlkbpjq",
              title: "CI pipeline integration",
              priority: Some(2),
              tags: &[],
              is_blocking: false,
              blocked_by: None,
            },
            TaskData {
              status: "open",
              id: "zvhqtxmn",
              title: "integration test suite",
              priority: Some(2),
              tags: &[],
              is_blocking: false,
              blocked_by: None,
            },
          ],
        },
      ],
      theme,
    }
  }

  #[test]
  fn it_omits_branches_for_single_task_phase() {
    let t = theme();
    let graph = IterationGraph {
      title: "Single task test",
      phases: vec![PhaseData {
        number: 1,
        name: Some("solo"),
        tasks: vec![TaskData {
          status: "open",
          id: "abcd1234",
          title: "the only task",
          priority: None,
          tags: &[],
          is_blocking: false,
          blocked_by: None,
        }],
      }],
      theme: &t,
    };
    let output = render(&graph);
    assert!(!output.contains('\u{256E}'), "should not have ╮ for single-task phase");
    assert!(!output.contains('\u{256F}'), "should not have ╯ for single-task phase");
    assert!(output.contains("abcd1234"));
  }

  #[test]
  fn it_renders_branch_open_close_for_two_tasks() {
    let t = theme();
    let graph = sample_graph(&t);
    let output = render(&graph);
    assert!(output.contains("\u{251C}\u{2500}\u{256E}"), "should have ├─╮");
  }

  #[test]
  fn it_renders_branch_open_for_three_tasks() {
    let t = theme();
    let graph = sample_graph(&t);
    let output = render(&graph);
    assert!(
      output.contains("\u{251C}\u{2500}\u{256E}\u{2500}\u{256E}"),
      "should have ├─╮─╮"
    );
  }

  #[test]
  fn it_renders_continuation_line_between_phases() {
    let t = theme();
    let graph = sample_graph(&t);
    let output = render(&graph);
    let lines: Vec<&str> = output.lines().collect();
    let continuation_lines: Vec<&&str> = lines.iter().filter(|l| l.trim() == "\u{2502}").collect();
    assert!(
      continuation_lines.len() >= 2,
      "should have continuation lines between phases, found {}",
      continuation_lines.len()
    );
  }

  #[test]
  fn it_renders_phase_headers() {
    let t = theme();
    let graph = sample_graph(&t);
    let output = render(&graph);
    assert!(output.contains("Phase 1"), "should contain Phase 1");
    assert!(output.contains("Phase 2"), "should contain Phase 2");
    assert!(output.contains("Phase 3"), "should contain Phase 3");
    assert!(output.contains("foundation"), "should contain phase name");
    assert!(output.contains("core implementation"), "should contain phase name");
    assert!(output.contains("delivery"), "should contain phase name");
  }

  #[test]
  fn it_renders_phase_without_name() {
    let t = theme();
    let graph = IterationGraph {
      title: "No name test",
      phases: vec![PhaseData {
        number: 1,
        name: None,
        tasks: vec![TaskData {
          status: "done",
          id: "testtest",
          title: "a task",
          priority: None,
          tags: &[],
          is_blocking: false,
          blocked_by: None,
        }],
      }],
      theme: &t,
    };
    let output = render(&graph);
    assert!(output.contains("Phase 1"), "should still show Phase N");
    assert!(output.contains("\u{2500}\u{2500}"), "should still show separator");
  }

  #[test]
  fn it_renders_task_ids() {
    let t = theme();
    let graph = sample_graph(&t);
    let output = render(&graph);
    assert!(output.contains("cdrzjvwk"));
    assert!(output.contains("hpvrlbme"));
    assert!(output.contains("nfkbqmrx"));
    assert!(output.contains("mxdtqrbn"));
    assert!(output.contains("qtsdwcaz"));
    assert!(output.contains("rwlkbpjq"));
    assert!(output.contains("zvhqtxmn"));
  }

  #[test]
  fn it_shows_blocked_status_for_blocked_task() {
    let t = theme();
    let graph = sample_graph(&t);
    let output = render(&graph);
    assert!(output.contains('\u{2297}'), "should use blocked icon ⊗");
  }

  #[test]
  fn it_shows_phase_and_task_counts_in_summary() {
    let t = theme();
    let graph = sample_graph(&t);
    let output = render(&graph);
    assert!(output.contains("3 phases"), "should contain phase count");
    assert!(output.contains("7 tasks"), "should contain task count");
  }

  #[test]
  fn it_shows_pipe_in_branch_columns_for_other_tasks() {
    let t = theme();
    let graph = sample_graph(&t);
    let output = render(&graph);
    let has_pipe_in_task_line = output.lines().any(|l| l.contains('\u{2502}') && l.contains("cdrzjvwk"));
    assert!(has_pipe_in_task_line, "task rows should have │ for non-active columns");
  }

  #[test]
  fn it_shows_title_on_first_line() {
    let t = theme();
    let graph = sample_graph(&t);
    let output = render(&graph);
    let first_line = output.lines().next().unwrap();
    assert!(
      first_line.contains("Q1 LLM Benchmark Evaluation"),
      "first line should contain the title"
    );
  }

  #[test]
  fn it_uses_rounded_close_for_last_phase() {
    let t = theme();
    let graph = sample_graph(&t);
    let output = render(&graph);
    assert!(output.contains("\u{2570}\u{2500}\u{256F}"), "last phase should use ╰─╯");
  }

  #[test]
  fn it_uses_singular_forms_in_summary() {
    let t = theme();
    let graph = IterationGraph {
      title: "Singular",
      phases: vec![PhaseData {
        number: 1,
        name: None,
        tasks: vec![TaskData {
          status: "open",
          id: "xxxxxxxx",
          title: "only task",
          priority: None,
          tags: &[],
          is_blocking: false,
          blocked_by: None,
        }],
      }],
      theme: &t,
    };
    let output = render(&graph);
    assert!(output.contains("1 phase"), "should use singular 'phase'");
    assert!(output.contains("1 task"), "should use singular 'task'");
    assert!(!output.contains("1 phases"), "should not use plural for 1");
  }

  #[test]
  fn it_uses_tee_close_for_non_last_phase() {
    let t = theme();
    let graph = sample_graph(&t);
    let output = render(&graph);
    assert!(
      output.contains("\u{251C}\u{2500}\u{256F}"),
      "non-last phase should use ├─╯"
    );
  }
}

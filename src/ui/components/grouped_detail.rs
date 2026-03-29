use std::io;

use yansi::Paint;

use crate::{
  model::{Artifact, Task},
  ui::{
    components::{ArtifactDetail, TaskDetail},
    theme::Theme,
  },
};

/// A named group of detail items under a heading.
pub enum DetailGroup<'a> {
  Artifacts { heading: String, items: Vec<&'a Artifact> },
  Tasks { heading: String, items: Vec<&'a Task> },
}

/// A grouped detail list that renders full `TaskDetail`/`ArtifactDetail`
/// blocks under group headings, separated by horizontal rules.
pub struct GroupedDetail<'a> {
  groups: Vec<DetailGroup<'a>>,
  theme: &'a Theme,
}

impl<'a> GroupedDetail<'a> {
  pub fn new(groups: Vec<DetailGroup<'a>>, theme: &'a Theme) -> Self {
    Self {
      groups,
      theme,
    }
  }

  /// Write the grouped detail view to the given writer.
  pub fn write_to(&self, w: &mut impl io::Write) -> io::Result<()> {
    let mut first_group = true;

    for group in &self.groups {
      let (heading, count) = match group {
        DetailGroup::Artifacts {
          heading,
          items,
        } => (heading, items.len()),
        DetailGroup::Tasks {
          heading,
          items,
        } => (heading, items.len()),
      };

      if count == 0 {
        continue;
      }

      if !first_group {
        writeln!(w)?;
      }
      first_group = false;

      writeln!(w, "{}", heading.paint(self.theme.list_heading))?;
      writeln!(w)?;

      match group {
        DetailGroup::Artifacts {
          items, ..
        } => {
          for (i, artifact) in items.iter().enumerate() {
            if i > 0 {
              writeln!(w)?;
            }
            self.write_rule(w)?;
            writeln!(w)?;
            ArtifactDetail::new(artifact).write_to(w, self.theme)?;
          }
        }
        DetailGroup::Tasks {
          items, ..
        } => {
          for (i, task) in items.iter().enumerate() {
            if i > 0 {
              writeln!(w)?;
            }
            self.write_rule(w)?;
            writeln!(w)?;
            TaskDetail::new(task).write_to(w, self.theme)?;
          }
        }
      }
    }

    Ok(())
  }

  fn write_rule(&self, w: &mut impl io::Write) -> io::Result<()> {
    let width = terminal_size::terminal_size().map(|(w, _)| w.0 as usize).unwrap_or(80);
    let rule: String = "─".repeat(width);
    writeln!(w, "{}", rule.paint(self.theme.border))
  }
}

crate::ui::macros::impl_display_via_write_to!(GroupedDetail<'_>);

#[cfg(test)]
mod tests {
  use super::*;

  mod grouped_detail {
    use super::*;

    mod display {
      use super::*;

      #[test]
      fn it_delegates_to_write_to() {
        yansi::disable();
        let task = crate::test_helpers::make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
        let groups = vec![DetailGroup::Tasks {
          heading: "Open".to_string(),
          items: vec![&task],
        }];
        let theme = Theme::default();
        let detail = GroupedDetail::new(groups, &theme);
        let display = detail.to_string();
        let mut buf = Vec::new();
        detail.write_to(&mut buf).unwrap();
        let write_output = String::from_utf8(buf).unwrap();
        assert_eq!(display, write_output.trim_end());
      }
    }

    mod write_to {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_hides_empty_groups() {
        yansi::disable();
        let task = crate::test_helpers::make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
        let groups = vec![
          DetailGroup::Tasks {
            heading: "Empty".to_string(),
            items: vec![],
          },
          DetailGroup::Tasks {
            heading: "Open".to_string(),
            items: vec![&task],
          },
        ];
        let theme = Theme::default();
        let detail = GroupedDetail::new(groups, &theme);
        let mut buf = Vec::new();
        detail.write_to(&mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(!output.contains("Empty"), "Should not contain empty group heading");
        assert!(output.contains("Open"), "Should contain non-empty group heading");
      }

      #[test]
      fn it_produces_no_output_when_all_groups_empty() {
        let groups: Vec<DetailGroup> = vec![
          DetailGroup::Tasks {
            heading: "Open".to_string(),
            items: vec![],
          },
          DetailGroup::Artifacts {
            heading: "Spec".to_string(),
            items: vec![],
          },
        ];
        let theme = Theme::default();
        let detail = GroupedDetail::new(groups, &theme);
        let mut buf = Vec::new();
        detail.write_to(&mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.is_empty(), "Should produce no output when all groups are empty");
      }

      #[test]
      fn it_renders_artifact_detail_with_rule() {
        yansi::disable();
        let artifact = crate::test_helpers::make_test_artifact("zyxwvutsrqponmlkzyxwvutsrqponmlk");
        let groups = vec![DetailGroup::Artifacts {
          heading: "Spec".to_string(),
          items: vec![&artifact],
        }];
        let theme = Theme::default();
        let detail = GroupedDetail::new(groups, &theme);
        let mut buf = Vec::new();
        detail.write_to(&mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Spec"), "Should contain group heading");
        assert!(output.contains('─'), "Should contain horizontal rule");
      }

      #[test]
      fn it_renders_task_detail_with_rule() {
        yansi::disable();
        let task = crate::test_helpers::make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
        let groups = vec![DetailGroup::Tasks {
          heading: "Open".to_string(),
          items: vec![&task],
        }];
        let theme = Theme::default();
        let detail = GroupedDetail::new(groups, &theme);
        let mut buf = Vec::new();
        detail.write_to(&mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Open"), "Should contain group heading");
        assert!(output.contains('─'), "Should contain horizontal rule");
        assert!(output.contains(&task.title), "Should contain task title");
      }

      #[test]
      fn it_separates_items_with_rules() {
        yansi::disable();
        let task1 = crate::test_helpers::make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
        let task2 = crate::test_helpers::make_test_task("klmnopqrstuvwxyzklmnopqrstuvwxyz");
        let groups = vec![DetailGroup::Tasks {
          heading: "Open".to_string(),
          items: vec![&task1, &task2],
        }];
        let theme = Theme::default();
        let detail = GroupedDetail::new(groups, &theme);
        let mut buf = Vec::new();
        detail.write_to(&mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        let rule_count = output.lines().filter(|l| l.contains('─')).count();
        assert_eq!(rule_count, 2, "Should have a rule before each item");
      }

      #[test]
      fn it_separates_multiple_groups_with_blank_line() {
        yansi::disable();
        let task = crate::test_helpers::make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
        let artifact = crate::test_helpers::make_test_artifact("klmnopqrstuvwxyzklmnopqrstuvwxyz");
        let groups = vec![
          DetailGroup::Tasks {
            heading: "Open".to_string(),
            items: vec![&task],
          },
          DetailGroup::Artifacts {
            heading: "Spec".to_string(),
            items: vec![&artifact],
          },
        ];
        let theme = Theme::default();
        let detail = GroupedDetail::new(groups, &theme);
        let mut buf = Vec::new();
        detail.write_to(&mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.contains("Open"), "Should contain first heading");
        assert!(output.contains("Spec"), "Should contain second heading");
      }
    }
  }
}

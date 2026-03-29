use std::io;

use yansi::Paint;

use crate::{
  model::Task,
  ui::{
    components::{Tags, TaskStatus},
    markdown,
    theme::Theme,
  },
};

/// Detail view for a task, matching the output of `task show`.
pub struct TaskDetail<'a> {
  task: &'a Task,
}

impl<'a> TaskDetail<'a> {
  pub fn new(task: &'a Task) -> Self {
    Self {
      task,
    }
  }

  /// Write the detail view to the given writer.
  pub fn write_to(&self, w: &mut impl io::Write, theme: &Theme) -> io::Result<()> {
    // Title heading
    writeln!(w, "{}", self.task.title.paint(theme.md_heading))?;
    // Status on its own line
    writeln!(w, "{}", TaskStatus::new(&self.task.status, theme))?;
    // Priority, phase, assigned_to (omitted if absent)
    if let Some(priority) = self.task.priority {
      writeln!(w, "{} P{priority}", "priority:".paint(theme.muted))?;
    }
    if let Some(phase) = self.task.phase {
      writeln!(w, "{} {phase}", "phase:".paint(theme.muted))?;
    }
    if let Some(ref assigned_to) = self.task.assigned_to {
      writeln!(w, "{} {assigned_to}", "assigned:".paint(theme.muted))?;
    }
    // Tags with @ prefix (omitted if empty)
    if !self.task.tags.is_empty() {
      writeln!(w, "{}", Tags::new(&self.task.tags, theme))?;
    }

    // Blank line before description
    if !self.task.description.is_empty() {
      writeln!(w)?;
      markdown::render(w, &self.task.description, theme)?;
    }

    // Links section (omitted if empty)
    if !self.task.links.is_empty() {
      writeln!(w)?;
      writeln!(w, "{}", "── Links ──".paint(theme.border))?;
      writeln!(w)?;
      for link in &self.task.links {
        writeln!(w, "- **{}:** {}", link.rel, link.ref_)?;
      }
    }

    // Metadata section (omitted if empty)
    if !self.task.metadata.is_empty() {
      writeln!(w)?;
      writeln!(w, "{}", "── Metadata ──".paint(theme.border))?;
      writeln!(w)?;
      for (key, value) in &self.task.metadata {
        writeln!(w, "- **{key}:** {value}")?;
      }
    }

    Ok(())
  }
}

crate::ui::macros::impl_display_via_write_to!(TaskDetail<'_>, theme);

#[cfg(test)]
mod tests {
  use super::*;

  fn make_task() -> Task {
    Task {
      title: "Test Task".to_string(),
      ..crate::test_helpers::make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk")
    }
  }

  mod display {
    use super::*;

    #[test]
    fn it_delegates_to_write_to() {
      let mut task = make_task();
      task.description = "Some desc".to_string();
      task.tags = vec!["test".to_string()];
      let detail = TaskDetail::new(&task);
      let display = detail.to_string();
      let mut buf = Vec::new();
      detail.write_to(&mut buf, &Theme::default()).unwrap();
      let write_output = String::from_utf8(buf).unwrap();
      assert_eq!(display, write_output.trim_end());
    }
  }

  mod write_to {
    use super::*;
    use crate::model::{Link, RelationshipType};

    #[test]
    fn it_writes_styled_title_heading() {
      let task = make_task();
      let detail = TaskDetail::new(&task);
      let mut buf = Vec::new();
      detail.write_to(&mut buf, &Theme::default()).unwrap();
      let output = String::from_utf8(buf).unwrap();
      assert!(output.contains("Test Task"), "Should contain styled title");
    }

    #[test]
    fn it_writes_status_on_own_line() {
      let task = make_task();
      let detail = TaskDetail::new(&task);
      let mut buf = Vec::new();
      detail.write_to(&mut buf, &Theme::default()).unwrap();
      let output = String::from_utf8(buf).unwrap();
      assert!(output.contains("open"), "Should contain status text");
      // Status should not be on same line as title
      let lines: Vec<&str> = output.lines().collect();
      assert!(lines.len() >= 2, "Should have at least 2 lines (title + status)");
    }

    #[test]
    fn it_omits_tags_when_empty() {
      let task = make_task();
      let detail = TaskDetail::new(&task);
      let mut buf = Vec::new();
      detail.write_to(&mut buf, &Theme::default()).unwrap();
      let output = String::from_utf8(buf).unwrap();
      assert!(!output.contains("@"), "Should not contain @ tags when empty");
    }

    #[test]
    fn it_writes_tags_with_at_prefix() {
      let mut task = make_task();
      task.tags = vec!["rust".to_string(), "cli".to_string()];
      let detail = TaskDetail::new(&task);
      let mut buf = Vec::new();
      detail.write_to(&mut buf, &Theme::default()).unwrap();
      let output = String::from_utf8(buf).unwrap();
      assert!(output.contains("@rust"), "Should contain @rust tag");
      assert!(output.contains("@cli"), "Should contain @cli tag");
    }

    #[test]
    fn it_renders_description_with_markdown() {
      let mut task = make_task();
      task.description = "Some **bold** description".to_string();
      let detail = TaskDetail::new(&task);
      let mut buf = Vec::new();
      detail.write_to(&mut buf, &Theme::default()).unwrap();
      let output = String::from_utf8(buf).unwrap();
      assert!(output.contains("bold"), "Should contain rendered markdown description");
    }

    #[test]
    fn it_omits_description_when_empty() {
      let task = make_task();
      let detail = TaskDetail::new(&task);
      let mut buf = Vec::new();
      detail.write_to(&mut buf, &Theme::default()).unwrap();
      let output = String::from_utf8(buf).unwrap();
      let lines: Vec<&str> = output.lines().collect();
      // With no description, tags, links, or metadata: just title + status
      assert_eq!(lines.len(), 2, "Should only have title and status lines");
    }

    #[test]
    fn it_writes_links_with_separator() {
      let mut task = make_task();
      task.links = vec![Link {
        ref_: "https://example.com".to_string(),
        rel: RelationshipType::RelatesTo,
      }];
      let detail = TaskDetail::new(&task);
      let mut buf = Vec::new();
      detail.write_to(&mut buf, &Theme::default()).unwrap();
      let output = String::from_utf8(buf).unwrap();
      assert!(output.contains("Links"), "Should contain Links separator");
      assert!(output.contains("https://example.com"), "Should contain link reference");
    }

    #[test]
    fn it_omits_links_when_empty() {
      let task = make_task();
      let detail = TaskDetail::new(&task);
      let mut buf = Vec::new();
      detail.write_to(&mut buf, &Theme::default()).unwrap();
      let output = String::from_utf8(buf).unwrap();
      assert!(
        !output.contains("Links"),
        "Should not contain Links separator when empty"
      );
    }

    #[test]
    fn it_writes_metadata_with_separator() {
      let mut task = make_task();
      task
        .metadata
        .insert("wave".to_string(), toml::Value::String("3".to_string()));
      let detail = TaskDetail::new(&task);
      let mut buf = Vec::new();
      detail.write_to(&mut buf, &Theme::default()).unwrap();
      let output = String::from_utf8(buf).unwrap();
      assert!(output.contains("Metadata"), "Should contain Metadata separator");
      assert!(output.contains("wave"), "Should contain metadata key");
    }

    #[test]
    fn it_omits_metadata_when_empty() {
      let task = make_task();
      let detail = TaskDetail::new(&task);
      let mut buf = Vec::new();
      detail.write_to(&mut buf, &Theme::default()).unwrap();
      let output = String::from_utf8(buf).unwrap();
      assert!(
        !output.contains("Metadata"),
        "Should not contain Metadata separator when empty"
      );
    }
  }
}

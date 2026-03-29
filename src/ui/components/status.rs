use std::fmt;

use yansi::Paint;

use crate::{model, ui::theme::Theme};

/// Atomic component for rendering a task status with theme-appropriate colors.
pub struct TaskStatus<'a> {
  status: &'a model::Status,
  theme: &'a Theme,
}

impl<'a> TaskStatus<'a> {
  pub fn new(status: &'a model::Status, theme: &'a Theme) -> Self {
    Self {
      status,
      theme,
    }
  }
}

impl fmt::Display for TaskStatus<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let style = match self.status {
      model::Status::Open => self.theme.status_open,
      model::Status::InProgress => self.theme.status_in_progress,
      model::Status::Done => self.theme.status_done,
      model::Status::Cancelled => self.theme.status_cancelled,
    };
    write!(f, "{}", self.status.to_string().paint(style))
  }
}

/// Atomic component for rendering an iteration status with theme-appropriate colors.
pub struct IterationStatus<'a> {
  status: &'a model::iteration::Status,
  theme: &'a Theme,
}

impl<'a> IterationStatus<'a> {
  pub fn new(status: &'a model::iteration::Status, theme: &'a Theme) -> Self {
    Self {
      status,
      theme,
    }
  }
}

impl fmt::Display for IterationStatus<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let style = match self.status {
      model::iteration::Status::Active => self.theme.status_in_progress,
      model::iteration::Status::Completed => self.theme.status_done,
      model::iteration::Status::Failed => self.theme.status_cancelled,
    };
    write!(f, "{}", self.status.to_string().paint(style))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod task_status {
    use super::*;

    #[test]
    fn it_renders_open() {
      let theme = Theme::default();
      let rendered = TaskStatus::new(&model::Status::Open, &theme).to_string();
      assert!(rendered.contains("open"), "Should contain 'open'");
    }

    #[test]
    fn it_renders_in_progress() {
      let theme = Theme::default();
      let rendered = TaskStatus::new(&model::Status::InProgress, &theme).to_string();
      assert!(rendered.contains("in-progress"), "Should contain 'in-progress'");
    }

    #[test]
    fn it_renders_done() {
      let theme = Theme::default();
      let rendered = TaskStatus::new(&model::Status::Done, &theme).to_string();
      assert!(rendered.contains("done"), "Should contain 'done'");
    }

    #[test]
    fn it_renders_cancelled() {
      let theme = Theme::default();
      let rendered = TaskStatus::new(&model::Status::Cancelled, &theme).to_string();
      assert!(rendered.contains("cancelled"), "Should contain 'cancelled'");
    }
  }

  mod iteration_status {
    use super::*;

    #[test]
    fn it_renders_active() {
      let theme = Theme::default();
      let rendered = IterationStatus::new(&model::iteration::Status::Active, &theme).to_string();
      assert!(rendered.contains("active"), "Should contain 'active'");
    }

    #[test]
    fn it_renders_completed() {
      let theme = Theme::default();
      let rendered = IterationStatus::new(&model::iteration::Status::Completed, &theme).to_string();
      assert!(rendered.contains("completed"), "Should contain 'completed'");
    }

    #[test]
    fn it_renders_failed() {
      let theme = Theme::default();
      let rendered = IterationStatus::new(&model::iteration::Status::Failed, &theme).to_string();
      assert!(rendered.contains("failed"), "Should contain 'failed'");
    }
  }
}

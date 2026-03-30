use std::fmt;

use yansi::Paint;

use crate::ui::theme::Theme;

/// Renders inline blocked/blocking status indicators for a task.
pub struct Indicators<'a> {
  blocked_by: Vec<&'a str>,
  is_blocking: bool,
  theme: &'a Theme,
}

impl<'a> Indicators<'a> {
  pub fn new(theme: &'a Theme) -> Self {
    Self {
      is_blocking: false,
      blocked_by: Vec::new(),
      theme,
    }
  }

  /// Sets the IDs of tasks blocking this one.
  pub fn blocked_by(mut self, ids: Vec<&'a str>) -> Self {
    self.blocked_by = ids;
    self
  }

  /// Marks this task as blocking other tasks.
  pub fn blocking(mut self, v: bool) -> Self {
    self.is_blocking = v;
    self
  }
}

impl fmt::Display for Indicators<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut parts = Vec::new();

    if !self.blocked_by.is_empty() {
      parts.push(format!("{}", "⊗ blocked".paint(self.theme.indicator_blocked)));
      for id in &self.blocked_by {
        parts.push(format!(
          "{}{}",
          "blocked-by ".paint(self.theme.indicator_blocked_by_label),
          id.paint(self.theme.indicator_blocked_by_id),
        ));
      }
    }

    if self.is_blocking {
      parts.push(format!("{}", "! blocking".paint(self.theme.indicator_blocking)));
    }

    write!(f, "{}", parts.join("  "))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn theme() -> Theme {
    Theme::default()
  }

  #[test]
  fn it_renders_blocked_by_one_id() {
    let t = theme();
    let rendered = format!("{}", Indicators::new(&t).blocked_by(vec!["hpvrlbme"]));
    assert!(rendered.contains("⊗ blocked"));
    assert!(rendered.contains("blocked-by"));
    assert!(rendered.contains("hpvrlbme"));
  }

  #[test]
  fn it_renders_blocking_only() {
    let t = theme();
    let rendered = format!("{}", Indicators::new(&t).blocking(true));
    assert!(rendered.contains("! blocking"));
    assert!(!rendered.contains("blocked-by"));
  }

  #[test]
  fn it_renders_both_blocked_and_blocking() {
    let t = theme();
    let rendered = format!("{}", Indicators::new(&t).blocked_by(vec!["abc12345"]).blocking(true));
    assert!(rendered.contains("⊗ blocked"));
    assert!(rendered.contains("blocked-by"));
    assert!(rendered.contains("abc12345"));
    assert!(rendered.contains("! blocking"));
  }

  #[test]
  fn it_renders_empty_string_for_no_indicators() {
    let t = theme();
    let rendered = format!("{}", Indicators::new(&t));
    assert_eq!(rendered, "");
  }
}

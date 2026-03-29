use std::io;

use yansi::Paint;

use crate::{
  model::Iteration,
  ui::{
    components::{IterationStatus, Tags},
    markdown,
    theme::Theme,
  },
};

/// Detail view for an iteration, matching the output of `iteration show`.
pub struct IterationDetail<'a> {
  iteration: &'a Iteration,
}

impl<'a> IterationDetail<'a> {
  pub fn new(iteration: &'a Iteration) -> Self {
    Self {
      iteration,
    }
  }

  /// Write the detail view to the given writer.
  pub fn write_to(&self, w: &mut impl io::Write, theme: &Theme) -> io::Result<()> {
    writeln!(w, "{}", self.iteration.title.paint(theme.md_heading))?;
    writeln!(w, "{}", IterationStatus::new(&self.iteration.status, theme))?;
    if !self.iteration.tags.is_empty() {
      writeln!(w, "{}", Tags::new(&self.iteration.tags, theme))?;
    }

    if !self.iteration.description.is_empty() {
      writeln!(w)?;
      markdown::render(w, &self.iteration.description, theme)?;
    }

    // Tasks section
    if !self.iteration.tasks.is_empty() {
      writeln!(w)?;
      writeln!(w, "{}", "── Tasks ──".paint(theme.border))?;
      writeln!(w)?;
      for task_ref in &self.iteration.tasks {
        writeln!(w, "- {task_ref}")?;
      }
    }

    // Links section
    if !self.iteration.links.is_empty() {
      writeln!(w)?;
      writeln!(w, "{}", "── Links ──".paint(theme.border))?;
      writeln!(w)?;
      for link in &self.iteration.links {
        writeln!(w, "- **{}:** {}", link.rel, link.ref_)?;
      }
    }

    // Metadata section
    if !self.iteration.metadata.is_empty() {
      writeln!(w)?;
      writeln!(w, "{}", "── Metadata ──".paint(theme.border))?;
      writeln!(w)?;
      for (key, value) in &self.iteration.metadata {
        writeln!(w, "- **{key}:** {value}")?;
      }
    }

    Ok(())
  }
}

crate::ui::macros::impl_display_via_write_to!(IterationDetail<'_>, theme);

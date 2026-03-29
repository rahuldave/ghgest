use chrono::Utc;
use clap::Args;

use crate::{
  config,
  config::Config,
  store,
  ui::{components::TagChange, theme::Theme},
};

/// Add tags to an iteration
#[derive(Debug, Args)]
pub struct Command {
  /// Iteration ID or unique prefix
  pub id: String,
  /// Tags to add (space-separated)
  pub tags: Vec<String>,
}

impl Command {
  pub fn call(&self, config: &Config, _theme: &Theme) -> crate::Result<()> {
    let data_dir = config::data_dir(config)?;
    let id = store::resolve_iteration_id(&data_dir, &self.id, false)?;
    let mut iteration = store::read_iteration(&data_dir, &id)?;

    super::super::tags::apply_tags(&mut iteration.tags, &self.tags);

    iteration.updated_at = Utc::now();
    store::write_iteration(&data_dir, &iteration)?;

    TagChange::new("Tagged", "iteration", &id, &self.tags).write_to(&mut std::io::stdout())?;
    Ok(())
  }
}

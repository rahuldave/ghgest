use clap::Args;

use crate::{
  config,
  config::Config,
  store,
  ui::{components::Confirmation, theme::Theme},
};

/// Remove a task from an iteration
#[derive(Debug, Args)]
pub struct Command {
  /// Iteration ID or unique prefix
  pub id: String,
  /// Task ID or unique prefix to remove
  pub task_id: String,
}

impl Command {
  pub fn call(&self, config: &Config, theme: &Theme) -> crate::Result<()> {
    let data_dir = config::data_dir(config)?;
    let id = store::resolve_iteration_id(&data_dir, &self.id, false)?;
    let task_id = store::resolve_task_id(&data_dir, &self.task_id, true)?;

    let task_ref = format!("tasks/{task_id}");
    store::remove_iteration_task(&data_dir, &id, &task_ref)?;

    Confirmation::new("Removed task from", "iteration", &id).write_to(&mut std::io::stdout(), theme)?;
    Ok(())
  }
}

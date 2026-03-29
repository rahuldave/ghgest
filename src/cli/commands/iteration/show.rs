use clap::Args;

use crate::{
  config,
  config::Config,
  store,
  ui::{components::IterationDetail, theme::Theme},
};

/// Display an iteration's full details, tasks, and links
#[derive(Debug, Args)]
pub struct Command {
  /// Iteration ID or unique prefix
  pub id: String,
  /// Output iteration details as JSON
  #[arg(short, long)]
  pub json: bool,
}

impl Command {
  pub fn call(&self, config: &Config, theme: &Theme) -> crate::Result<()> {
    let data_dir = config::data_dir(config)?;
    let id = store::resolve_iteration_id(&data_dir, &self.id, true)?;
    let iteration = store::read_iteration(&data_dir, &id)?;

    if self.json {
      let json = serde_json::to_string_pretty(&iteration)?;
      println!("{json}");
      return Ok(());
    }

    IterationDetail::new(&iteration).write_to(&mut std::io::stdout(), theme)?;

    Ok(())
  }
}

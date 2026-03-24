use clap::Args;

use crate::{
  config,
  config::Config,
  store,
  ui::{components::Confirmation, theme::Theme},
};

/// Move an artifact to the archive
#[derive(Debug, Args)]
pub struct Command {
  /// Artifact ID or unique prefix
  pub id: String,
}

impl Command {
  pub fn call(&self, config: &Config, theme: &Theme) -> crate::Result<()> {
    let data_dir = config::data_dir(config)?;
    let id = store::resolve_artifact_id(&data_dir, &self.id, false)?;
    store::archive_artifact(&data_dir, &id)?;
    Confirmation::new("Archived", "artifact", &id).write_to(&mut std::io::stdout(), theme)?;
    Ok(())
  }
}

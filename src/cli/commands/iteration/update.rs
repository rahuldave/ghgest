use clap::Args;

use crate::{
  config,
  config::Config,
  model::{IterationPatch, iteration::Status},
  store,
  ui::{components::Confirmation, theme::Theme},
};

/// Update an iteration's title, description, status, tags, or metadata
#[derive(Debug, Args)]
pub struct Command {
  /// Iteration ID or unique prefix
  pub id: String,
  /// New description
  #[arg(short, long)]
  pub description: Option<String>,
  /// Key=value metadata pair, merged with existing (repeatable, e.g. -m key=value)
  #[arg(short, long)]
  pub metadata: Vec<String>,
  /// New status: active, completed, or failed
  #[arg(short, long)]
  pub status: Option<String>,
  /// Replace all tags with this comma-separated list
  #[arg(long)]
  pub tags: Option<String>,
  /// New title
  #[arg(short, long)]
  pub title: Option<String>,
}

impl Command {
  pub fn call(&self, config: &Config, theme: &Theme) -> crate::Result<()> {
    let data_dir = config::data_dir(config)?;
    let id = store::resolve_iteration_id(&data_dir, &self.id, true)?;

    let status = self
      .status
      .as_deref()
      .map(|s| s.parse::<Status>().map_err(crate::Error::generic))
      .transpose()?;

    let metadata = if self.metadata.is_empty() {
      None
    } else {
      let pairs = crate::cli::helpers::split_key_value_pairs(&self.metadata)?;
      let mut table = store::read_iteration(&data_dir, &id)?.metadata;
      for (key, value) in pairs {
        table.insert(key, toml::Value::String(value));
      }
      Some(table)
    };

    let tags = self.tags.as_deref().map(crate::cli::helpers::parse_tags);

    let patch = IterationPatch {
      description: self.description.clone(),
      metadata,
      status,
      tags,
      title: self.title.clone(),
    };

    let iteration = store::update_iteration(&data_dir, &id, patch)?;
    Confirmation::new("Updated", "iteration", &iteration.id).write_to(&mut std::io::stdout(), theme)?;
    Ok(())
  }
}

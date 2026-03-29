use clap::Args;

use crate::{
  config,
  config::Config,
  model::{NewIteration, iteration::Status},
  store,
  ui::{components::Confirmation, theme::Theme},
};

/// Create a new iteration
#[derive(Debug, Args)]
pub struct Command {
  /// Iteration title
  pub title: String,
  /// Description text
  #[arg(short, long)]
  pub description: Option<String>,
  /// Key=value metadata pair (repeatable, e.g. -m key=value)
  #[arg(short, long)]
  pub metadata: Vec<String>,
  /// Initial status: active, completed, or failed (default: active)
  #[arg(short, long)]
  pub status: Option<String>,
  /// Comma-separated list of tags
  #[arg(long)]
  pub tags: Option<String>,
}

impl Command {
  pub fn call(&self, config: &Config, theme: &Theme) -> crate::Result<()> {
    let status = match &self.status {
      Some(s) => s.parse::<Status>().map_err(crate::Error::generic)?,
      None => Status::Active,
    };

    let metadata = {
      let pairs = crate::cli::helpers::split_key_value_pairs(&self.metadata)?;
      let mut table = toml::Table::new();
      for (key, value) in pairs {
        table.insert(key, toml::Value::String(value));
      }
      table
    };

    let tags = self
      .tags
      .as_deref()
      .map(crate::cli::helpers::parse_tags)
      .unwrap_or_default();

    let new = NewIteration {
      description: self.description.clone().unwrap_or_default(),
      links: vec![],
      metadata,
      status,
      tags,
      tasks: vec![],
      title: self.title.clone(),
    };

    let data_dir = config::data_dir(config)?;
    let iteration = store::create_iteration(&data_dir, new)?;
    Confirmation::new("Created", "iteration", &iteration.id).write_to(&mut std::io::stdout(), theme)?;
    Ok(())
  }
}

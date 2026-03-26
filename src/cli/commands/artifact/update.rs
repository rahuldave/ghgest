use std::io::IsTerminal;

use clap::Args;

use crate::{
  config,
  config::Config,
  model::ArtifactPatch,
  store,
  ui::{components::Confirmation, theme::Theme},
};

/// Update an artifact's title, body, type, tags, or metadata
#[derive(Debug, Args)]
pub struct Command {
  /// Artifact ID or unique prefix
  pub id: String,
  /// Replace the body content (opens $EDITOR if omitted and stdin is a terminal)
  #[arg(short, long)]
  pub body: Option<String>,
  /// Artifact type (e.g. spec, adr, rfc, note)
  #[arg(long = "type")]
  pub kind: Option<String>,
  /// Key=value metadata pair, merged with existing (repeatable, e.g. -m key=value)
  #[arg(short, long)]
  pub metadata: Vec<String>,
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
    let id = store::resolve_artifact_id(&data_dir, &self.id, true)?;

    if self.body.is_none() && std::io::stdin().is_terminal() && crate::cli::editor::resolve_editor().is_some() {
      let path = store::artifact_path(&data_dir, &id);
      crate::cli::editor::open_editor(&path)?;
    }

    let metadata = if self.metadata.is_empty() {
      None
    } else {
      let pairs = crate::cli::helpers::split_key_value_pairs(&self.metadata)?;
      let mut map = store::read_artifact(&data_dir, &id)?.metadata;
      for (key, value) in pairs {
        map.insert(
          yaml_serde::Value::String(key),
          yaml_serde::Value::String(value),
        );
      }
      Some(map)
    };

    let patch = ArtifactPatch {
      body: self.body.clone(),
      kind: self.kind.clone(),
      metadata,
      tags: self
        .tags
        .as_deref()
        .map(crate::cli::helpers::parse_tags),
      title: self.title.clone(),
    };

    let artifact = store::update_artifact(&data_dir, &id, patch)?;
    Confirmation::new("Updated", "artifact", &artifact.id).write_to(&mut std::io::stdout(), theme)?;
    Ok(())
  }
}

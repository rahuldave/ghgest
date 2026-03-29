use chrono::Utc;
use clap::Args;

use crate::{
  config,
  config::Config,
  model::{Link, RelationshipType},
  store,
  ui::{components::LinkAdded, theme::Theme},
};

/// Create a relationship between an iteration and another entity
#[derive(Debug, Args)]
pub struct Command {
  /// Iteration ID or unique prefix
  pub id: String,
  /// Relationship type: blocks, blocked-by, child-of, parent-of, or relates-to
  #[arg(value_enum)]
  pub rel: RelationshipType,
  /// Target task or artifact ID or unique prefix
  pub target_id: String,
  /// Target is an artifact instead of a task
  #[arg(long)]
  pub artifact: bool,
}

impl Command {
  pub fn call(&self, config: &Config, _theme: &Theme) -> crate::Result<()> {
    let data_dir = config::data_dir(config)?;
    let id = store::resolve_iteration_id(&data_dir, &self.id, false)?;

    let target_id = if self.artifact {
      store::resolve_artifact_id(&data_dir, &self.target_id, true)?
    } else {
      store::resolve_task_id(&data_dir, &self.target_id, true)?
    };

    let ref_path = if self.artifact {
      format!("artifacts/{target_id}")
    } else {
      format!("tasks/{target_id}")
    };

    let mut iteration = store::read_iteration(&data_dir, &id)?;
    iteration.links.push(Link {
      ref_: ref_path,
      rel: self.rel.clone(),
    });
    iteration.updated_at = Utc::now();
    store::write_iteration(&data_dir, &iteration)?;

    LinkAdded::new(&id, &self.rel.to_string(), &target_id).write_to(&mut std::io::stdout())?;
    Ok(())
  }
}

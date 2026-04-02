use clap::Args;

use super::link;
use crate::cli::{self, AppContext};

/// Shortcut for `task link <id> blocks <blocking_id>`.
#[derive(Debug, Args)]
pub struct Command {
  /// Source task ID or unique prefix (the task that blocks).
  pub id: String,
  /// Target task or artifact ID or unique prefix (the task being blocked).
  pub blocking_id: String,
  /// Target is an artifact instead of a task (no reciprocal link is created).
  #[arg(long)]
  pub artifact: bool,
}

impl Command {
  /// Delegate to the link command with the "blocks" relationship.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    let link_cmd = link::Command {
      id: self.id.clone(),
      rel: crate::model::link::RelationshipType::Blocks,
      target_id: self.blocking_id.clone(),
      artifact: self.artifact,
    };
    link_cmd.call(ctx)
  }
}

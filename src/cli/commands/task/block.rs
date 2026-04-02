use clap::Args;

use super::link;
use crate::cli::{self, AppContext};

/// Shortcut for `task link <id> blocks <blocking_id>`.
#[derive(Debug, Args)]
pub struct Command {
  /// Target is an artifact instead of a task (no reciprocal link is created).
  #[arg(long)]
  pub artifact: bool,
  /// Source task ID or unique prefix (the task that blocks).
  pub id: String,
  /// Output the task as JSON after linking.
  #[arg(short, long, conflicts_with = "quiet")]
  pub json: bool,
  /// Output only the task ID.
  #[arg(short, long, conflicts_with = "json")]
  pub quiet: bool,
  /// Target task or artifact ID or unique prefix (the task being blocked).
  pub blocking_id: String,
}

impl Command {
  /// Delegate to the link command with the "blocks" relationship.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    let link_cmd = link::Command {
      artifact: self.artifact,
      id: self.id.clone(),
      json: self.json,
      quiet: self.quiet,
      rel: crate::model::link::RelationshipType::Blocks,
      target_id: self.blocking_id.clone(),
    };
    link_cmd.call(ctx)
  }
}

mod get;
mod set;

use clap::{Args, Subcommand};

use crate::cli::{self, AppContext};

/// Read or write task metadata fields.
#[derive(Debug, Args)]
pub struct Command {
  #[command(subcommand)]
  command: MetaCommand,
}

#[derive(Debug, Subcommand)]
enum MetaCommand {
  Get(get::Command),
  Set(set::Command),
}

impl Command {
  /// Dispatch to the get or set metadata subcommand.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    match &self.command {
      MetaCommand::Get(cmd) => cmd.call(ctx),
      MetaCommand::Set(cmd) => cmd.call(ctx),
    }
  }
}

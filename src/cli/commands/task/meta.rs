mod get;
mod set;

use std::path::Path;

use clap::{Args, Subcommand};

use crate::{cli, ui::theme::Theme};

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
  pub fn call(&self, data_dir: &Path, theme: &Theme) -> cli::Result<()> {
    match &self.command {
      MetaCommand::Get(cmd) => cmd.call(data_dir),
      MetaCommand::Set(cmd) => cmd.call(data_dir, theme),
    }
  }
}

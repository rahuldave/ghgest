mod get;
mod set;

use clap::{Args, Subcommand};

use crate::{config::Config, ui::theme::Theme};

/// Read or write artifact metadata fields
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
  pub fn call(&self, config: &Config, theme: &Theme) -> crate::Result<()> {
    match &self.command {
      MetaCommand::Get(cmd) => cmd.call(config),
      MetaCommand::Set(cmd) => cmd.call(config, theme),
    }
  }
}

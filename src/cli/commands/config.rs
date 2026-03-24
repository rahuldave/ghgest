mod get;
mod set;
mod show;

use clap::{Args, Subcommand};

use crate::{config::Config, ui::theme::Theme};

/// View and modify gest configuration
#[derive(Debug, Args)]
pub struct Command {
  #[command(subcommand)]
  command: ConfigCommand,
}

#[derive(Debug, Subcommand)]
enum ConfigCommand {
  Get(get::Command),
  Set(set::Command),
  Show(show::Command),
}

impl Command {
  pub fn call(&self, config: &Config, theme: &Theme) -> crate::Result<()> {
    match &self.command {
      ConfigCommand::Get(cmd) => cmd.call(config),
      ConfigCommand::Set(cmd) => cmd.call(config, theme),
      ConfigCommand::Show(cmd) => cmd.call(config),
    }
  }
}

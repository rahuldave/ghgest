//! Subcommands for viewing and modifying gest configuration.

mod get;
mod set;
mod show;

use clap::{Args, Subcommand};

use crate::{cli, config::Settings, ui::theme::Theme};

/// Entry point for the `config` subcommand tree.
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
  /// Dispatch to the appropriate config subcommand.
  pub fn call(&self, settings: &Settings, theme: &Theme) -> cli::Result<()> {
    match &self.command {
      ConfigCommand::Get(cmd) => cmd.call(settings),
      ConfigCommand::Set(cmd) => cmd.call(theme),
      ConfigCommand::Show(cmd) => cmd.call(settings, theme),
    }
  }
}

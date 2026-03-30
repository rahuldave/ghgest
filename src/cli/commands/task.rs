//! Task management commands for creating, updating, listing, and linking tasks.

mod create;
mod link;
mod list;
mod meta;
mod show;
mod tag;
mod untag;
mod update;

use std::path::Path;

use clap::{Args, Subcommand};

use crate::{cli, config::Settings, ui::theme::Theme};

/// Top-level CLI command that dispatches to task subcommands.
#[derive(Debug, Args)]
pub struct Command {
  #[command(subcommand)]
  command: TaskCommand,
}

#[derive(Debug, Subcommand)]
enum TaskCommand {
  Create(create::Command),
  Link(link::Command),
  List(list::Command),
  Meta(meta::Command),
  Show(show::Command),
  Tag(tag::Command),
  Untag(untag::Command),
  Update(update::Command),
}

impl Command {
  /// Route to the appropriate task subcommand.
  pub fn call(&self, _settings: &Settings, theme: &Theme, data_dir: &Path) -> cli::Result<()> {
    match &self.command {
      TaskCommand::Create(cmd) => cmd.call(data_dir, theme),
      TaskCommand::Link(cmd) => cmd.call(data_dir, theme),
      TaskCommand::List(cmd) => cmd.call(data_dir, theme),
      TaskCommand::Meta(cmd) => cmd.call(data_dir, theme),
      TaskCommand::Show(cmd) => cmd.call(data_dir, theme),
      TaskCommand::Tag(cmd) => cmd.call(data_dir, theme),
      TaskCommand::Untag(cmd) => cmd.call(data_dir, theme),
      TaskCommand::Update(cmd) => cmd.call(data_dir, theme),
    }
  }
}

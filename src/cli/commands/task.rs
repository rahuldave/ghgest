mod create;
mod link;
mod list;
mod meta;
mod show;
mod tag;
mod untag;
mod update;

use clap::{Args, Subcommand};

use crate::{config::Config, ui::theme::Theme};

/// Manage tasks (issues, work items, and their lifecycle)
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
  pub fn call(&self, config: &Config, theme: &Theme) -> crate::Result<()> {
    match &self.command {
      TaskCommand::Create(cmd) => cmd.call(config, theme),
      TaskCommand::Link(cmd) => cmd.call(config, theme),
      TaskCommand::List(cmd) => cmd.call(config, theme),
      TaskCommand::Meta(cmd) => cmd.call(config, theme),
      TaskCommand::Show(cmd) => cmd.call(config, theme),
      TaskCommand::Tag(cmd) => cmd.call(config, theme),
      TaskCommand::Untag(cmd) => cmd.call(config, theme),
      TaskCommand::Update(cmd) => cmd.call(config, theme),
    }
  }
}

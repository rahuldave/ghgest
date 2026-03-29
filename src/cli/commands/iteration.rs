mod add;
mod create;
mod graph;
mod link;
mod list;
mod meta;
mod remove;
mod show;
mod tag;
mod untag;
mod update;

use clap::{Args, Subcommand};

use crate::{config::Config, ui::theme::Theme};

/// Manage iterations (execution plans grouping tasks into phases)
#[derive(Debug, Args)]
pub struct Command {
  #[command(subcommand)]
  command: IterationCommand,
}

#[derive(Debug, Subcommand)]
enum IterationCommand {
  Add(add::Command),
  Create(create::Command),
  Graph(graph::Command),
  Link(link::Command),
  List(list::Command),
  Meta(meta::Command),
  Remove(remove::Command),
  Show(show::Command),
  Tag(tag::Command),
  Untag(untag::Command),
  Update(update::Command),
}

impl Command {
  pub fn call(&self, config: &Config, theme: &Theme) -> crate::Result<()> {
    match &self.command {
      IterationCommand::Add(cmd) => cmd.call(config, theme),
      IterationCommand::Create(cmd) => cmd.call(config, theme),
      IterationCommand::Graph(cmd) => cmd.call(config, theme),
      IterationCommand::Link(cmd) => cmd.call(config, theme),
      IterationCommand::List(cmd) => cmd.call(config, theme),
      IterationCommand::Meta(cmd) => cmd.call(config, theme),
      IterationCommand::Remove(cmd) => cmd.call(config, theme),
      IterationCommand::Show(cmd) => cmd.call(config, theme),
      IterationCommand::Tag(cmd) => cmd.call(config, theme),
      IterationCommand::Untag(cmd) => cmd.call(config, theme),
      IterationCommand::Update(cmd) => cmd.call(config, theme),
    }
  }
}

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

use std::path::Path;

use clap::{Args, Subcommand};

use crate::{cli, config::Settings, ui::theme::Theme};

/// Manage iterations (execution plans grouping tasks into phases).
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
  /// Dispatch to the appropriate iteration subcommand.
  pub fn call(&self, _settings: &Settings, theme: &Theme, data_dir: &Path) -> cli::Result<()> {
    match &self.command {
      IterationCommand::Add(cmd) => cmd.call(data_dir, theme),
      IterationCommand::Create(cmd) => cmd.call(data_dir, theme),
      IterationCommand::Graph(cmd) => cmd.call(data_dir, theme),
      IterationCommand::Link(cmd) => cmd.call(data_dir, theme),
      IterationCommand::List(cmd) => cmd.call(data_dir, theme),
      IterationCommand::Meta(cmd) => cmd.call(data_dir, theme),
      IterationCommand::Remove(cmd) => cmd.call(data_dir, theme),
      IterationCommand::Show(cmd) => cmd.call(data_dir, theme),
      IterationCommand::Tag(cmd) => cmd.call(data_dir, theme),
      IterationCommand::Untag(cmd) => cmd.call(data_dir, theme),
      IterationCommand::Update(cmd) => cmd.call(data_dir, theme),
    }
  }
}

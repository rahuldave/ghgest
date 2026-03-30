//! Task management commands for creating, updating, listing, and linking tasks.

mod create;
mod link;
mod list;
mod meta;
mod show;
mod tag;
mod untag;
mod update;

use clap::{Args, Subcommand};

use crate::cli::{self, AppContext};

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
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    match &self.command {
      TaskCommand::Create(cmd) => cmd.call(ctx),
      TaskCommand::Link(cmd) => cmd.call(ctx),
      TaskCommand::List(cmd) => cmd.call(ctx),
      TaskCommand::Meta(cmd) => cmd.call(ctx),
      TaskCommand::Show(cmd) => cmd.call(ctx),
      TaskCommand::Tag(cmd) => cmd.call(ctx),
      TaskCommand::Untag(cmd) => cmd.call(ctx),
      TaskCommand::Update(cmd) => cmd.call(ctx),
    }
  }
}

//! Task management commands for creating, updating, listing, and linking tasks.

mod block;
mod cancel;
mod complete;
mod create;
mod link;
mod list;
mod meta;
mod note;
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
  Block(block::Command),
  Cancel(cancel::Command),
  Complete(complete::Command),
  #[command(visible_alias = "new")]
  Create(create::Command),
  Link(link::Command),
  #[command(visible_alias = "ls")]
  List(list::Command),
  Meta(meta::Command),
  Note(note::Command),
  #[command(visible_alias = "view")]
  Show(show::Command),
  Tag(tag::Command),
  Untag(untag::Command),
  #[command(visible_alias = "edit")]
  Update(update::Command),
}

impl Command {
  /// Route to the appropriate task subcommand.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    match &self.command {
      TaskCommand::Block(cmd) => cmd.call(ctx),
      TaskCommand::Cancel(cmd) => cmd.call(ctx),
      TaskCommand::Complete(cmd) => cmd.call(ctx),
      TaskCommand::Create(cmd) => cmd.call(ctx),
      TaskCommand::Link(cmd) => cmd.call(ctx),
      TaskCommand::List(cmd) => cmd.call(ctx),
      TaskCommand::Meta(cmd) => cmd.call(ctx),
      TaskCommand::Note(cmd) => cmd.call(ctx),
      TaskCommand::Show(cmd) => cmd.call(ctx),
      TaskCommand::Tag(cmd) => cmd.call(ctx),
      TaskCommand::Untag(cmd) => cmd.call(ctx),
      TaskCommand::Update(cmd) => cmd.call(ctx),
    }
  }
}

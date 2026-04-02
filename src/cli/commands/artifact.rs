//! CLI commands for managing artifacts (specs, ADRs, RFCs, notes, and other documents).

mod archive;
mod create;
mod list;
mod meta;
mod show;
mod tag;
mod untag;
mod update;

use clap::{Args, Subcommand};

use crate::cli::{self, AppContext};

/// Top-level artifact command that dispatches to subcommands.
#[derive(Debug, Args)]
pub struct Command {
  #[command(subcommand)]
  command: ArtifactCommand,
}

#[derive(Debug, Subcommand)]
enum ArtifactCommand {
  Archive(archive::Command),
  #[command(visible_alias = "new")]
  Create(create::Command),
  #[command(visible_alias = "ls")]
  List(list::Command),
  Meta(meta::Command),
  #[command(visible_alias = "view")]
  Show(show::Command),
  Tag(tag::Command),
  Untag(untag::Command),
  #[command(visible_alias = "edit")]
  Update(update::Command),
}

impl Command {
  /// Dispatch to the appropriate artifact subcommand.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    match &self.command {
      ArtifactCommand::Archive(cmd) => cmd.call(ctx),
      ArtifactCommand::Create(cmd) => cmd.call(ctx),
      ArtifactCommand::List(cmd) => cmd.call(ctx),
      ArtifactCommand::Meta(cmd) => cmd.call(ctx),
      ArtifactCommand::Show(cmd) => cmd.call(ctx),
      ArtifactCommand::Tag(cmd) => cmd.call(ctx),
      ArtifactCommand::Untag(cmd) => cmd.call(ctx),
      ArtifactCommand::Update(cmd) => cmd.call(ctx),
    }
  }
}

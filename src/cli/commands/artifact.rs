//! CLI commands for managing artifacts (specs, ADRs, RFCs, notes, and other documents).

mod archive;
mod create;
mod list;
mod meta;
mod show;
mod tag;
mod untag;
mod update;

use std::path::Path;

use clap::{Args, Subcommand};

use crate::{cli, config::Settings, ui::theme::Theme};

/// Top-level artifact command that dispatches to subcommands.
#[derive(Debug, Args)]
pub struct Command {
  #[command(subcommand)]
  command: ArtifactCommand,
}

#[derive(Debug, Subcommand)]
enum ArtifactCommand {
  Archive(archive::Command),
  Create(create::Command),
  List(list::Command),
  Meta(meta::Command),
  Show(show::Command),
  Tag(tag::Command),
  Untag(untag::Command),
  Update(update::Command),
}

impl Command {
  /// Dispatch to the appropriate artifact subcommand.
  pub fn call(&self, _settings: &Settings, theme: &Theme, data_dir: &Path) -> cli::Result<()> {
    match &self.command {
      ArtifactCommand::Archive(cmd) => cmd.call(data_dir, theme),
      ArtifactCommand::Create(cmd) => cmd.call(data_dir, theme),
      ArtifactCommand::List(cmd) => cmd.call(data_dir, theme),
      ArtifactCommand::Meta(cmd) => cmd.call(data_dir, theme),
      ArtifactCommand::Show(cmd) => cmd.call(data_dir, theme),
      ArtifactCommand::Tag(cmd) => cmd.call(data_dir, theme),
      ArtifactCommand::Untag(cmd) => cmd.call(data_dir, theme),
      ArtifactCommand::Update(cmd) => cmd.call(data_dir, theme),
    }
  }
}

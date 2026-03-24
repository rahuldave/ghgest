mod archive;
mod create;
mod list;
mod meta;
mod show;
mod tag;
mod untag;
mod update;

use clap::{Args, Subcommand};

use crate::{config::Config, ui::theme::Theme};

/// Manage artifacts (specs, ADRs, RFCs, notes, and other documents)
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
  pub fn call(&self, config: &Config, theme: &Theme) -> crate::Result<()> {
    match &self.command {
      ArtifactCommand::Archive(cmd) => cmd.call(config, theme),
      ArtifactCommand::Create(cmd) => cmd.call(config, theme),
      ArtifactCommand::List(cmd) => cmd.call(config, theme),
      ArtifactCommand::Meta(cmd) => cmd.call(config, theme),
      ArtifactCommand::Show(cmd) => cmd.call(config, theme),
      ArtifactCommand::Tag(cmd) => cmd.call(config, theme),
      ArtifactCommand::Untag(cmd) => cmd.call(config, theme),
      ArtifactCommand::Update(cmd) => cmd.call(config, theme),
    }
  }
}

use clap::Args;

use crate::{AppContext, actions, cli::Error, ui::json};

/// Show a single note.
#[derive(Args, Debug)]
pub struct Command {
  /// The note ID or prefix.
  id: String,
  #[command(flatten)]
  output: json::Flags,
}

impl Command {
  /// Render the resolved note's body and metadata.
  pub async fn call(&self, context: &AppContext) -> Result<(), Error> {
    log::debug!("task note show: entry");
    actions::note::show(context, &self.id, &self.output).await
  }
}

use clap::Args;

use crate::{AppContext, actions, cli::Error, ui::json};

/// Update a note's body.
#[derive(Args, Debug)]
pub struct Command {
  /// The note ID or prefix.
  id: String,
  /// The new body text (use `-` to open `$EDITOR`).
  #[arg(long, short)]
  body: Option<String>,
  #[command(flatten)]
  output: json::Flags,
}

impl Command {
  /// Replace the resolved note's body within a recorded transaction.
  pub async fn call(&self, context: &AppContext) -> Result<(), Error> {
    log::debug!("artifact note update: entry");
    actions::note::update(context, &self.id, self.body.as_deref(), "artifact", &self.output).await
  }
}

use clap::Args;

use crate::{AppContext, actions, cli::Error, ui::json};

/// Remove a tag from an artifact.
#[derive(Args, Debug)]
pub struct Command {
  /// The artifact ID or prefix.
  id: String,
  /// The tag label to remove.
  label: String,
  #[command(flatten)]
  output: json::Flags,
}

impl Command {
  /// Detach the given tag label from the resolved artifact within a recorded transaction.
  pub async fn call(&self, context: &AppContext) -> Result<(), Error> {
    actions::tag::untag::<actions::Artifact>(context, &self.id, &self.label, &self.output).await
  }
}

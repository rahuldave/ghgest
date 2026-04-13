use clap::Args;

use crate::{AppContext, actions, cli::Error, ui::json};

/// Remove a tag from an iteration.
#[derive(Args, Debug)]
pub struct Command {
  /// The iteration ID or prefix.
  id: String,
  /// The tag label to remove.
  label: String,
  #[command(flatten)]
  output: json::Flags,
}

impl Command {
  /// Detach the given tag label from the resolved iteration within a recorded transaction.
  pub async fn call(&self, context: &AppContext) -> Result<(), Error> {
    actions::tag::untag::<actions::Iteration>(context, &self.id, &self.label, &self.output).await
  }
}

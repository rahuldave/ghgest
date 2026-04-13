use clap::Args;

use crate::{AppContext, actions, cli::Error, ui::json};

/// Get a metadata value from a task by dot-delimited path.
#[derive(Args, Debug)]
pub struct Command {
  /// The task ID or prefix.
  id: String,
  /// The dot-delimited metadata path.
  path: String,
  #[command(flatten)]
  output: json::Flags,
}

impl Command {
  /// Resolve the task and print the metadata value at the given dot-path.
  pub async fn call(&self, context: &AppContext) -> Result<(), Error> {
    actions::meta::get::<actions::Task>(context, &self.id, &self.path, &self.output).await
  }
}

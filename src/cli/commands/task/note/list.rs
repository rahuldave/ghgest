use clap::Args;

use crate::{
  AppContext, actions,
  cli::{Error, limit::LimitArgs},
  ui::json,
};

/// List notes on a task.
#[derive(Args, Debug)]
pub struct Command {
  /// The task ID or prefix.
  id: String,
  #[command(flatten)]
  limit: LimitArgs,
  #[command(flatten)]
  output: json::Flags,
}

impl Command {
  /// Render notes attached to the resolved task.
  pub async fn call(&self, context: &AppContext) -> Result<(), Error> {
    log::debug!("task note list: entry");
    actions::note::list::<actions::Task>(context, &self.id, &self.limit, &self.output).await
  }
}

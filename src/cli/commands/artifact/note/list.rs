use clap::Args;

use crate::{
  AppContext, actions,
  cli::{Error, limit::LimitArgs},
  ui::json,
};

/// List notes on an artifact.
#[derive(Args, Debug)]
pub struct Command {
  /// The artifact ID or prefix.
  id: String,
  #[command(flatten)]
  limit: LimitArgs,
  #[command(flatten)]
  output: json::Flags,
}

impl Command {
  /// Render notes attached to the resolved artifact.
  pub async fn call(&self, context: &AppContext) -> Result<(), Error> {
    log::debug!("artifact note list: entry");
    actions::note::list::<actions::Artifact>(context, &self.id, &self.limit, &self.output).await
  }
}

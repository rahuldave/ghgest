use clap::Args;

use crate::{
  AppContext,
  cli::{Error, prompt},
  store::repo,
  ui::{components::SuccessMessage, json},
};

/// Delete a note from an artifact.
#[derive(Args, Debug)]
pub struct Command {
  /// The note ID or prefix.
  id: String,
  #[command(flatten)]
  output: json::Flags,
  /// Skip the interactive confirmation prompt.
  #[arg(long)]
  yes: bool,
}

impl Command {
  /// Confirm and delete the resolved note within a recorded transaction.
  pub async fn call(&self, context: &AppContext) -> Result<(), Error> {
    log::debug!("artifact note delete: entry");
    let project_id = context.project_id().as_ref().ok_or(Error::UninitializedProject)?;
    let conn = context.store().connect().await?;
    let note_id = repo::resolve::resolve_id(&conn, "notes", &self.id).await?;

    let before_note = repo::note::find_by_id(&conn, note_id.clone()).await?;
    if !prompt::confirm_destructive("delete", &format!("artifact note {}", note_id.short()), self.yes)? {
      log::info!("artifact note delete: aborted by user");
      return Ok(());
    }
    let tx = repo::transaction::begin(&conn, project_id, "artifact note delete").await?;
    repo::note::delete(&conn, &note_id).await?;
    if let Some(note) = &before_note {
      let before = serde_json::to_value(note)?;
      repo::transaction::record_event(&conn, tx.id(), "notes", &note_id.to_string(), "deleted", Some(&before)).await?;
    }

    let short_id = note_id.short();
    self
      .output
      .print_delete(|| SuccessMessage::new("deleted note").id(short_id.clone()).to_string())?;
    Ok(())
  }
}

use clap::Args;

use crate::{
  AppContext,
  cli::Error,
  store::repo,
  ui::{components::SuccessMessage, json},
};

/// Restore an archived project to active status.
#[derive(Args, Debug)]
pub struct Command {
  /// The project ID or prefix.
  id: String,
  #[command(flatten)]
  output: json::Flags,
}

impl Command {
  /// Clear the archived flag on the project and print a workspace reattach hint.
  pub async fn call(&self, context: &AppContext) -> Result<(), Error> {
    log::debug!("project unarchive: entry");
    let conn = context.store().connect().await?;

    let id = repo::resolve::resolve_id(&conn, repo::resolve::Table::Projects, &self.id).await?;
    let project = repo::project::find_by_id(&conn, id)
      .await?
      .ok_or_else(|| Error::Argument(format!("project {} not found", self.id)))?;

    repo::project::unarchive(&conn, project.id()).await?;

    let short_id = project.id().short();
    if self.output.json {
      let json = serde_json::json!({
        "id": project.id().to_string(),
        "root": project.root().display().to_string(),
      });
      println!("{}", serde_json::to_string_pretty(&json)?);
    } else if self.output.quiet {
      println!("{short_id}");
    } else {
      let message = SuccessMessage::new("unarchived project")
        .id(short_id.clone())
        .field("root", project.root().display().to_string());
      println!("{message}");
      println!();
      println!(
        "  Workspace paths are not automatically restored \u{2014} run `gest project attach {}` to re-enable workspace discovery.",
        short_id
      );
    }
    Ok(())
  }
}

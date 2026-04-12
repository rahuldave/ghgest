use clap::Args;

use crate::{
  AppContext,
  cli::{Error, prompt},
  store::repo,
  ui::components::SuccessMessage,
};

/// Delete a project and all of its owned entities.
///
/// This is a hard delete that cascades through every task, iteration, and
/// artifact owned by the project, along with all their notes, tags, and
/// relationships. The operation writes tombstone files and is **not**
/// reversible via `gest undo`.
#[derive(Args, Debug)]
pub struct Command {
  /// The project ID or prefix.
  id: String,
  /// Skip the interactive confirmation prompt.
  #[arg(long)]
  yes: bool,
}

impl Command {
  /// Confirm and cascade-delete the project along with all owned entities, writing tombstone files.
  pub async fn call(&self, context: &AppContext) -> Result<(), Error> {
    log::debug!("project delete: entry");
    let conn = context.store().connect().await?;

    let id = repo::resolve::resolve_id(&conn, repo::resolve::Table::Projects, &self.id).await?;
    let project = repo::project::find_by_id(&conn, id.clone())
      .await?
      .ok_or_else(|| Error::Argument(format!("project {} not found", id.short())))?;

    let task_count = count_owned(&conn, "tasks", &id).await?;
    let iteration_count = count_owned(&conn, "iterations", &id).await?;
    let artifact_count = count_owned(&conn, "artifacts", &id).await?;

    let target = format!(
      "project {} ({}). This will delete {} tasks, {} iterations, {} artifacts, \
      and all associated notes, tags, and relationships. This cannot be undone",
      id.short(),
      project.root().display(),
      task_count,
      iteration_count,
      artifact_count,
    );
    if !prompt::confirm_destructive("delete", &target, self.yes)? {
      log::info!("project delete: aborted by user");
      return Ok(());
    }

    let deleted_at = chrono::Utc::now();
    let summary = repo::project::delete(&conn, &id, context.gest_dir().as_deref(), deleted_at).await?;

    let short_id = id.short();
    log::info!("deleted project");
    let msg = SuccessMessage::new("deleted project")
      .id(short_id)
      .field("root", project.root().display().to_string())
      .field("tasks", summary.tasks.to_string())
      .field("iterations", summary.iterations.to_string())
      .field("artifacts", summary.artifacts.to_string())
      .field("notes", summary.notes.to_string())
      .field("tags", summary.tags.to_string())
      .field("relationships", summary.relationships.to_string());
    println!("{msg}");
    Ok(())
  }
}

/// Count rows in `table` belonging to the given project.
async fn count_owned(
  conn: &libsql::Connection,
  table: &str,
  project_id: &crate::store::model::primitives::Id,
) -> Result<i64, Error> {
  let sql = format!("SELECT COUNT(*) FROM {table} WHERE project_id = ?1");
  let mut rows = conn
    .query(&sql, [project_id.to_string()])
    .await
    .map_err(crate::store::Error::from)?;
  let row = rows.next().await.map_err(crate::store::Error::from)?.unwrap();
  let count: i64 = row.get(0).map_err(crate::store::Error::from)?;
  Ok(count)
}

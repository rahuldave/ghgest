use chrono::Utc;
use libsql::{Connection, Error as DbError, Value};
use serde_json::Value as JsonValue;

use crate::store::model::{
  Error as ModelError,
  primitives::Id,
  transaction::{Event, Model},
};

/// Errors that can occur in transaction repository operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// The underlying database driver returned an error.
  #[error(transparent)]
  Database(#[from] DbError),
  /// A row could not be converted into a domain model.
  #[error(transparent)]
  Model(#[from] ModelError),
  /// No undoable transaction found.
  #[error("nothing to undo")]
  NothingToUndo,
}

/// Begin a new transaction for the given command.
pub async fn begin(conn: &Connection, project_id: &Id, command: &str) -> Result<Model, Error> {
  let id = Id::new();
  let now = Utc::now();
  conn
    .execute(
      "INSERT INTO transactions (id, project_id, command, created_at) VALUES (?1, ?2, ?3, ?4)",
      [
        id.to_string(),
        project_id.to_string(),
        command.to_string(),
        now.to_rfc3339(),
      ],
    )
    .await?;

  find_by_id(conn, id)
    .await?
    .ok_or_else(|| Error::Model(ModelError::InvalidValue("transaction not found after insert".into())))
}

/// Find a transaction by its ID.
pub async fn find_by_id(conn: &Connection, id: impl Into<Id>) -> Result<Option<Model>, Error> {
  let id = id.into();
  let mut rows = conn
    .query(
      "SELECT id, project_id, command, created_at, undone_at FROM transactions WHERE id = ?1",
      [id.to_string()],
    )
    .await?;

  match rows.next().await? {
    Some(row) => Ok(Some(Model::try_from(row)?)),
    None => Ok(None),
  }
}

/// Return the most recent non-undone transaction for a project.
pub async fn latest_undoable(conn: &Connection, project_id: &Id) -> Result<Option<Model>, Error> {
  let mut rows = conn
    .query(
      "SELECT id, project_id, command, created_at, undone_at FROM transactions \
        WHERE project_id = ?1 AND undone_at IS NULL \
        ORDER BY created_at DESC LIMIT 1",
      [project_id.to_string()],
    )
    .await?;

  match rows.next().await? {
    Some(row) => Ok(Some(Model::try_from(row)?)),
    None => Ok(None),
  }
}

/// Return the N most recent non-undone transactions for a project.
pub async fn latest_undoable_n(conn: &Connection, project_id: &Id, n: u32) -> Result<Vec<Model>, Error> {
  let mut rows = conn
    .query(
      "SELECT id, project_id, command, created_at, undone_at FROM transactions \
        WHERE project_id = ?1 AND undone_at IS NULL \
        ORDER BY created_at DESC LIMIT ?2",
      libsql::params![project_id.to_string(), n],
    )
    .await?;

  let mut results = Vec::new();
  while let Some(row) = rows.next().await? {
    results.push(Model::try_from(row)?);
  }
  Ok(results)
}

/// Record a change event within a transaction.
pub async fn record_event(
  conn: &Connection,
  transaction_id: &Id,
  table_name: &str,
  row_id: &str,
  event_type: &str,
  before_data: Option<&JsonValue>,
) -> Result<(), Error> {
  let id = Id::new();
  let before: Value = match before_data {
    Some(d) => Value::from(d.to_string()),
    None => Value::Null,
  };
  conn
    .execute(
      "INSERT INTO transaction_events (id, transaction_id, before_data, event_type, row_id, table_name) \
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
      libsql::params![
        id.to_string(),
        transaction_id.to_string(),
        before,
        event_type.to_string(),
        row_id.to_string(),
        table_name.to_string(),
      ],
    )
    .await?;
  Ok(())
}

/// Undo a transaction by replaying its events in reverse.
pub async fn undo(conn: &Connection, transaction_id: &Id) -> Result<String, Error> {
  // Get the transaction
  let tx = find_by_id(conn, transaction_id.clone())
    .await?
    .ok_or(Error::NothingToUndo)?;

  // Get all events in reverse order
  let mut rows = conn
    .query(
      "SELECT id, transaction_id, before_data, created_at, event_type, row_id, table_name \
        FROM transaction_events WHERE transaction_id = ?1 ORDER BY created_at DESC",
      [transaction_id.to_string()],
    )
    .await?;

  let mut events = Vec::new();
  while let Some(row) = rows.next().await? {
    events.push(Event::try_from(row)?);
  }

  // Replay each event
  for event in &events {
    match event.event_type() {
      "created" => {
        // Undo a create by deleting the row
        let sql = format!("DELETE FROM {} WHERE id = ?1", event.table_name());
        conn.execute(&sql, [event.row_id().to_string()]).await?;
      }
      "modified" => {
        // Undo a modify by restoring before_data
        if let Some(before) = event.before_data()
          && let Some(obj) = before.as_object()
        {
          let mut sets = Vec::new();
          let mut params: Vec<String> = Vec::new();
          let mut idx = 1;
          for (key, val) in obj {
            if key == "id" {
              continue;
            }
            sets.push(format!("{key} = ?{idx}"));
            params.push(match val {
              JsonValue::String(s) => s.clone(),
              JsonValue::Null => continue,
              other => other.to_string(),
            });
            idx += 1;
          }
          if !sets.is_empty() {
            params.push(event.row_id().to_string());
            let sql = format!(
              "UPDATE {} SET {} WHERE id = ?{idx}",
              event.table_name(),
              sets.join(", ")
            );
            conn
              .execute(&sql, libsql::params_from_iter(params.iter().map(|s| s.as_str())))
              .await?;
          }
        }
      }
      "deleted" => {
        // Undo a delete by re-inserting from before_data
        if let Some(before) = event.before_data()
          && let Some(obj) = before.as_object()
        {
          let keys: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
          let placeholders: Vec<String> = (1..=keys.len()).map(|i| format!("?{i}")).collect();
          let values: Vec<String> = obj
            .values()
            .map(|v| match v {
              JsonValue::String(s) => s.clone(),
              JsonValue::Null => String::new(),
              other => other.to_string(),
            })
            .collect();
          let sql = format!(
            "INSERT OR IGNORE INTO {} ({}) VALUES ({})",
            event.table_name(),
            keys.join(", "),
            placeholders.join(", ")
          );
          conn
            .execute(&sql, libsql::params_from_iter(values.iter().map(|s| s.as_str())))
            .await?;
        }
      }
      _ => {}
    }
  }

  // Mark the transaction as undone
  conn
    .execute(
      "UPDATE transactions SET undone_at = ?1 WHERE id = ?2",
      [Utc::now().to_rfc3339(), transaction_id.to_string()],
    )
    .await?;

  Ok(tx.command().to_string())
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use tempfile::TempDir;

  use super::*;
  use crate::store::{self, Db, model::Project};

  async fn setup() -> (Arc<Db>, Connection, TempDir, Id) {
    let (store, tmp) = store::open_temp().await.unwrap();
    let conn = store.connect().await.unwrap();
    let project = Project::new("/tmp/tx-test".into());
    conn
      .execute(
        "INSERT INTO projects (id, root, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        [
          project.id().to_string(),
          project.root().to_string_lossy().into_owned(),
          project.created_at().to_rfc3339(),
          project.updated_at().to_rfc3339(),
        ],
      )
      .await
      .unwrap();
    let pid = project.id().clone();
    (store, conn, tmp, pid)
  }

  mod begin_fn {
    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test]
    async fn it_creates_a_transaction() {
      let (_store, conn, _tmp, pid) = setup().await;

      let tx = begin(&conn, &pid, "task create").await.unwrap();

      assert_eq!(tx.command(), "task create");
      assert!(tx.undone_at().is_none());
    }
  }

  mod latest_undoable_fn {
    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test]
    async fn it_returns_most_recent() {
      let (_store, conn, _tmp, pid) = setup().await;

      begin(&conn, &pid, "first").await.unwrap();
      let second = begin(&conn, &pid, "second").await.unwrap();

      let latest = latest_undoable(&conn, &pid).await.unwrap().unwrap();

      assert_eq!(latest.id(), second.id());
    }

    #[tokio::test]
    async fn it_returns_none_when_empty() {
      let (_store, conn, _tmp, pid) = setup().await;

      let latest = latest_undoable(&conn, &pid).await.unwrap();

      assert!(latest.is_none());
    }
  }

  mod latest_undoable_n_fn {
    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test]
    async fn it_returns_n_most_recent() {
      let (_store, conn, _tmp, pid) = setup().await;

      begin(&conn, &pid, "first").await.unwrap();
      begin(&conn, &pid, "second").await.unwrap();
      begin(&conn, &pid, "third").await.unwrap();

      let results = latest_undoable_n(&conn, &pid, 2).await.unwrap();

      assert_eq!(results.len(), 2);
      assert_eq!(results[0].command(), "third");
      assert_eq!(results[1].command(), "second");
    }

    #[tokio::test]
    async fn it_returns_fewer_when_not_enough() {
      let (_store, conn, _tmp, pid) = setup().await;

      begin(&conn, &pid, "only one").await.unwrap();

      let results = latest_undoable_n(&conn, &pid, 5).await.unwrap();

      assert_eq!(results.len(), 1);
      assert_eq!(results[0].command(), "only one");
    }

    #[tokio::test]
    async fn it_returns_empty_when_none() {
      let (_store, conn, _tmp, pid) = setup().await;

      let results = latest_undoable_n(&conn, &pid, 3).await.unwrap();

      assert!(results.is_empty());
    }
  }

  mod undo_fn {
    use pretty_assertions::assert_eq;

    use super::*;

    #[tokio::test]
    async fn it_undoes_a_create_by_deleting() {
      let (_store, conn, _tmp, pid) = setup().await;

      // Create a task
      let task_id = Id::new();
      conn
        .execute(
          "INSERT INTO tasks (id, project_id, title) VALUES (?1, ?2, ?3)",
          [task_id.to_string(), pid.to_string(), "Undo me".to_string()],
        )
        .await
        .unwrap();

      // Record the transaction
      let tx = begin(&conn, &pid, "task create").await.unwrap();
      record_event(&conn, tx.id(), "tasks", &task_id.to_string(), "created", None)
        .await
        .unwrap();

      // Undo it
      let cmd = undo(&conn, tx.id()).await.unwrap();
      assert_eq!(cmd, "task create");

      // Task should be gone
      let mut rows = conn
        .query("SELECT id FROM tasks WHERE id = ?1", [task_id.to_string()])
        .await
        .unwrap();
      assert!(rows.next().await.unwrap().is_none());
    }
  }
}

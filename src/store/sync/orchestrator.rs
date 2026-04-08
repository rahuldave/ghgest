//! Top-level coordinator for the per-entity sync layout (ADR-0016).
//!
//! Phase 3 will replace `super::reader::import_all` / `super::writer::export_all`
//! with calls into this module. Until then the orchestrator exists only as a
//! scaffold so that phase 2 entity adapters can land independently and so the
//! per-entity modules are linked into the binary (otherwise the empty stubs
//! would be flagged as dead code).

use std::path::Path;

use libsql::Connection;

use super::{Error, entities};
use crate::store::model::primitives::Id;

/// Import every per-entity file under `gest_dir` into SQLite.
///
/// This walks every entity adapter in a fixed order. The current order is
/// chosen so that referenced entities (authors, tags, projects) land before
/// the things that reference them (tasks, artifacts, iterations, notes,
/// relationships, events). Phase 3 will revisit the order if any adapter
/// surfaces a hard dependency that demands a different sequence.
#[allow(dead_code)] // wired up in phase 3
pub async fn import_all(conn: &Connection, project_id: &Id, gest_dir: &Path) -> Result<(), Error> {
  entities::project::read_all(conn, project_id, gest_dir).await?;
  entities::author::read_all(conn, project_id, gest_dir).await?;
  entities::tag::read_all(conn, project_id, gest_dir).await?;
  entities::task::read_all(conn, project_id, gest_dir).await?;
  entities::artifact::read_all(conn, project_id, gest_dir).await?;
  entities::iteration::read_all(conn, project_id, gest_dir).await?;
  entities::relationship::read_all(conn, project_id, gest_dir).await?;
  entities::event::read_all(conn, project_id, gest_dir).await?;
  Ok(())
}

/// Export every entity in SQLite to its per-entity file under `gest_dir`.
#[allow(dead_code)] // wired up in phase 3
pub async fn export_all(conn: &Connection, project_id: &Id, gest_dir: &Path) -> Result<(), Error> {
  entities::project::write_all(conn, project_id, gest_dir).await?;
  entities::author::write_all(conn, project_id, gest_dir).await?;
  entities::tag::write_all(conn, project_id, gest_dir).await?;
  entities::task::write_all(conn, project_id, gest_dir).await?;
  entities::artifact::write_all(conn, project_id, gest_dir).await?;
  entities::iteration::write_all(conn, project_id, gest_dir).await?;
  entities::relationship::write_all(conn, project_id, gest_dir).await?;
  entities::event::write_all(conn, project_id, gest_dir).await?;
  Ok(())
}

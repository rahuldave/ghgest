//! Per-entity sync adapter for relationships.
//!
//! Stub created in the foundation task; the read/write bodies land in the
//! `Per-entity sync: relationship/<id>.yaml per-edge files` task in phase 2.

use std::path::Path;

use libsql::Connection;

use crate::store::{model::primitives::Id, sync::Error};

/// Import every relationship file under `gest_dir` into SQLite.
pub async fn read_all(_conn: &Connection, _project_id: &Id, _gest_dir: &Path) -> Result<(), Error> {
  Ok(())
}

/// Export every relationship in SQLite to its per-entity file under `gest_dir`.
pub async fn write_all(_conn: &Connection, _project_id: &Id, _gest_dir: &Path) -> Result<(), Error> {
  Ok(())
}

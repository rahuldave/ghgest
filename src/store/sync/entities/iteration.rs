//! Per-entity sync adapter for iterations.
//!
//! Stub created in the foundation task; the read/write bodies land in the
//! `Per-entity sync: iteration/<id>.yaml + iteration notes` task in phase 2.

use std::path::Path;

use libsql::Connection;

use crate::store::{model::primitives::Id, sync::Error};

/// Import every iteration file under `gest_dir` into SQLite.
pub async fn read_all(_conn: &Connection, _project_id: &Id, _gest_dir: &Path) -> Result<(), Error> {
  Ok(())
}

/// Export every iteration in SQLite to its per-entity file under `gest_dir`.
pub async fn write_all(_conn: &Connection, _project_id: &Id, _gest_dir: &Path) -> Result<(), Error> {
  Ok(())
}

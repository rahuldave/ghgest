//! Per-entity sync adapter for artifacts.
//!
//! Stub created in the foundation task; the read/write bodies land in the
//! `Per-entity sync: artifact/<id>.md ...` task in phase 2.

use std::path::Path;

use libsql::Connection;

use crate::store::{model::primitives::Id, sync::Error};

/// Import every artifact file under `gest_dir` into SQLite.
pub async fn read_all(_conn: &Connection, _project_id: &Id, _gest_dir: &Path) -> Result<(), Error> {
  Ok(())
}

/// Export every artifact in SQLite to its per-entity file under `gest_dir`.
pub async fn write_all(_conn: &Connection, _project_id: &Id, _gest_dir: &Path) -> Result<(), Error> {
  Ok(())
}

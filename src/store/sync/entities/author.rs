//! Per-entity sync adapter for authors.
//!
//! Stub created in the foundation task; the read/write bodies land in the
//! `Per-entity sync: author/<id>.yaml` task in phase 2.

use std::path::Path;

use libsql::Connection;

use crate::store::{model::primitives::Id, sync::Error};

/// Import every author file under `gest_dir` into SQLite.
pub async fn read_all(_conn: &Connection, _project_id: &Id, _gest_dir: &Path) -> Result<(), Error> {
  Ok(())
}

/// Export every author in SQLite to its per-entity file under `gest_dir`.
pub async fn write_all(_conn: &Connection, _project_id: &Id, _gest_dir: &Path) -> Result<(), Error> {
  Ok(())
}

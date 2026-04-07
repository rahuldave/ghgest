use chrono::{DateTime, Utc};
use libsql::Row;
use serde::{Deserialize, Serialize};

use super::{Error, primitives::Id};

/// Tracks content digests for local sync mirror files.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Model {
  digest: String,
  file_path: String,
  project_id: Id,
  synced_at: DateTime<Utc>,
}

impl Model {
  /// The SHA-256 content digest of the file.
  pub fn digest(&self) -> &str {
    &self.digest
  }

  /// The relative path of the synced file within `.gest/`.
  pub fn file_path(&self) -> &str {
    &self.file_path
  }

  /// The project this digest belongs to.
  pub fn project_id(&self) -> &Id {
    &self.project_id
  }

  /// When this digest was last recorded.
  pub fn synced_at(&self) -> &DateTime<Utc> {
    &self.synced_at
  }
}

/// Expects columns in order: `file_path`, `project_id`, `digest`, `synced_at`.
impl TryFrom<Row> for Model {
  type Error = Error;

  fn try_from(row: Row) -> Result<Self, Self::Error> {
    let file_path: String = row.get(0)?;
    let project_id: String = row.get(1)?;
    let digest: String = row.get(2)?;
    let synced_at: String = row.get(3)?;

    let project_id: Id = project_id.parse().map_err(Error::InvalidValue)?;
    let synced_at = DateTime::parse_from_rfc3339(&synced_at)
      .map(|dt| dt.with_timezone(&Utc))
      .map_err(|e| Error::InvalidValue(e.to_string()))?;

    Ok(Self {
      digest,
      file_path,
      project_id,
      synced_at,
    })
  }
}

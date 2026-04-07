use chrono::{DateTime, Utc};
use libsql::Row;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Error, primitives::Id};

/// A persistent document (spec, ADR, design doc, etc.) within a project.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Model {
  archived_at: Option<DateTime<Utc>>,
  #[serde(skip)]
  body: String,
  created_at: DateTime<Utc>,
  id: Id,
  metadata: Value,
  project_id: Id,
  title: String,
  updated_at: DateTime<Utc>,
}

impl Model {
  /// When this artifact was archived, if at all.
  pub fn archived_at(&self) -> Option<&DateTime<Utc>> {
    self.archived_at.as_ref()
  }

  /// The artifact's markdown body content.
  pub fn body(&self) -> &str {
    &self.body
  }

  /// When this artifact was first created.
  pub fn created_at(&self) -> &DateTime<Utc> {
    &self.created_at
  }

  /// The unique identifier for this artifact.
  pub fn id(&self) -> &Id {
    &self.id
  }

  /// Whether this artifact is archived.
  pub fn is_archived(&self) -> bool {
    self.archived_at.is_some()
  }

  /// Custom metadata stored as JSON.
  pub fn metadata(&self) -> &Value {
    &self.metadata
  }

  /// The project this artifact belongs to.
  pub fn project_id(&self) -> &Id {
    &self.project_id
  }

  /// The artifact's title.
  pub fn title(&self) -> &str {
    &self.title
  }

  /// When this artifact was last modified.
  pub fn updated_at(&self) -> &DateTime<Utc> {
    &self.updated_at
  }
}

/// Expects columns in order: `id`, `project_id`, `archived_at`, `body`, `created_at`,
/// `metadata`, `title`, `updated_at`.
impl TryFrom<Row> for Model {
  type Error = Error;

  fn try_from(row: Row) -> Result<Self, Self::Error> {
    let id: String = row.get(0)?;
    let project_id: String = row.get(1)?;
    let archived_at: Option<String> = row.get(2)?;
    let body: String = row.get(3)?;
    let created_at: String = row.get(4)?;
    let metadata: String = row.get(5)?;
    let title: String = row.get(6)?;
    let updated_at: String = row.get(7)?;

    let archived_at = archived_at
      .map(|s| {
        DateTime::parse_from_rfc3339(&s)
          .map(|dt| dt.with_timezone(&Utc))
          .map_err(|e| Error::InvalidValue(e.to_string()))
      })
      .transpose()?;
    let created_at = DateTime::parse_from_rfc3339(&created_at)
      .map(|dt| dt.with_timezone(&Utc))
      .map_err(|e| Error::InvalidValue(e.to_string()))?;
    let id: Id = id.parse().map_err(Error::InvalidValue)?;
    let metadata: Value = serde_json::from_str(&metadata).map_err(|e| Error::InvalidValue(e.to_string()))?;
    let project_id: Id = project_id.parse().map_err(Error::InvalidValue)?;
    let updated_at = DateTime::parse_from_rfc3339(&updated_at)
      .map(|dt| dt.with_timezone(&Utc))
      .map_err(|e| Error::InvalidValue(e.to_string()))?;

    Ok(Self {
      archived_at,
      body,
      created_at,
      id,
      metadata,
      project_id,
      title,
      updated_at,
    })
  }
}

/// Parameters for creating a new artifact.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct New {
  pub body: String,
  pub metadata: Option<Value>,
  pub title: String,
}

/// Optional fields for updating an existing artifact.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Patch {
  pub body: Option<String>,
  pub metadata: Option<Value>,
  pub title: Option<String>,
}

/// Criteria for filtering artifacts.
#[derive(Clone, Debug, Default)]
pub struct Filter {
  pub all: bool,
  pub only_archived: bool,
  pub tag: Option<String>,
}

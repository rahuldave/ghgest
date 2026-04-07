use chrono::{DateTime, Utc};
use libsql::Row;
use serde::{Deserialize, Serialize};

use super::{
  Error,
  primitives::{EntityType, Id},
};

/// A note attached to an entity.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Model {
  author_id: Option<Id>,
  body: String,
  created_at: DateTime<Utc>,
  entity_id: Id,
  entity_type: EntityType,
  id: Id,
  updated_at: DateTime<Utc>,
}

impl Model {
  /// The author who created this note.
  pub fn author_id(&self) -> Option<&Id> {
    self.author_id.as_ref()
  }

  /// The note's content.
  pub fn body(&self) -> &str {
    &self.body
  }

  /// When this note was first created.
  pub fn created_at(&self) -> &DateTime<Utc> {
    &self.created_at
  }

  /// The entity this note is attached to.
  pub fn entity_id(&self) -> &Id {
    &self.entity_id
  }

  /// The type of entity this note is attached to.
  pub fn entity_type(&self) -> EntityType {
    self.entity_type
  }

  /// The unique identifier for this note.
  pub fn id(&self) -> &Id {
    &self.id
  }

  /// When this note was last modified.
  pub fn updated_at(&self) -> &DateTime<Utc> {
    &self.updated_at
  }
}

/// Expects columns in order: `id`, `entity_id`, `entity_type`, `author_id`, `body`,
/// `created_at`, `updated_at`.
impl TryFrom<Row> for Model {
  type Error = Error;

  fn try_from(row: Row) -> Result<Self, Self::Error> {
    let id: String = row.get(0)?;
    let entity_id: String = row.get(1)?;
    let entity_type: String = row.get(2)?;
    let author_id: Option<String> = row.get(3)?;
    let body: String = row.get(4)?;
    let created_at: String = row.get(5)?;
    let updated_at: String = row.get(6)?;

    let author_id = author_id
      .map(|s| s.parse::<Id>())
      .transpose()
      .map_err(Error::InvalidValue)?;
    let created_at = DateTime::parse_from_rfc3339(&created_at)
      .map(|dt| dt.with_timezone(&Utc))
      .map_err(|e| Error::InvalidValue(e.to_string()))?;
    let entity_id: Id = entity_id.parse().map_err(Error::InvalidValue)?;
    let entity_type: EntityType = entity_type.parse().map_err(Error::InvalidValue)?;
    let id: Id = id.parse().map_err(Error::InvalidValue)?;
    let updated_at = DateTime::parse_from_rfc3339(&updated_at)
      .map(|dt| dt.with_timezone(&Utc))
      .map_err(|e| Error::InvalidValue(e.to_string()))?;

    Ok(Self {
      author_id,
      body,
      created_at,
      entity_id,
      entity_type,
      id,
      updated_at,
    })
  }
}

/// Parameters for creating a new note.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct New {
  pub author_id: Option<Id>,
  pub body: String,
}

/// Optional fields for updating an existing note.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Patch {
  pub body: Option<String>,
}

use libsql::Row;
use serde::{Deserialize, Serialize};

use super::{
  Error,
  primitives::{EntityType, Id},
};

/// A join between an entity and a tag.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Model {
  entity_id: Id,
  entity_type: EntityType,
  tag_id: Id,
}

impl Model {
  /// Create a new entity-tag association.
  pub fn new(entity_type: EntityType, entity_id: Id, tag_id: Id) -> Self {
    Self {
      entity_id,
      entity_type,
      tag_id,
    }
  }

  /// The ID of the tagged entity.
  pub fn entity_id(&self) -> &Id {
    &self.entity_id
  }

  /// The type of the tagged entity.
  pub fn entity_type(&self) -> EntityType {
    self.entity_type
  }

  /// The ID of the associated tag.
  pub fn tag_id(&self) -> &Id {
    &self.tag_id
  }
}

/// Expects columns in order: `entity_id`, `entity_type`, `tag_id`.
impl TryFrom<Row> for Model {
  type Error = Error;

  fn try_from(row: Row) -> Result<Self, Self::Error> {
    let entity_id: String = row.get(0)?;
    let entity_type: String = row.get(1)?;
    let tag_id: String = row.get(2)?;

    let entity_id: Id = entity_id.parse().map_err(Error::InvalidValue)?;
    let entity_type: EntityType = entity_type.parse().map_err(Error::InvalidValue)?;
    let tag_id: Id = tag_id.parse().map_err(Error::InvalidValue)?;

    Ok(Self {
      entity_id,
      entity_type,
      tag_id,
    })
  }
}

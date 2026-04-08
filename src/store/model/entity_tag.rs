use serde::{Deserialize, Serialize};

use super::primitives::{EntityType, Id};

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
}

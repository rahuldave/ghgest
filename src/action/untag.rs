use chrono::Utc;

use super::{Resolvable, Storable, Taggable};
use crate::{config::Settings, store};

/// Remove tags from an entity.
///
/// Resolves the ID prefix, reads the entity, removes the specified tags,
/// updates the timestamp, and writes the entity back to the store.
pub fn untag<T: Resolvable + Storable + Taggable>(
  config: &Settings,
  id_prefix: &str,
  tags: &[String],
) -> store::Result<T> {
  let id = T::resolve_id(config, id_prefix)?;
  let mut entity = T::read(config, &id)?;
  store::tag::remove_tags(entity.tags_mut(), tags);
  entity.set_updated_at(Utc::now());
  T::write(config, &entity)?;
  Ok(entity)
}

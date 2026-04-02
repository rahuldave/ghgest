use chrono::Utc;

use super::{Resolvable, Storable, Taggable};
use crate::{config::Settings, store};

/// Add tags to an entity, deduplicating with any existing tags.
///
/// Resolves the ID prefix, reads the entity, applies the tags, updates the
/// timestamp, and writes the entity back to the store.
pub fn tag<T: Resolvable + Storable + Taggable>(
  config: &Settings,
  id_prefix: &str,
  tags: &[String],
) -> store::Result<T> {
  let id = T::resolve_id(config, id_prefix)?;
  let mut entity = T::read(config, &id)?;
  store::tag::apply_tags(entity.tags_mut(), tags);
  entity.set_updated_at(Utc::now());
  T::write(config, &entity)?;
  Ok(entity)
}

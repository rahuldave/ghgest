use chrono::Utc;

use super::{Linkable, Resolvable, Storable};
use crate::{
  config::Settings,
  model::{
    Id,
    artifact::Artifact,
    link::{Link, RelationshipType},
  },
  store,
};

/// Create a bidirectional link between two entities.
///
/// When `artifact` is `true`, the target is resolved as an [`Artifact`] and no reciprocal link is
/// created (since artifacts are not [`Linkable`]).
pub fn link<T: Resolvable + Storable + Linkable>(
  config: &Settings,
  source_id: &str,
  target_id: &str,
  rel: &RelationshipType,
  artifact: bool,
) -> store::Result<(Id, Id)> {
  let id = T::resolve_id(config, source_id)?;

  let (resolved_target_id, ref_path) = if artifact {
    let tid = Artifact::resolve_id(config, target_id)?;
    let path = format!("{}/{tid}", Artifact::entity_prefix());
    (tid, path)
  } else {
    let tid = T::resolve_id(config, target_id)?;
    let path = format!("{}/{tid}", T::entity_prefix());
    (tid, path)
  };

  let mut source = T::read(config, &id)?;
  source.links_mut().push(Link {
    ref_: ref_path,
    rel: rel.clone(),
  });
  source.set_updated_at(Utc::now());
  T::write(config, &source)?;

  if !artifact {
    let mut target = T::read(config, &resolved_target_id)?;
    target.links_mut().push(Link {
      ref_: format!("{}/{id}", T::entity_prefix()),
      rel: rel.inverse(),
    });
    target.set_updated_at(Utc::now());
    T::write(config, &target)?;
  }

  Ok((id, resolved_target_id))
}

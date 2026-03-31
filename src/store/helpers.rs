use std::{fs, path::Path};

use super::fs::read_dir_files;

/// Load entities from one or two directories, parsing each file with the provided closure.
///
/// Files from `primary_dir` are always read (unless `skip_primary` is true).
/// Files from `secondary_dir` are read only when `include_secondary` is true.
pub fn load_entities_from_dirs<T>(
  primary_dir: &Path,
  secondary_dir: &Path,
  extension: &str,
  skip_primary: bool,
  include_secondary: bool,
  parse: impl Fn(&str) -> super::Result<T>,
) -> super::Result<Vec<T>> {
  let mut entities = Vec::new();

  if !skip_primary {
    for path in read_dir_files(primary_dir, extension)? {
      let content = fs::read_to_string(&path)?;
      let entity = parse(&content)?;
      entities.push(entity);
    }
  }

  if include_secondary {
    for path in read_dir_files(secondary_dir, extension)? {
      let content = fs::read_to_string(&path)?;
      let entity = parse(&content)?;
      entities.push(entity);
    }
  }

  Ok(entities)
}

/// Read a single entity file by ID, checking the active path first, then a secondary path.
///
/// The `entity_label` is used in the "not found" error message (e.g. "Task", "Artifact").
pub fn read_entity_file<T>(
  active: &Path,
  secondary: &Path,
  secondary_log_label: &str,
  entity_label: &str,
  id: &crate::model::Id,
  parse: impl Fn(&str) -> super::Result<T>,
) -> super::Result<T> {
  let path = if active.exists() {
    active
  } else if secondary.exists() {
    log::debug!("reading {secondary_log_label} {entity_label} {id}");
    secondary
  } else {
    return Err(super::Error::generic(format!("{entity_label} not found: '{id}'")));
  };

  log::trace!("reading {entity_label} from {}", path.display());
  let content = fs::read_to_string(path)?;
  parse(&content)
}

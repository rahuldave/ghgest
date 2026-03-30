use std::{
  fs,
  path::{Path, PathBuf},
};

use super::Error;
use crate::model::Id;

/// Create all required store subdirectories under `data_dir`.
pub fn ensure_dirs(data_dir: &Path) -> super::Result<()> {
  fs::create_dir_all(data_dir.join("artifacts"))?;
  fs::create_dir_all(data_dir.join("artifacts/archive"))?;
  fs::create_dir_all(data_dir.join("iterations"))?;
  fs::create_dir_all(data_dir.join("iterations/resolved"))?;
  fs::create_dir_all(data_dir.join("tasks"))?;
  fs::create_dir_all(data_dir.join("tasks/resolved"))?;
  Ok(())
}

/// Collect file stems in `dir` whose names start with `prefix` and have the given extension.
pub(crate) fn collect_prefix_matches(dir: &Path, extension: &str, prefix: &str) -> super::Result<Vec<String>> {
  let mut matches = Vec::new();
  for path in read_dir_files(dir, extension)? {
    if let Some(stem) = path.file_stem().and_then(|s| s.to_str())
      && stem.starts_with(prefix)
    {
      matches.push(stem.to_string());
    }
  }
  Ok(matches)
}

/// Write `content` to `dest` and remove `src` if it exists, ensuring store dirs first.
pub(crate) fn move_entity_file(data_dir: &Path, content: &str, dest: &Path, src: &Path) -> super::Result<()> {
  ensure_dirs(data_dir)?;
  fs::write(dest, content)?;
  if src.exists() {
    fs::remove_file(src)?;
  }
  Ok(())
}

/// List files in `dir` with the given extension, sorted by path.
pub(crate) fn read_dir_files(dir: &Path, extension: &str) -> super::Result<Vec<PathBuf>> {
  if !dir.exists() {
    return Ok(Vec::new());
  }

  let mut paths = Vec::new();
  for entry in fs::read_dir(dir)? {
    let entry = entry?;
    let path = entry.path();
    if path.is_file()
      && let Some(ext) = path.extension().and_then(|e| e.to_str())
      && ext == extension
    {
      paths.push(path);
    }
  }
  paths.sort();
  Ok(paths)
}

/// Resolve an ID prefix to a unique [`Id`], searching `primary_dir` first and
/// optionally falling back to `secondary_dir`.
pub(crate) fn resolve_id(
  primary_dir: &Path,
  secondary_dir: Option<&Path>,
  extension: &str,
  prefix: &str,
  include_secondary: bool,
  hint: &str,
) -> super::Result<Id> {
  let active_matches = collect_prefix_matches(primary_dir, extension, prefix)?;

  match active_matches.len() {
    1 => {
      return active_matches[0].parse().map_err(|e: String| Error::generic(e));
    }
    n if n > 1 => {
      let ids = active_matches.join(", ");
      return Err(Error::generic(format!(
        "Ambiguous ID prefix '{prefix}', matches: {ids}"
      )));
    }
    _ => {}
  }

  if include_secondary && let Some(secondary) = secondary_dir {
    let secondary_matches = collect_prefix_matches(secondary, extension, prefix)?;
    match secondary_matches.len() {
      0 => {}
      1 => {
        return secondary_matches[0].parse().map_err(|e: String| Error::generic(e));
      }
      _ => {
        let ids = secondary_matches.join(", ");
        return Err(Error::generic(format!(
          "Ambiguous ID prefix '{prefix}', matches: {ids}"
        )));
      }
    }
  }

  let mut msg = format!("{hint} not found: '{prefix}'");
  if !include_secondary {
    msg.push_str(" (try --all)");
  }
  Err(Error::generic(msg))
}

#[cfg(test)]
mod tests {
  mod ensure_dirs {
    #[test]
    fn it_creates_all_subdirectories() {
      let dir = tempfile::tempdir().unwrap();
      crate::store::ensure_dirs(dir.path()).unwrap();

      assert!(dir.path().join("artifacts").is_dir());
      assert!(dir.path().join("artifacts/archive").is_dir());
      assert!(dir.path().join("iterations").is_dir());
      assert!(dir.path().join("iterations/resolved").is_dir());
      assert!(dir.path().join("tasks").is_dir());
      assert!(dir.path().join("tasks/resolved").is_dir());
    }

    #[test]
    fn it_is_idempotent() {
      let dir = tempfile::tempdir().unwrap();
      crate::store::ensure_dirs(dir.path()).unwrap();
      crate::store::ensure_dirs(dir.path()).unwrap();

      assert!(dir.path().join("tasks").is_dir());
    }
  }
}

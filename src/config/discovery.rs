use std::{
  fmt::Write,
  path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

use super::env::GEST_DATA_DIR;

pub fn data_dir(config: &super::Config) -> crate::Result<PathBuf> {
  if let Ok(dir) = GEST_DATA_DIR.value() {
    let expanded = expand_path(&dir);
    log::debug!("$GEST_DATA_DIR is set, using it as data directory");
    log::trace!("data directory resolved to {}", expanded.display());
    return Ok(expanded);
  }

  if let Some(ref dir) = config.storage.data_dir {
    let expanded = expand_path(dir);
    log::debug!("config specifies data_dir, using it as data directory");
    log::trace!("data directory resolved to {}", expanded.display());
    return Ok(expanded);
  }

  let cwd = std::env::current_dir().map_err(crate::Error::from)?;

  if let Some(gest_dir) = walk_up_for(&cwd, ".gest") {
    log::debug!("found .gest directory, using it as data directory");
    log::trace!("data directory resolved to {}", gest_dir.display());
    return Ok(gest_dir);
  }

  let data_home =
    dir_spec::data_home().ok_or_else(|| crate::Error::generic("unable to determine data home directory"))?;

  if let Some(git_root) = walk_up_for_parent(&cwd, ".git") {
    let hash = path_hash(&git_root);
    let dir = data_home.join("gest").join(&hash);
    log::debug!("found .git root, using hashed path as data directory");
    log::trace!("data directory resolved to {}", dir.display());
    return Ok(dir);
  }

  let hash = path_hash(&cwd);
  let dir = data_home.join("gest").join(&hash);
  log::debug!("no .gest or .git found, using hashed cwd as data directory");
  log::trace!("data directory resolved to {}", dir.display());
  Ok(dir)
}

fn expand_env_vars(s: &str) -> String {
  let mut result = String::with_capacity(s.len());
  let mut chars = s.chars().peekable();

  while let Some(c) = chars.next() {
    if c == '$' {
      let braced = chars.peek() == Some(&'{');
      if braced {
        chars.next(); // consume '{'
      }

      let mut var_name = String::new();
      while let Some(&ch) = chars.peek() {
        if braced && ch == '}' {
          chars.next(); // consume '}'
          break;
        }
        if !braced && !ch.is_ascii_alphanumeric() && ch != '_' {
          break;
        }
        var_name.push(ch);
        chars.next();
      }

      if var_name.is_empty() {
        result.push('$');
        if braced {
          result.push('{');
        }
      } else {
        match std::env::var(&var_name) {
          Ok(val) => result.push_str(&val),
          Err(_) => {
            // Leave unresolved vars as-is
            result.push('$');
            if braced {
              result.push('{');
            }
            result.push_str(&var_name);
            if braced {
              result.push('}');
            }
          }
        }
      }
    } else {
      result.push(c);
    }
  }

  result
}

fn expand_path(path: &Path) -> PathBuf {
  let s = path.to_string_lossy();

  // Expand environment variables: $VAR and ${VAR}
  let expanded = expand_env_vars(&s);

  // Expand leading tilde
  let expanded = if let Some(rest) = expanded.strip_prefix("~/") {
    match dir_spec::home() {
      Some(home) => home.join(rest).to_string_lossy().into_owned(),
      None => expanded,
    }
  } else if expanded == "~" {
    match dir_spec::home() {
      Some(home) => home.to_string_lossy().into_owned(),
      None => expanded,
    }
  } else {
    expanded
  };

  let path = PathBuf::from(&expanded);

  // Resolve relative paths to absolute
  if path.is_relative() {
    std::env::current_dir().map(|cwd| cwd.join(&path)).unwrap_or(path)
  } else {
    path
  }
}

fn path_hash(path: &Path) -> String {
  let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
  let mut hasher = Sha256::new();
  hasher.update(canonical.as_os_str().as_encoded_bytes());
  let result = hasher.finalize();
  let mut hash = String::with_capacity(16);
  for b in &result[..8] {
    write!(hash, "{b:02x}").unwrap();
  }
  hash
}

fn walk_up_for(start: &Path, name: &str) -> Option<PathBuf> {
  let mut current = start.to_path_buf();
  loop {
    let candidate = current.join(name);
    if candidate.is_dir() {
      return Some(candidate);
    }
    if !current.pop() {
      return None;
    }
  }
}

fn walk_up_for_parent(start: &Path, name: &str) -> Option<PathBuf> {
  let mut current = start.to_path_buf();
  loop {
    let candidate = current.join(name);
    if candidate.is_dir() {
      return Some(current);
    }
    if !current.pop() {
      return None;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod data_dir {
    use super::*;
    use crate::config::{Config, StorageConfig};

    #[test]
    fn it_uses_gest_data_dir_env_var() {
      let dir = tempfile::tempdir().unwrap();
      let expected = dir.path().join("custom-data");
      std::fs::create_dir_all(&expected).unwrap();

      temp_env::with_var("GEST_DATA_DIR", Some(expected.to_str().unwrap()), || {
        let config = Config::default();
        let result = data_dir(&config).unwrap();
        assert_eq!(result, expected);
      });
    }

    #[test]
    fn it_uses_config_data_dir_when_env_unset() {
      let dir = tempfile::tempdir().unwrap();
      let expected = dir.path().join("config-data");
      std::fs::create_dir_all(&expected).unwrap();

      temp_env::with_var_unset("GEST_DATA_DIR", || {
        let config = Config {
          storage: StorageConfig {
            data_dir: Some(expected.clone()),
          },
          ..Config::default()
        };
        let result = data_dir(&config).unwrap();
        assert_eq!(result, expected);
      });
    }
  }

  mod expand_path {
    use super::*;

    #[test]
    fn it_expands_bare_tilde() {
      let path = expand_path(Path::new("~"));
      let home = dir_spec::home().unwrap();
      assert_eq!(path, home);
    }

    #[test]
    fn it_expands_braced_env_vars() {
      temp_env::with_var("GEST_TEST_BRACED", Some("/braced/path"), || {
        let path = expand_path(Path::new("${GEST_TEST_BRACED}/data"));
        assert_eq!(path, PathBuf::from("/braced/path/data"));
      });
    }

    #[test]
    fn it_expands_env_vars() {
      temp_env::with_var("GEST_TEST_EXPAND", Some("/custom/path"), || {
        let path = expand_path(Path::new("$GEST_TEST_EXPAND/data"));
        assert_eq!(path, PathBuf::from("/custom/path/data"));
      });
    }

    #[test]
    fn it_expands_tilde() {
      let path = expand_path(Path::new("~/some/dir"));
      let home = dir_spec::home().unwrap();
      assert_eq!(path, home.join("some/dir"));
    }

    #[test]
    fn it_handles_combined_tilde_and_env_var() {
      temp_env::with_var("GEST_TEST_SUBDIR", Some("mydata"), || {
        let path = expand_path(Path::new("~/$GEST_TEST_SUBDIR/gest"));
        let home = dir_spec::home().unwrap();
        assert_eq!(path, home.join("mydata/gest"));
      });
    }

    #[test]
    fn it_leaves_absolute_paths_unchanged() {
      let path = expand_path(Path::new("/absolute/path"));
      assert_eq!(path, PathBuf::from("/absolute/path"));
    }

    #[test]
    fn it_preserves_unresolved_vars() {
      temp_env::with_var_unset("GEST_NONEXISTENT_VAR", || {
        let path = expand_path(Path::new("$GEST_NONEXISTENT_VAR/data"));
        assert!(path.to_string_lossy().contains("$GEST_NONEXISTENT_VAR"));
      });
    }

    #[test]
    fn it_resolves_relative_paths() {
      let path = expand_path(Path::new("./local-data"));
      let cwd = std::env::current_dir().unwrap();
      assert_eq!(path, cwd.join("./local-data"));
    }
  }

  mod path_hash {
    use super::*;

    #[test]
    fn it_produces_a_16_char_hex_hash() {
      let hash = path_hash(Path::new("/some/test/path"));
      assert_eq!(hash.len(), 16);
      assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn it_returns_deterministic_results() {
      let path = Path::new("/some/test/path");
      let hash1 = path_hash(path);
      let hash2 = path_hash(path);
      assert_eq!(hash1, hash2);
    }
  }

  mod walk_up_for {
    use super::*;

    #[test]
    fn it_finds_directory_in_ancestor() {
      let tmp = tempfile::tempdir().unwrap();
      let target = tmp.path().join(".gest");
      std::fs::create_dir(&target).unwrap();

      let child = tmp.path().join("a").join("b").join("c");
      std::fs::create_dir_all(&child).unwrap();

      let result = walk_up_for(&child, ".gest");
      assert_eq!(result, Some(target));
    }

    #[test]
    fn it_finds_directory_in_current() {
      let tmp = tempfile::tempdir().unwrap();
      let target = tmp.path().join(".gest");
      std::fs::create_dir(&target).unwrap();

      let result = walk_up_for(tmp.path(), ".gest");
      assert_eq!(result, Some(target));
    }

    #[test]
    fn it_returns_none_when_not_found() {
      let tmp = tempfile::tempdir().unwrap();
      let result = walk_up_for(tmp.path(), ".nonexistent");
      assert_eq!(result, None);
    }
  }
}

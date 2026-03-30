use std::path::Path;

use clap::Args;

use crate::{
  cli::{self, AppContext},
  ui::{theme::Theme, views::system::InitView},
};

/// Directories created inside `.gest/` during initialization.
const SUBDIRS: &[&str] = &[
  "artifacts",
  "artifacts/archive",
  "iterations",
  "iterations/resolved",
  "tasks",
  "tasks/archive",
];

/// Initialize a `.gest` directory in the current project.
#[derive(Debug, Args)]
pub struct Command;

impl Command {
  /// Create the `.gest/` directory tree under the current working directory.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    let cwd = std::env::current_dir()?;
    let base = cwd.join(".gest");
    init_at(&base, &ctx.theme)
  }
}

/// Create any missing subdirectories under `base` and display the result.
fn init_at(base: &Path, theme: &Theme) -> cli::Result<()> {
  let mut created_subdirs = Vec::new();
  for subdir in SUBDIRS {
    let path = base.join(subdir);
    if !path.exists() {
      std::fs::create_dir_all(&path)?;
      created_subdirs.push((*subdir).to_string());
    }
  }

  let view = InitView::new(".gest/", ".gest/config.toml", theme);
  println!("{view}");

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  mod init_at {
    use super::*;

    #[test]
    fn it_creates_gest_directory_structure() {
      let tmp = tempfile::tempdir().unwrap();
      let base = tmp.path().join(".gest");

      init_at(&base, &Theme::default()).unwrap();

      assert!(base.join("tasks").is_dir());
      assert!(base.join("tasks/archive").is_dir());
      assert!(base.join("artifacts").is_dir());
      assert!(base.join("artifacts/archive").is_dir());
      assert!(base.join("iterations").is_dir());
      assert!(base.join("iterations/resolved").is_dir());
    }

    #[test]
    fn it_creates_missing_subdirs_when_partially_initialized() {
      let tmp = tempfile::tempdir().unwrap();
      let base = tmp.path().join(".gest");

      std::fs::create_dir_all(base.join("tasks")).unwrap();

      init_at(&base, &Theme::default()).unwrap();

      assert!(base.join("artifacts").is_dir());
      assert!(base.join("tasks/archive").is_dir());
      assert!(base.join("artifacts/archive").is_dir());
    }

    #[test]
    fn it_is_idempotent() {
      let tmp = tempfile::tempdir().unwrap();
      let base = tmp.path().join(".gest");

      init_at(&base, &Theme::default()).unwrap();
      init_at(&base, &Theme::default()).unwrap();

      assert!(base.join("tasks").is_dir());
      assert!(base.join("artifacts").is_dir());
    }
  }
}

use std::path::Path;

use clap::Args;

use crate::{
  cli::{self, AppContext},
  ui::{theme::Theme, views::system::InitView},
};

/// Directories created during initialization.
const SUBDIRS: &[&str] = &[
  "artifacts",
  "artifacts/archive",
  "iterations",
  "iterations/resolved",
  "tasks",
  "tasks/resolved",
];

/// Initialize gest for the current project.
#[derive(Debug, Args)]
pub struct Command {
  /// Initialize a `.gest` directory in the current project instead of the global data directory.
  #[arg(long)]
  local: bool,
}

impl Command {
  /// Initialize the store directory tree, either locally or globally.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    if self.local {
      let cwd = std::env::current_dir()?;
      let base = cwd.join(".gest");
      init_at(&base, Some(".gest/config.toml"), &ctx.theme)
    } else {
      init_at(&ctx.data_dir, None, &ctx.theme)
    }
  }
}

/// Create any missing subdirectories under `base` and display the result.
fn init_at(base: &Path, config_path: Option<&str>, theme: &Theme) -> cli::Result<()> {
  for subdir in SUBDIRS {
    let path = base.join(subdir);
    if !path.exists() {
      std::fs::create_dir_all(&path)?;
    }
  }

  let data_dir = base.display().to_string();
  let view = InitView::new(&data_dir, config_path, theme);
  println!("{view}");

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  mod init_at {
    use super::*;

    #[test]
    fn it_creates_directory_structure() {
      let tmp = tempfile::tempdir().unwrap();
      let base = tmp.path().join("data");

      init_at(&base, None, &Theme::default()).unwrap();

      assert!(base.join("tasks").is_dir());
      assert!(base.join("tasks/resolved").is_dir());
      assert!(base.join("artifacts").is_dir());
      assert!(base.join("artifacts/archive").is_dir());
      assert!(base.join("iterations").is_dir());
      assert!(base.join("iterations/resolved").is_dir());
    }

    #[test]
    fn it_creates_missing_subdirs_when_partially_initialized() {
      let tmp = tempfile::tempdir().unwrap();
      let base = tmp.path().join("data");

      std::fs::create_dir_all(base.join("tasks")).unwrap();

      init_at(&base, None, &Theme::default()).unwrap();

      assert!(base.join("artifacts").is_dir());
      assert!(base.join("tasks/resolved").is_dir());
      assert!(base.join("artifacts/archive").is_dir());
    }

    #[test]
    fn it_is_idempotent() {
      let tmp = tempfile::tempdir().unwrap();
      let base = tmp.path().join("data");

      init_at(&base, None, &Theme::default()).unwrap();
      init_at(&base, None, &Theme::default()).unwrap();

      assert!(base.join("tasks").is_dir());
      assert!(base.join("artifacts").is_dir());
    }

    #[test]
    fn it_accepts_optional_config_path() {
      let tmp = tempfile::tempdir().unwrap();
      let base = tmp.path().join(".gest");

      init_at(&base, Some(".gest/config.toml"), &Theme::default()).unwrap();

      assert!(base.join("tasks").is_dir());
      assert!(base.join("tasks/resolved").is_dir());
    }
  }
}

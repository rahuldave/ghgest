use std::path::{Path, PathBuf};

use assert_cmd::Command;
use tempfile::TempDir;

/// Test helper wrapping `assert_cmd::Command` with per-test filesystem isolation.
pub struct GestCmd {
  temp_dir: TempDir,
}

impl GestCmd {
  /// Create a new `GestCmd` with a fresh temp directory and run `gest init` in it.
  pub fn new() -> Self {
    let temp_dir = TempDir::new().expect("failed to create temp dir");

    // Initialize a gest project in the temp dir
    let mut init = Self::build_cmd(&temp_dir);
    init.args(["init"]);
    init.assert().success();

    Self {
      temp_dir,
    }
  }

  /// Return a `Command` pre-configured with isolation env vars.
  pub fn cmd(&self) -> Command {
    let mut cmd = Self::build_cmd(&self.temp_dir);
    cmd.arg("--no-color");
    cmd
  }

  /// Return a `Command` with isolation env vars but no extra args.
  pub fn raw_cmd(&self) -> Command {
    Self::build_cmd(&self.temp_dir)
  }

  /// Shorthand for `cmd().args(args).assert()`.
  pub fn run(&self, args: &[&str]) -> assert_cmd::assert::Assert {
    self.cmd().args(args).assert()
  }

  /// Read a file relative to the data directory.
  pub fn read_data_file(&self, relative: &str) -> String {
    let path = self.data_dir().join(relative);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()))
  }

  /// Return the temp directory path.
  pub fn temp_dir_path(&self) -> &Path {
    self.temp_dir.path()
  }

  fn data_dir(&self) -> PathBuf {
    self.temp_dir.path().join(".gest")
  }

  fn build_cmd(temp_dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("gest").expect("gest binary not found");
    let path = temp_dir.path();
    cmd.current_dir(path);
    cmd.env("GEST_CONFIG", path.join("gest.toml"));
    cmd.env("GEST_DATA_DIR", path.join(".gest"));
    cmd.env("GEST_STATE_DIR", path.join(".gest-state"));
    cmd
  }
}

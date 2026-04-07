use std::path::Path;

use assert_cmd::Command;
use tempfile::TempDir;

/// Test helper wrapping `assert_cmd::Command` with per-test filesystem isolation.
pub struct GestCmd {
  temp_dir: TempDir,
}

impl GestCmd {
  fn build_cmd(temp_dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("gest").expect("gest binary not found");
    let path = temp_dir.path();
    cmd.current_dir(path);
    cmd.env("GEST_CONFIG", path.join("gest.toml"));
    cmd.env("GEST_STORAGE__DATA_DIR", path.join(".gest-data"));
    cmd.env("GEST_PROJECT_DIR", path.join(".gest"));
    cmd.env("GEST_STATE_DIR", path.join(".gest-state"));
    cmd
  }

  /// Create a new `GestCmd` with a fresh temp directory and run `gest init` in it.
  pub fn new() -> Self {
    let temp_dir = TempDir::new().expect("failed to create temp dir");

    // Pre-create the data and state directories so config resolution succeeds
    // before `init` populates the full directory tree.
    std::fs::create_dir_all(temp_dir.path().join(".gest")).expect("failed to create .gest dir");
    std::fs::create_dir_all(temp_dir.path().join(".gest-data")).expect("failed to create .gest-data dir");
    std::fs::create_dir_all(temp_dir.path().join(".gest-state")).expect("failed to create .gest-state dir");

    // Initialize a gest project in the temp dir
    let mut init = Self::build_cmd(&temp_dir);
    init.args(["init"]);
    init.assert().success();

    Self {
      temp_dir,
    }
  }

  /// Create a new `GestCmd` with a fresh temp directory without running `gest init`.
  ///
  /// The data and state directories are created so that config loading succeeds,
  /// but the store structure is not initialized. Useful for commands that do not
  /// need an initialized project (e.g. `version`).
  pub fn new_uninit() -> Self {
    let temp_dir = TempDir::new().expect("failed to create temp dir");

    std::fs::create_dir_all(temp_dir.path().join(".gest")).expect("failed to create .gest dir");
    std::fs::create_dir_all(temp_dir.path().join(".gest-data")).expect("failed to create .gest-data dir");
    std::fs::create_dir_all(temp_dir.path().join(".gest-state")).expect("failed to create .gest-state dir");

    Self {
      temp_dir,
    }
  }

  /// Return a `Command` pre-configured with isolation env vars and color disabled.
  pub fn cmd(&self) -> Command {
    let mut cmd = Self::build_cmd(&self.temp_dir);
    cmd.env("NO_COLOR", "1");
    cmd
  }

  /// Create an artifact with the given title and body, returning the new artifact ID.
  pub fn create_artifact(&self, title: &str, body: &str) -> String {
    let output = self
      .cmd()
      .args(["artifact", "create", title, "--body", body])
      .output()
      .expect("failed to run artifact create");

    assert!(
      output.status.success(),
      "artifact create failed: {}",
      String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    extract_id_from_create_output(&stdout)
      .unwrap_or_else(|| panic!("could not extract artifact ID from output:\n{stdout}"))
  }

  /// Create an iteration with the given title, returning the new iteration ID.
  pub fn create_iteration(&self, title: &str) -> String {
    let output = self
      .cmd()
      .args(["iteration", "create", title])
      .output()
      .expect("failed to run iteration create");

    assert!(
      output.status.success(),
      "iteration create failed: {}",
      String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    extract_id_from_create_output(&stdout)
      .unwrap_or_else(|| panic!("could not extract iteration ID from output:\n{stdout}"))
  }

  /// Create a task with the given title, returning the new task ID.
  pub fn create_task(&self, title: &str) -> String {
    let output = self
      .cmd()
      .args(["task", "create", title])
      .output()
      .expect("failed to run task create");

    assert!(
      output.status.success(),
      "task create failed: {}",
      String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    extract_id_from_create_output(&stdout).unwrap_or_else(|| panic!("could not extract task ID from output:\n{stdout}"))
  }

  /// Return a `Command` with isolation env vars but no extra args.
  pub fn raw_cmd(&self) -> Command {
    Self::build_cmd(&self.temp_dir)
  }

  /// Shorthand for `cmd().args(args).assert()`.
  pub fn run(&self, args: &[&str]) -> assert_cmd::assert::Assert {
    self.cmd().args(args).assert()
  }

  /// Return the temp directory path.
  pub fn temp_dir_path(&self) -> &Path {
    self.temp_dir.path()
  }
}

/// Extract the entity ID from a "created <entity>  <id>" output line.
pub fn extract_id_from_create_output(output: &str) -> Option<String> {
  output
    .lines()
    .find(|line| line.to_lowercase().contains("created"))
    .and_then(|line| line.split_whitespace().last().map(str::to_string))
}

/// Extract the rendered prefix length for `short_id` from a colored output buffer.
///
/// IDs are displayed as `<CSI>...m{prefix}<CSI>0m<CSI>...m{rest}<CSI>0m`. We scan for the id
/// as a contiguous run of visible characters, allowing escape sequences to interleave; the
/// first interleaved escape after at least one visible character marks the prefix→rest
/// boundary.
pub fn rendered_prefix_len(output: &str, short_id: &str) -> Option<usize> {
  let bytes = output.as_bytes();
  let target = short_id.as_bytes();
  let mut i = 0;
  while i < bytes.len() {
    let mut j = i;
    let mut t = 0;
    let mut prefix_len: Option<usize> = None;
    let mut visible_seen = 0usize;
    let mut last_was_visible = true;
    while t < target.len() && j < bytes.len() {
      if bytes[j] == 0x1b && j + 1 < bytes.len() && bytes[j + 1] == b'[' {
        if t > 0 && prefix_len.is_none() && last_was_visible {
          prefix_len = Some(visible_seen);
        }
        j += 2;
        while j < bytes.len() && !(0x40..=0x7e).contains(&bytes[j]) {
          j += 1;
        }
        if j < bytes.len() {
          j += 1;
        }
        last_was_visible = false;
        continue;
      }
      if bytes[j] == target[t] {
        t += 1;
        j += 1;
        visible_seen += 1;
        last_was_visible = true;
      } else {
        break;
      }
    }
    if t == target.len() {
      return Some(prefix_len.unwrap_or(visible_seen));
    }
    i += 1;
  }
  None
}

/// Strip ANSI escape sequences from a string.
pub fn strip_ansi(s: &str) -> String {
  let mut out = String::with_capacity(s.len());
  let bytes = s.as_bytes();
  let mut i = 0;
  while i < bytes.len() {
    if bytes[i] == 0x1b && i + 1 < bytes.len() && bytes[i + 1] == b'[' {
      i += 2;
      while i < bytes.len() && !(0x40..=0x7e).contains(&bytes[i]) {
        i += 1;
      }
      if i < bytes.len() {
        i += 1;
      }
    } else {
      out.push(bytes[i] as char);
      i += 1;
    }
  }
  out
}

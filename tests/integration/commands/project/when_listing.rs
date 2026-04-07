use assert_cmd::Command;

use crate::support::helpers::GestCmd;

/// Build a second `gest init` command against the same data store as `g` but
/// running from `dir`. Used to seed a second project entry in the shared store.
fn init_extra_project(g: &GestCmd, dir: &std::path::Path) {
  std::fs::create_dir_all(dir).expect("failed to create extra project dir");
  let data_dir = g.temp_dir_path().join(".gest-data");
  let state_dir = g.temp_dir_path().join(".gest-state");
  let config = g.temp_dir_path().join("gest.toml");
  let project_dir = dir.join(".gest");
  std::fs::create_dir_all(&project_dir).expect("failed to create extra .gest dir");

  Command::cargo_bin("gest")
    .expect("gest binary not found")
    .current_dir(dir)
    .env("GEST_CONFIG", config)
    .env("GEST_STORAGE__DATA_DIR", data_dir)
    .env("GEST_PROJECT_DIR", project_dir)
    .env("GEST_STATE_DIR", state_dir)
    .env("NO_COLOR", "1")
    .args(["init"])
    .assert()
    .success();
}

#[test]
fn it_lists_projects_in_grid_format() {
  let g = GestCmd::new();

  let output = g
    .cmd()
    .args(["project", "list"])
    .output()
    .expect("project list failed to run");

  assert!(
    output.status.success(),
    "project list exited non-zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("projects"), "missing projects header in: {stdout}");
  assert!(stdout.contains("1 project"), "missing count summary in: {stdout}");
  let root = g.temp_dir_path().display().to_string();
  assert!(stdout.contains(&root), "missing project root in: {stdout}");
}

#[test]
fn it_lists_projects_as_json() {
  let g = GestCmd::new();

  let output = g
    .cmd()
    .args(["project", "list", "--json"])
    .output()
    .expect("project list --json failed to run");

  assert!(output.status.success(), "project list --json exited non-zero");
  let stdout = String::from_utf8_lossy(&output.stdout);
  let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("output should be valid JSON");
  let arr = parsed.as_array().expect("output should be a JSON array");
  assert_eq!(arr.len(), 1, "expected one project, got: {stdout}");
  assert!(arr[0]["id"].is_string(), "project should have id: {stdout}");
  assert!(arr[0]["root"].is_string(), "project should have root: {stdout}");
}

#[test]
fn it_lists_multiple_projects_with_plural_summary() {
  let g = GestCmd::new();
  let extra_dir = g.temp_dir_path().join("other-project");
  init_extra_project(&g, &extra_dir);

  let output = g
    .cmd()
    .args(["project", "list"])
    .output()
    .expect("project list failed to run");

  assert!(
    output.status.success(),
    "project list exited non-zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("2 projects"), "expected plural summary in: {stdout}");
}

#[test]
fn it_lists_multiple_projects_shows_all_roots() {
  let g = GestCmd::new();
  let extra_dir = g.temp_dir_path().join("second-project");
  init_extra_project(&g, &extra_dir);

  let output = g
    .cmd()
    .args(["project", "list"])
    .output()
    .expect("project list failed to run");

  assert!(
    output.status.success(),
    "project list exited non-zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );
  let stdout = String::from_utf8_lossy(&output.stdout);
  let first_root = g.temp_dir_path().display().to_string();
  let second_root = extra_dir.display().to_string();
  assert!(stdout.contains(&first_root), "missing first project root in: {stdout}");
  assert!(
    stdout.contains(&second_root),
    "missing second project root in: {stdout}"
  );
}

#[test]
fn it_lists_multiple_projects_as_json_with_correct_count() {
  let g = GestCmd::new();
  let extra_dir = g.temp_dir_path().join("third-project");
  init_extra_project(&g, &extra_dir);

  let output = g
    .cmd()
    .args(["project", "list", "--json"])
    .output()
    .expect("project list --json failed to run");

  assert!(output.status.success(), "project list --json exited non-zero");
  let stdout = String::from_utf8_lossy(&output.stdout);
  let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("output should be valid JSON");
  let arr = parsed.as_array().expect("output should be a JSON array");
  assert_eq!(arr.len(), 2, "expected two projects in JSON array, got: {stdout}");
  for entry in arr {
    assert!(entry["id"].is_string(), "each project should have id field");
    assert!(entry["root"].is_string(), "each project should have root field");
  }
}

#[test]
fn it_list_human_output_is_not_valid_json() {
  let g = GestCmd::new();

  let output = g
    .cmd()
    .args(["project", "list"])
    .output()
    .expect("project list failed to run");

  assert!(output.status.success(), "project list exited non-zero");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(
    serde_json::from_str::<serde_json::Value>(&stdout).is_err(),
    "human-readable list output should not parse as JSON, got: {stdout}"
  );
}

use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_groups_tasks_by_status_in_expanded_output() {
  let env = GestCmd::new();

  env
    .run([
      "task",
      "create",
      "Bison habitat assessment",
      "-d",
      "assess bison habitat suitability",
    ])
    .assert()
    .success();

  let in_progress_output = env
    .run([
      "task",
      "create",
      "Bison relocation plan",
      "-d",
      "plan bison relocation route",
    ])
    .output()
    .expect("failed to create in-progress task");

  let in_progress_stdout = String::from_utf8_lossy(&in_progress_output.stdout);
  let in_progress_id = in_progress_stdout
    .split_whitespace()
    .last()
    .expect("expected task id in output");

  env
    .run(["task", "update", in_progress_id, "--status", "in-progress"])
    .assert()
    .success();

  env
    .run(["search", "bison", "--expand"])
    .assert()
    .success()
    .stdout(predicate::str::contains("open"))
    .stdout(predicate::str::contains("in-progress"));
}

#[test]
fn it_includes_resolved_and_archived_items_with_all_flag() {
  let env = GestCmd::new();

  // Create and resolve a task
  let task_output = env
    .run(["task", "create", "Penguin census complete", "-d", "final penguin count"])
    .output()
    .expect("failed to create task");

  let task_stdout = String::from_utf8_lossy(&task_output.stdout);
  let task_id = task_stdout
    .split_whitespace()
    .last()
    .expect("expected task id in output");

  env
    .run(["task", "update", task_id, "--status", "done"])
    .assert()
    .success();

  // Create and archive an artifact
  let artifact_output = env
    .run([
      "artifact",
      "create",
      "-t",
      "Penguin habitat map",
      "-b",
      "archived map of penguin habitats",
    ])
    .output()
    .expect("failed to create artifact");

  let artifact_stdout = String::from_utf8_lossy(&artifact_output.stdout);
  let artifact_id = artifact_stdout
    .split_whitespace()
    .last()
    .expect("expected artifact id in output");

  env.run(["artifact", "archive", artifact_id]).assert().success();

  // Search with --json --expand --all should include resolved/archived items
  let output = env
    .run(["search", "--json", "--expand", "--all", "penguin"])
    .output()
    .expect("failed to run search --json --expand --all");

  assert!(output.status.success(), "expected success exit code");

  let stdout = String::from_utf8_lossy(&output.stdout);
  let json: serde_json::Value = serde_json::from_str(&stdout).expect("expected valid JSON output");

  let tasks = json["tasks"].as_array().expect("expected tasks array");
  assert!(
    tasks.iter().any(|t| t["title"] == "Penguin census complete"),
    "expected resolved task to appear in results"
  );
  let resolved_task = tasks
    .iter()
    .find(|t| t["title"] == "Penguin census complete")
    .expect("expected resolved task");
  assert_eq!(resolved_task["status"], "done");
  assert_eq!(resolved_task["description"], "final penguin count");
  assert!(resolved_task["created_at"].is_string(), "expected created_at");
  assert!(resolved_task["updated_at"].is_string(), "expected updated_at");

  let artifacts = json["artifacts"].as_array().expect("expected artifacts array");
  assert!(
    artifacts.iter().any(|a| a["title"] == "Penguin habitat map"),
    "expected archived artifact to appear in results"
  );
  let archived_artifact = artifacts
    .iter()
    .find(|a| a["title"] == "Penguin habitat map")
    .expect("expected archived artifact");
  assert_eq!(archived_artifact["body"], "archived map of penguin habitats");
  assert!(archived_artifact["created_at"].is_string(), "expected created_at");
  assert!(archived_artifact["updated_at"].is_string(), "expected updated_at");
}

#[test]
fn it_renders_artifact_body_in_expanded_output() {
  let env = GestCmd::new();

  env
    .run([
      "artifact",
      "create",
      "-t",
      "Walrus feeding schedule",
      "-b",
      "feed walruses three times daily",
    ])
    .assert()
    .success();

  env
    .run(["search", "walrus", "--expand"])
    .assert()
    .success()
    .stdout(predicate::str::contains("Walrus feeding schedule"))
    .stdout(predicate::str::contains("feed walruses three times daily"));
}

#[test]
fn it_renders_expanded_output_without_json() {
  let env = GestCmd::new();

  env
    .run([
      "task",
      "create",
      "Otter habitat survey",
      "-d",
      "survey otter habitats along the coast",
    ])
    .assert()
    .success();

  env
    .run(["search", "otter", "--expand"])
    .assert()
    .success()
    .stdout(predicate::str::contains("Otter habitat survey"))
    .stdout(predicate::str::contains("─"));
}

#[test]
fn it_renders_horizontal_rules_in_expanded_output() {
  let env = GestCmd::new();

  env
    .run([
      "task",
      "create",
      "Capybara census project",
      "-d",
      "count all capybaras in the region",
    ])
    .assert()
    .success();

  env
    .run(["search", "capybara", "--expand"])
    .assert()
    .success()
    .stdout(predicate::str::contains("─"));
}

#[test]
fn it_returns_full_artifact_detail_as_json() {
  let env = GestCmd::new();

  env
    .run([
      "artifact",
      "create",
      "-t",
      "Narwhal migration report",
      "-b",
      "detailed findings on narwhal migration",
    ])
    .assert()
    .success();

  let output = env
    .run(["search", "--json", "--expand", "narwhal"])
    .output()
    .expect("failed to run search --json --expand");

  assert!(output.status.success(), "expected success exit code");

  let stdout = String::from_utf8_lossy(&output.stdout);
  let json: serde_json::Value = serde_json::from_str(&stdout).expect("expected valid JSON output");

  let artifacts = json["artifacts"].as_array().expect("expected artifacts array");
  assert_eq!(artifacts.len(), 1, "expected exactly one artifact match");

  let artifact = &artifacts[0];
  assert_eq!(artifact["title"], "Narwhal migration report");
  assert_eq!(artifact["body"], "detailed findings on narwhal migration");
  assert!(artifact["id"].is_string(), "expected id to be a string");
  assert!(!artifact["type"].is_object(), "expected type to be a scalar value");
  assert!(artifact["created_at"].is_string(), "expected created_at to be a string");
  assert!(artifact["updated_at"].is_string(), "expected updated_at to be a string");
  assert!(artifact["tags"].is_array(), "expected tags to be an array");
}

#[test]
fn it_returns_full_task_detail_as_json() {
  let env = GestCmd::new();

  env
    .run([
      "task",
      "create",
      "Kangaroo relocation strategy",
      "-d",
      "relocate kangaroos to new habitat",
    ])
    .assert()
    .success();

  let output = env
    .run(["search", "--json", "--expand", "kangaroo"])
    .output()
    .expect("failed to run search --json --expand");

  assert!(output.status.success(), "expected success exit code");

  let stdout = String::from_utf8_lossy(&output.stdout);
  let json: serde_json::Value = serde_json::from_str(&stdout).expect("expected valid JSON output");

  let tasks = json["tasks"].as_array().expect("expected tasks array");
  assert_eq!(tasks.len(), 1, "expected exactly one task match");

  let task = &tasks[0];
  assert_eq!(task["title"], "Kangaroo relocation strategy");
  assert_eq!(task["description"], "relocate kangaroos to new habitat");
  assert_eq!(task["status"], "open");
  assert!(task["id"].is_string(), "expected id to be a string");
  assert!(task["created_at"].is_string(), "expected created_at to be a string");
  assert!(task["updated_at"].is_string(), "expected updated_at to be a string");
  assert!(task["tags"].is_array(), "expected tags to be an array");
}

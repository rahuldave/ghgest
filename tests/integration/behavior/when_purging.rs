//! Integration tests for the `gest purge` top-level command.

use std::process;

use crate::support::helpers::{GestCmd, strip_ansi};

/// Count rows in a given table via sqlite3.
fn count_rows(g: &GestCmd, table: &str) -> usize {
  let db = g.db_path();
  let sql = format!("SELECT COUNT(*) FROM {table};");
  let output = process::Command::new("sqlite3")
    .arg(db)
    .arg(sql)
    .output()
    .expect("sqlite3 should be available");
  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  stdout.trim().parse().unwrap_or(0)
}

#[test]
fn it_purges_terminal_tasks_with_yes() {
  let g = GestCmd::new();
  let id = g.create_task("Done task");
  g.complete_task(&id);

  let before = count_rows(&g, "tasks");

  let output = g
    .cmd()
    .args(["purge", "--tasks", "--yes"])
    .output()
    .expect("purge failed to run");

  assert!(
    output.status.success(),
    "purge exited non-zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );

  let after = count_rows(&g, "tasks");

  assert!(after < before, "expected task count to decrease after purge");
}

#[test]
fn it_purges_terminal_iterations_with_yes() {
  let g = GestCmd::new();
  let id = g.create_iteration("Sprint 1");
  g.complete_iteration(&id);

  let before = count_rows(&g, "iterations");

  let output = g
    .cmd()
    .args(["purge", "--iterations", "--yes"])
    .output()
    .expect("purge failed to run");

  assert!(
    output.status.success(),
    "purge exited non-zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );

  let after = count_rows(&g, "iterations");

  assert!(after < before, "expected iteration count to decrease after purge");
}

#[test]
fn it_purges_archived_artifacts_with_yes() {
  let g = GestCmd::new();
  let short_id = g.create_artifact("Old spec", "body text");

  // Archive the artifact via gest CLI
  let archive_output = g
    .cmd()
    .args(["artifact", "archive", &short_id])
    .output()
    .expect("artifact archive failed to run");
  assert!(
    archive_output.status.success(),
    "artifact archive failed: {}",
    String::from_utf8_lossy(&archive_output.stderr)
  );

  let before = count_rows(&g, "artifacts");

  let output = g
    .cmd()
    .args(["purge", "--artifacts", "--yes"])
    .output()
    .expect("purge failed to run");

  assert!(
    output.status.success(),
    "purge exited non-zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );

  let after = count_rows(&g, "artifacts");

  assert!(after < before, "expected artifact count to decrease after purge");
}

#[test]
fn it_shows_nothing_to_purge_when_store_is_clean() {
  let g = GestCmd::new();

  let output = g.cmd().args(["purge", "--yes"]).output().expect("purge failed to run");

  assert!(output.status.success());
  let stdout = strip_ansi(&String::from_utf8_lossy(&output.stdout));

  assert!(
    stdout.contains("Nothing to purge"),
    "expected 'Nothing to purge' but got: {stdout}"
  );
}

#[test]
fn it_dry_run_has_zero_side_effects() {
  let g = GestCmd::new();
  let id = g.create_task("Done task for dry-run");
  g.complete_task(&id);

  let before = count_rows(&g, "tasks");

  let output = g
    .cmd()
    .args(["purge", "--tasks", "--dry-run"])
    .output()
    .expect("purge failed to run");

  assert!(output.status.success());

  let after = count_rows(&g, "tasks");

  assert_eq!(before, after, "dry-run should not modify the database");

  let stdout = strip_ansi(&String::from_utf8_lossy(&output.stdout));

  assert!(stdout.contains("tasks:"), "dry-run output should show task counts");
}

#[test]
fn it_defaults_to_all_selectors_when_no_flags() {
  let g = GestCmd::new();
  let id = g.create_task("Cancelled task");
  g.cancel_task(&id);

  let output = g
    .cmd()
    .args(["purge", "--dry-run"])
    .output()
    .expect("purge failed to run");

  assert!(output.status.success());
  let stdout = strip_ansi(&String::from_utf8_lossy(&output.stdout));

  assert!(
    stdout.contains("tasks:"),
    "default purge should include tasks: {stdout}"
  );
}

#[test]
fn it_purges_combined_selectors() {
  let g = GestCmd::new();

  let task_id = g.create_task("Done combined");
  g.complete_task(&task_id);

  let iter_id = g.create_iteration("Sprint combined");
  g.complete_iteration(&iter_id);

  let tasks_before = count_rows(&g, "tasks");
  let iters_before = count_rows(&g, "iterations");

  let output = g
    .cmd()
    .args(["purge", "--tasks", "--iterations", "--yes"])
    .output()
    .expect("purge failed to run");

  assert!(output.status.success());

  let tasks_after = count_rows(&g, "tasks");
  let iters_after = count_rows(&g, "iterations");

  assert!(tasks_after < tasks_before, "expected tasks to decrease");
  assert!(iters_after < iters_before, "expected iterations to decrease");
}

#[test]
fn it_undo_restores_purged_tasks() {
  let g = GestCmd::new();
  let id = g.create_task("Undo me");
  g.complete_task(&id);

  let before = count_rows(&g, "tasks");

  g.cmd()
    .args(["purge", "--tasks", "--yes"])
    .output()
    .expect("purge failed to run");

  let after_purge = count_rows(&g, "tasks");

  assert!(after_purge < before, "purge should have removed the task");

  let undo_output = g.cmd().args(["undo"]).output().expect("undo failed to run");

  assert!(
    undo_output.status.success(),
    "undo exited non-zero: {}",
    String::from_utf8_lossy(&undo_output.stderr)
  );

  let after_undo = count_rows(&g, "tasks");

  assert_eq!(after_undo, before, "undo should restore the task");
}

#[test]
fn it_shows_purge_summary_in_dry_run() {
  let g = GestCmd::new();

  let t1 = g.create_task("Done 1");
  g.complete_task(&t1);
  let t2 = g.create_task("Cancelled 1");
  g.cancel_task(&t2);

  let output = g
    .cmd()
    .args(["purge", "--tasks", "--dry-run"])
    .output()
    .expect("purge failed to run");

  assert!(output.status.success());
  let stdout = strip_ansi(&String::from_utf8_lossy(&output.stdout));

  assert!(stdout.contains("Purge summary"), "should show purge summary: {stdout}");
  assert!(stdout.contains("tasks: 2"), "should show task count of 2: {stdout}");
  assert!(stdout.contains("1 done"), "should show done count: {stdout}");
  assert!(stdout.contains("1 cancelled"), "should show cancelled count: {stdout}");
}

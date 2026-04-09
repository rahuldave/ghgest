//! Integration tests covering the `iteration graph` command output.

use crate::support::helpers::GestCmd;

/// Run `iteration graph` with color disabled so stdout contains no ANSI
/// escape sequences (the existing `strip_ansi` helper operates on bytes and
/// corrupts multi-byte UTF-8 box-drawing characters).
fn graph(g: &GestCmd, iter_id: &str) -> String {
  let mut cmd = g.raw_cmd();
  cmd.env("NO_COLOR", "1");
  cmd.args(["iteration", "graph", iter_id]);
  let output = cmd.output().expect("iteration graph failed to run");
  assert!(
    output.status.success(),
    "iteration graph exited non-zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );
  String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn it_renders_phase_header_and_summary_line() {
  let g = GestCmd::new();
  let iter_id = g.create_iteration_with_phases("Sprint", &[&["a task"]]);

  let out = graph(&g, &iter_id);

  assert!(out.contains("Sprint"), "output should contain iteration title: {out}");
  assert!(
    out.contains("1 phase \u{00B7} 1 task"),
    "output should contain summary line: {out}"
  );
  assert!(
    out.contains("\u{25C6}  Phase 1"),
    "output should contain phase header: {out}"
  );
}

#[test]
fn it_renders_branch_connectors_for_multi_task_phases() {
  let g = GestCmd::new();
  let iter_id = g.create_iteration_with_phases("Sprint", &[&["one", "two", "three"]]);

  let out = graph(&g, &iter_id);

  assert!(
    out.contains("\u{251C}\u{2500}\u{256E}\u{2500}\u{256E}"),
    "output should contain three-wide branch open: {out}"
  );
  assert!(
    out.contains("\u{2570}\u{2500}\u{256F}\u{2500}\u{256F}"),
    "output should contain three-wide rounded branch close on the last phase: {out}"
  );
}

#[test]
fn it_renders_continuation_line_between_phases() {
  let g = GestCmd::new();
  let iter_id = g.create_iteration_with_phases("Sprint", &[&["one"], &["two"]]);

  let out = graph(&g, &iter_id);

  let continuation_lines = out.lines().filter(|l| l.trim() == "\u{2502}").count();
  assert!(
    continuation_lines >= 1,
    "expected at least one continuation line between phases, got: {out}"
  );
}

#[test]
fn it_renders_priority_badge_when_task_has_priority() {
  let g = GestCmd::new();
  let iter_id = g.create_iteration("Sprint");
  let task_id = g.create_task("prioritized");
  g.cmd()
    .args(["task", "update", &task_id, "--priority", "1"])
    .assert()
    .success();
  g.cmd()
    .args(["iteration", "add", &iter_id, &task_id])
    .assert()
    .success();

  let out = graph(&g, &iter_id);

  assert!(out.contains("[P1]"), "output should contain priority badge: {out}");
}

#[test]
fn it_renders_blocked_and_blocking_indicators() {
  let g = GestCmd::new();
  let iter_id = g.create_iteration("Sprint");
  let blocker = g.create_task("blocker task");
  let blocked = g.create_task("blocked task");
  g.cmd().args(["task", "block", &blocker, &blocked]).assert().success();
  g.cmd()
    .args(["iteration", "add", &iter_id, &blocker])
    .assert()
    .success();
  g.cmd()
    .args(["iteration", "add", &iter_id, &blocked])
    .assert()
    .success();

  let out = graph(&g, &iter_id);

  assert!(
    out.contains("\u{2297} blocked"),
    "output should contain blocked status badge: {out}"
  );
  assert!(
    out.contains("! blocking"),
    "output should contain blocking indicator: {out}"
  );
  assert!(
    out.contains("blocked-by"),
    "output should contain blocked-by reference: {out}"
  );
}

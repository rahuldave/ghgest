use crate::support::helpers::GestCmd;

#[test]
fn it_appends_to_next_phase_when_phase_omitted() {
  let g = GestCmd::new();
  let iter_id = g.create_iteration("phase defaulting");
  let task_a = g.create_task("first task");
  let task_b = g.create_task("second task");

  let output_a = g
    .cmd()
    .args(["iteration", "add", &iter_id, &task_a, "--json"])
    .output()
    .expect("iteration add (a) failed to run");
  assert!(output_a.status.success(), "first iteration add exited non-zero");
  let stdout_a = String::from_utf8_lossy(&output_a.stdout);
  assert!(
    stdout_a.contains("\"phase\": 1") || stdout_a.contains("\"phase\":1"),
    "first task should land on phase 1, got: {stdout_a}"
  );

  let output_b = g
    .cmd()
    .args(["iteration", "add", &iter_id, &task_b, "--json"])
    .output()
    .expect("iteration add (b) failed to run");
  assert!(output_b.status.success(), "second iteration add exited non-zero");
  let stdout_b = String::from_utf8_lossy(&output_b.stdout);
  assert!(
    stdout_b.contains("\"phase\": 2") || stdout_b.contains("\"phase\":2"),
    "second task should land on phase 2, got: {stdout_b}"
  );
}

#[test]
fn it_uses_explicit_phase_when_provided() {
  let g = GestCmd::new();
  let iter_id = g.create_iteration("explicit phase");
  let task_id = g.create_task("only task");

  let output = g
    .cmd()
    .args(["iteration", "add", &iter_id, &task_id, "--phase", "5", "--json"])
    .output()
    .expect("iteration add failed to run");

  assert!(output.status.success(), "iteration add exited non-zero");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(
    stdout.contains("\"phase\": 5") || stdout.contains("\"phase\":5"),
    "task should land on explicit phase 5, got: {stdout}"
  );
}

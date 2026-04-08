use crate::support::helpers::GestCmd;

#[test]
fn it_deletes_an_iteration_preserving_member_tasks_and_writing_a_tombstone() {
  let g = GestCmd::new();
  let task_a = g.create_task("task a");
  let task_b = g.create_task("task b");
  let iteration_id = g.create_iteration_with_phases("Sprint", &[&[&task_a], &[&task_b]]);

  let output = g
    .cmd()
    .args(["iteration", "delete", &iteration_id, "--yes"])
    .output()
    .expect("iteration delete failed to run");
  assert!(
    output.status.success(),
    "iteration delete exited non-zero: {}",
    String::from_utf8_lossy(&output.stderr)
  );
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("deleted iteration"), "got: {stdout}");

  // Iteration row is gone.
  let show = g
    .cmd()
    .args(["iteration", "show", &iteration_id])
    .output()
    .expect("iteration show failed to run");
  assert!(!show.status.success(), "expected iteration show to fail after delete");

  // Member tasks must still exist.
  for task_id in [&task_a, &task_b] {
    let show_task = g
      .cmd()
      .args(["task", "show", task_id])
      .output()
      .expect("task show failed to run");
    assert!(
      show_task.status.success(),
      "task {task_id} should still exist after iteration delete: {}",
      String::from_utf8_lossy(&show_task.stderr)
    );
  }

  // Tombstone file persists with deleted_at.
  let iteration_dir = g.temp_dir_path().join(".gest").join("iteration");
  let tombstone = std::fs::read_dir(&iteration_dir)
    .expect("read iteration dir")
    .flatten()
    .map(|e| e.path())
    .find(|p| {
      p.extension().is_some_and(|ext| ext == "yaml")
        && p
          .file_stem()
          .and_then(|s| s.to_str())
          .is_some_and(|s| s.starts_with(&iteration_id))
    })
    .expect("tombstone iteration yaml should still exist after delete");
  let raw = std::fs::read_to_string(&tombstone).expect("read tombstone");
  assert!(raw.contains("deleted_at:"), "missing deleted_at:\n{raw}");
}

#[test]
fn it_is_undoable_and_restores_iteration_and_task_memberships() {
  let g = GestCmd::new();
  let task_id = g.create_task("member");
  let iteration_id = g.create_iteration_with_phases("Sprint", &[&[&task_id]]);

  let delete = g
    .cmd()
    .args(["iteration", "delete", &iteration_id, "--yes"])
    .output()
    .expect("iteration delete failed to run");
  assert!(delete.status.success());

  let undo = g.cmd().args(["undo"]).output().expect("undo failed to run");
  assert!(
    undo.status.success(),
    "undo exited non-zero: {}",
    String::from_utf8_lossy(&undo.stderr)
  );

  let show = g
    .cmd()
    .args(["iteration", "show", &iteration_id])
    .output()
    .expect("iteration show failed to run");
  assert!(
    show.status.success(),
    "expected iteration show to succeed after undo: {}",
    String::from_utf8_lossy(&show.stderr)
  );
  let stdout = String::from_utf8_lossy(&show.stdout);
  assert!(stdout.contains("Sprint"), "got: {stdout}");

  let status = g
    .cmd()
    .args(["iteration", "status", &iteration_id, "--json"])
    .output()
    .expect("iteration status failed to run");
  assert!(status.status.success());
  let status_stdout = String::from_utf8_lossy(&status.stdout);
  assert!(
    status_stdout.contains("\"total_tasks\": 1"),
    "expected 1 task membership after undo: {status_stdout}"
  );
  let _ = task_id;
}

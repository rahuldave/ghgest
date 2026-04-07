use crate::support::helpers::GestCmd;

#[test]
fn it_updates_task_status_to_done() {
  let g = GestCmd::new();
  let id = g.create_task("Finishable task");

  g.cmd()
    .args(["task", "update", &id, "--status", "done"])
    .assert()
    .success();

  let output = g.cmd().args(["task", "show", &id]).output().expect("task show failed");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("done"), "task should be done, got: {stdout}");
}

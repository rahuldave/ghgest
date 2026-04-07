use crate::support::helpers::GestCmd;

#[test]
fn it_shows_a_task_by_id() {
  let g = GestCmd::new();
  let id = g.create_task("Detail task");

  let output = g
    .cmd()
    .args(["task", "show", &id])
    .output()
    .expect("task show failed to run");

  assert!(output.status.success(), "task show exited non-zero");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("Detail task"), "got: {stdout}");
}

use crate::support::helpers::GestCmd;

#[test]
fn it_lists_open_tasks() {
  let g = GestCmd::new();
  g.create_task("Listed task");

  let output = g
    .cmd()
    .args(["task", "list"])
    .output()
    .expect("task list failed to run");

  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("Listed task"), "got: {stdout}");
}

use crate::support::helpers::GestCmd;

#[test]
fn it_gets_next_task() {
  let env = GestCmd::new();
  let iter_id = env.create_iteration("Sprint 1");
  let task_id = env.create_task("First task");

  env
    .cmd()
    .args(["task", "update", &task_id, "--phase", "1", "--priority", "1"])
    .assert()
    .success();

  env
    .cmd()
    .args(["iteration", "add", &iter_id, &task_id])
    .assert()
    .success();

  env.cmd().args(["iteration", "next", &iter_id]).assert().success();
}

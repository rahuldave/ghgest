use crate::support::helpers::GestCmd;

#[test]
fn it_shows_iteration_status() {
  let env = GestCmd::new();
  let iter_id = env.create_iteration("Sprint 1");

  env.cmd().args(["iteration", "status", &iter_id]).assert().success();
}

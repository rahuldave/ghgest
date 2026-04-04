use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_lists_resolved_tasks() {
  let env = GestCmd::new();
  let id = env.create_task("Done task");

  env.run(&["task", "update", &id, "--status", "done"]).success();

  // Without --all, resolved tasks should not appear
  env
    .run(&["task", "list"])
    .success()
    .stdout(predicate::str::contains("Done task").not());

  // With --all, resolved tasks should appear
  env
    .run(&["task", "list", "--all"])
    .success()
    .stdout(predicate::str::contains("Done task"));
}

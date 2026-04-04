use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_cancels_the_task() {
  let env = GestCmd::new();
  let id = env.create_task("Task to cancel");

  env
    .run(&["task", "cancel", &id])
    .success()
    .stdout(predicate::str::contains("cancelled"));
}

#[test]
fn it_produces_same_status_as_update_with_cancelled() {
  let env = GestCmd::new();
  let id = env.create_task("Compare cancel");

  env.run(&["task", "cancel", &id]).success();

  env
    .run(&["task", "show", &id])
    .success()
    .stdout(predicate::str::contains("cancelled"));
}

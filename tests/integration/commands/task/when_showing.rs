use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_errors_on_nonexistent_task() {
  let env = GestCmd::new();

  env.run(&["task", "show", "00000000"]).failure();
}

#[test]
fn it_shows_a_task() {
  let env = GestCmd::new();
  let id = env.create_task("Show me");

  env
    .run(&["task", "show", &id])
    .success()
    .stdout(predicate::str::contains("Show me"));
}

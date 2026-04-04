use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_blocks_two_tasks() {
  let env = GestCmd::new();
  let id1 = env.create_task("Blocker task");
  let id2 = env.create_task("Blocked task");

  env
    .run(&["task", "block", &id1, &id2])
    .success()
    .stdout(predicate::str::contains("Linked"));
}

#[test]
fn it_blocks_with_artifact_flag() {
  let env = GestCmd::new();
  let task_id = env.create_task("Blocker task");
  let artifact_id = env.create_artifact("Some artifact", "body text");

  env
    .run(&["task", "block", &task_id, &artifact_id, "--artifact"])
    .success()
    .stdout(predicate::str::contains("Linked"));
}

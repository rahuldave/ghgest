use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_links_two_tasks() {
  let env = GestCmd::new();
  let id1 = env.create_task("Blocker task");
  let id2 = env.create_task("Blocked task");

  env
    .run(&["task", "link", &id1, "blocks", &id2])
    .success()
    .stdout(predicate::str::contains("Linked"));
}

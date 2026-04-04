use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_removes_tags_from_a_task() {
  let env = GestCmd::new();
  let id = env.create_task("untag target");

  env.run(&["tag", "add", &id, "keep", "drop"]).success();

  env
    .run(&["tag", "remove", &id, "drop"])
    .success()
    .stdout(predicate::str::contains("Untagged task"));

  env
    .run(&["task", "show", &id])
    .success()
    .stdout(predicate::str::contains("keep"))
    .stdout(predicate::str::contains("drop").not());
}

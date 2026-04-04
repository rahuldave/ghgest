use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_comma_splits_tags() {
  let env = GestCmd::new();
  let id = env.create_task("Comma tag target");

  env.run(&["task", "tag", &id, "rust,cli"]).success();

  env
    .run(&["task", "show", &id])
    .success()
    .stdout(predicate::str::contains("rust").and(predicate::str::contains("cli")));
}

#[test]
fn it_tags_a_task() {
  let env = GestCmd::new();
  let id = env.create_task("Tag target");

  env.run(&["task", "tag", &id, "mytag"]).success();

  env
    .run(&["task", "show", &id])
    .success()
    .stdout(predicate::str::contains("mytag"));
}

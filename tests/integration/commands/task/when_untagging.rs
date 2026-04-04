use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_comma_splits_tags_on_untag() {
  let env = GestCmd::new();
  let id = env.create_task("Comma untag target");

  env.run(&["task", "tag", &id, "rust", "cli", "keep"]).success();

  env.run(&["task", "untag", &id, "rust,cli"]).success();

  env.run(&["task", "show", &id]).success().stdout(
    predicate::str::contains("keep")
      .and(predicate::str::contains("rust").not())
      .and(predicate::str::contains("cli").not()),
  );
}

#[test]
fn it_untags_a_task() {
  let env = GestCmd::new();
  let id = env.create_task("Untag target");

  env.run(&["task", "tag", &id, "removeme"]).success();

  env
    .run(&["task", "show", &id])
    .success()
    .stdout(predicate::str::contains("removeme"));

  env.run(&["task", "untag", &id, "removeme"]).success();

  env
    .run(&["task", "show", &id])
    .success()
    .stdout(predicate::str::contains("removeme").not());
}

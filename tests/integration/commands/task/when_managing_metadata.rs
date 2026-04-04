use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_gets_metadata() {
  let env = GestCmd::new();
  let id = env.create_task("Meta get task");

  env.run(&["task", "meta", "set", &id, "priority", "high"]).success();

  env
    .run(&["task", "meta", "get", &id, "priority"])
    .success()
    .stdout(predicate::str::contains("high"));
}

#[test]
fn it_gets_metadata_raw() {
  let env = GestCmd::new();
  let id = env.create_task("Meta get raw task");

  env.run(&["task", "meta", "set", &id, "foo", "bar"]).success();

  env
    .run(&["task", "meta", "get", &id, "foo", "--raw"])
    .success()
    .stdout(predicate::eq("bar\n"));
}

#[test]
fn it_gets_metadata_json() {
  let env = GestCmd::new();
  let id = env.create_task("Meta get json task");

  env.run(&["task", "meta", "set", &id, "foo", "bar"]).success();

  env
    .run(&["task", "meta", "get", &id, "foo", "--json"])
    .success()
    .stdout(predicate::str::contains(r#""foo": "bar""#));
}

#[test]
fn it_sets_metadata() {
  let env = GestCmd::new();
  let id = env.create_task("Meta task");

  env.run(&["task", "meta", "set", &id, "priority", "high"]).success();
}

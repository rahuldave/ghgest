use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_marks_task_as_done() {
  let env = GestCmd::new();
  let id = env.create_task("Complete me");

  env
    .run(&["task", "complete", &id])
    .success()
    .stdout(predicate::str::contains("done"));
}

#[test]
fn it_produces_same_output_as_update_status_done() {
  let env = GestCmd::new();
  let id_complete = env.create_task("Via complete");
  let id_update = env.create_task("Via update");

  let complete_output = env
    .cmd()
    .args(["task", "complete", &id_complete])
    .output()
    .expect("complete failed");
  let update_output = env
    .cmd()
    .args(["task", "update", &id_update, "--status", "done"])
    .output()
    .expect("update failed");

  let complete_stdout = String::from_utf8(complete_output.stdout).expect("not utf8");
  let update_stdout = String::from_utf8(update_output.stdout).expect("not utf8");

  // Normalize output by removing the task-specific ID prefix so the structure matches.
  let complete_normalized = complete_stdout.replace(&id_complete, "ID");
  let update_normalized = update_stdout.replace(&id_update, "ID");

  assert_eq!(complete_normalized, update_normalized);
}

#[test]
fn it_shows_task_as_done_in_show() {
  let env = GestCmd::new();
  let id = env.create_task("Verify done");

  env.run(&["task", "complete", &id]).success();

  env
    .run(&["task", "show", &id])
    .success()
    .stdout(predicate::str::contains("done"));
}

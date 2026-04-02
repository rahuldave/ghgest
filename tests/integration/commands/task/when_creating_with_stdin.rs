use predicates::prelude::*;

use crate::support::helpers::{GestCmd, extract_id_from_create_output};

#[test]
fn it_includes_piped_stdin_as_description() {
  let env = GestCmd::new();

  let output = env
    .cmd()
    .args(["task", "create", "test"])
    .write_stdin("piped content\n")
    .output()
    .expect("failed to run task create");

  assert!(output.status.success());

  let stdout = String::from_utf8_lossy(&output.stdout);
  let id = extract_id_from_create_output(&stdout).expect("could not extract task ID");

  env
    .run(&["task", "show", &id, "--json"])
    .success()
    .stdout(predicate::str::contains("piped content"));
}

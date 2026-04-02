use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_accepts_s_alias() {
  let env = GestCmd::new();

  // The `s` alias should be recognized as `serve` and show serve help.
  env
    .run(&["s", "--help"])
    .success()
    .stdout(predicate::str::contains("Start a local web server"));
}

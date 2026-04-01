use predicates::prelude::*;

use crate::support::helpers::GestCmd;

fn expected_version() -> String {
  format!("v{}", env!("CARGO_PKG_VERSION"))
}

#[test]
fn it_shows_version() {
  let env = GestCmd::new_uninit();

  env
    .raw_cmd()
    .args(["version"])
    .assert()
    .success()
    .stdout(predicate::str::contains(expected_version()));
}

#[test]
fn it_shows_version_with_flag() {
  let env = GestCmd::new_uninit();

  env
    .raw_cmd()
    .arg("--version")
    .assert()
    .success()
    .stdout(predicate::str::contains(expected_version()));
}

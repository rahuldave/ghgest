use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_accepts_u_alias() {
  let env = GestCmd::new();

  // The `u` alias should behave identically to `undo`.
  env
    .run(&["u"])
    .failure()
    .stderr(predicate::str::contains("Nothing to undo"));
}

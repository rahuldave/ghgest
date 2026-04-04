use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_outputs_only_the_short_id() {
  let env = GestCmd::new();

  env
    .cmd()
    .args([
      "artifact",
      "create",
      "--title",
      "Quiet Artifact",
      "--body",
      "body text",
      "-q",
    ])
    .assert()
    .success()
    .stdout(predicate::str::is_match(r"^[k-z]{8}\n$").unwrap());
}

#[test]
fn it_outputs_exactly_8_characters() {
  let env = GestCmd::new();

  let output = env
    .cmd()
    .args([
      "artifact",
      "create",
      "--title",
      "Quiet Artifact",
      "--body",
      "body text",
      "-q",
    ])
    .output()
    .expect("failed to run artifact create -q");

  assert!(output.status.success());

  let stdout = String::from_utf8(output.stdout).expect("stdout is not valid utf8");
  let id = stdout.trim();
  assert_eq!(
    id.len(),
    8,
    "expected 8-character short ID, got {id:?} ({} chars)",
    id.len()
  );
}

#[test]
fn it_outputs_only_characters_in_the_k_to_z_range() {
  let env = GestCmd::new();

  let output = env
    .cmd()
    .args([
      "artifact",
      "create",
      "--title",
      "Quiet Artifact",
      "--body",
      "body text",
      "-q",
    ])
    .output()
    .expect("failed to run artifact create -q");

  assert!(output.status.success());

  let stdout = String::from_utf8(output.stdout).expect("stdout is not valid utf8");
  let id = stdout.trim();
  assert!(
    id.chars().all(|c| ('k'..='z').contains(&c)),
    "expected all characters in k-z range, got {id:?}"
  );
}

#[test]
fn it_does_not_output_the_full_32_character_id() {
  let env = GestCmd::new();

  env
    .cmd()
    .args([
      "artifact",
      "create",
      "--title",
      "Quiet Artifact",
      "--body",
      "body text",
      "-q",
    ])
    .assert()
    .success()
    .stdout(predicate::str::is_match(r"^[k-z]{32}").unwrap().not());
}

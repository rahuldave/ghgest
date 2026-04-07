use crate::support::helpers::GestCmd;

#[test]
fn it_sets_and_gets_a_nested_value() {
  let g = GestCmd::new();
  let id = g.create_artifact("Meta nested artifact", "body");

  g.cmd()
    .args(["artifact", "meta", "set", &id, "outer.inner", "deep"])
    .assert()
    .success();

  let output = g
    .cmd()
    .args(["artifact", "meta", "get", &id, "outer.inner"])
    .output()
    .unwrap();
  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("deep"), "got: {stdout}");
}

#[test]
fn it_infers_scalar_types() {
  let g = GestCmd::new();
  let id = g.create_artifact("Meta scalar inference", "body");

  g.cmd()
    .args(["artifact", "meta", "set", &id, "count", "42"])
    .assert()
    .success();

  let output = g
    .cmd()
    .args(["artifact", "meta", "get", &id, "count", "--json"])
    .output()
    .unwrap();
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("42"), "got: {stdout}");
}

#[test]
fn it_parses_value_as_json_with_as_json_flag() {
  let g = GestCmd::new();
  let id = g.create_artifact("Meta as-json", "body");

  g.cmd()
    .args(["artifact", "meta", "set", &id, "tags", "[\"a\",\"b\"]", "--as-json"])
    .assert()
    .success();

  let output = g
    .cmd()
    .args(["artifact", "meta", "get", &id, "tags", "--json"])
    .output()
    .unwrap();
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("\"a\""), "got: {stdout}");
}

#[test]
fn it_unsets_an_existing_path() {
  let g = GestCmd::new();
  let id = g.create_artifact("Meta unset", "body");

  g.cmd()
    .args(["artifact", "meta", "set", &id, "k", "v"])
    .assert()
    .success();
  g.cmd().args(["artifact", "meta", "unset", &id, "k"]).assert().success();

  let output = g.cmd().args(["artifact", "meta", "get", &id, "k"]).output().unwrap();
  assert!(!output.status.success());
}

#[test]
fn it_errors_on_unset_missing_path() {
  let g = GestCmd::new();
  let id = g.create_artifact("Meta unset missing", "body");

  let output = g
    .cmd()
    .args(["artifact", "meta", "unset", &id, "missing"])
    .output()
    .unwrap();
  assert!(!output.status.success());
}

#[test]
fn it_supports_delete_alias() {
  let g = GestCmd::new();
  let id = g.create_artifact("Meta delete alias", "body");

  g.cmd()
    .args(["artifact", "meta", "set", &id, "k", "v"])
    .assert()
    .success();
  g.cmd()
    .args(["artifact", "meta", "delete", &id, "k"])
    .assert()
    .success();
}

#[test]
fn it_dumps_flat_pairs_with_raw_on_bare() {
  let g = GestCmd::new();
  let id = g.create_artifact("Meta raw bare", "body");

  g.cmd()
    .args(["artifact", "meta", "set", &id, "outer.inner", "deep"])
    .assert()
    .success();

  let output = g.cmd().args(["artifact", "meta", &id, "--raw"]).output().unwrap();
  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("outer.inner=deep"), "got: {stdout}");
}

#[test]
fn it_prints_bare_scalar_with_raw_on_get() {
  let g = GestCmd::new();
  let id = g.create_artifact("Meta raw get", "body");

  g.cmd()
    .args(["artifact", "meta", "set", &id, "k", "hello"])
    .assert()
    .success();

  let output = g
    .cmd()
    .args(["artifact", "meta", "get", &id, "k", "--raw"])
    .output()
    .unwrap();
  assert!(output.status.success());
  assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "hello");
}

#[test]
fn it_prints_wrapped_json_shape_on_get() {
  let g = GestCmd::new();
  let id = g.create_artifact("Meta wrapped json", "body");

  g.cmd()
    .args(["artifact", "meta", "set", &id, "outer.inner", "deep"])
    .assert()
    .success();

  let output = g
    .cmd()
    .args(["artifact", "meta", "get", &id, "outer.inner", "--json"])
    .output()
    .unwrap();
  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  let parsed: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid json");
  assert_eq!(parsed["outer.inner"], serde_json::json!("deep"));
}

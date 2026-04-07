use crate::support::helpers::{GestCmd, extract_id_from_create_output};

fn read_meta(g: &GestCmd, id: &str, path: &str) -> String {
  let output = g
    .cmd()
    .args(["iteration", "meta", "get", id, path, "--raw"])
    .output()
    .unwrap();
  assert!(output.status.success(), "meta get failed");
  String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn it_sets_and_gets_a_nested_value() {
  let g = GestCmd::new();
  let id = g.create_iteration("Meta nested iter");

  g.cmd()
    .args(["iteration", "meta", "set", &id, "outer.inner", "deep"])
    .assert()
    .success();

  let output = g
    .cmd()
    .args(["iteration", "meta", "get", &id, "outer.inner"])
    .output()
    .expect("iteration meta get failed");

  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("deep"), "got: {stdout}");
}

#[test]
fn it_infers_scalar_types() {
  let g = GestCmd::new();
  let id = g.create_iteration("Meta scalar inference");

  g.cmd()
    .args(["iteration", "meta", "set", &id, "flag", "true"])
    .assert()
    .success();
  g.cmd()
    .args(["iteration", "meta", "set", &id, "count", "42"])
    .assert()
    .success();

  let output = g
    .cmd()
    .args(["iteration", "meta", "get", &id, "count", "--json"])
    .output()
    .unwrap();
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("42"), "got: {stdout}");
}

#[test]
fn it_parses_value_as_json_with_as_json_flag() {
  let g = GestCmd::new();
  let id = g.create_iteration("Meta as-json");

  g.cmd()
    .args(["iteration", "meta", "set", &id, "tags", "[\"a\",\"b\"]", "--as-json"])
    .assert()
    .success();

  let output = g
    .cmd()
    .args(["iteration", "meta", "get", &id, "tags", "--json"])
    .output()
    .unwrap();
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("\"a\""), "got: {stdout}");
}

#[test]
fn it_unsets_an_existing_path() {
  let g = GestCmd::new();
  let id = g.create_iteration("Meta unset");

  g.cmd()
    .args(["iteration", "meta", "set", &id, "k", "v"])
    .assert()
    .success();
  g.cmd()
    .args(["iteration", "meta", "unset", &id, "k"])
    .assert()
    .success();

  let output = g.cmd().args(["iteration", "meta", "get", &id, "k"]).output().unwrap();
  assert!(!output.status.success());
}

#[test]
fn it_errors_on_unset_missing_path() {
  let g = GestCmd::new();
  let id = g.create_iteration("Meta unset missing");

  let output = g
    .cmd()
    .args(["iteration", "meta", "unset", &id, "missing"])
    .output()
    .unwrap();
  assert!(!output.status.success());
}

#[test]
fn it_supports_delete_alias() {
  let g = GestCmd::new();
  let id = g.create_iteration("Meta delete alias");

  g.cmd()
    .args(["iteration", "meta", "set", &id, "k", "v"])
    .assert()
    .success();
  g.cmd()
    .args(["iteration", "meta", "delete", &id, "k"])
    .assert()
    .success();
}

#[test]
fn it_dumps_flat_pairs_with_raw_on_bare() {
  let g = GestCmd::new();
  let id = g.create_iteration("Meta raw bare");

  g.cmd()
    .args(["iteration", "meta", "set", &id, "outer.inner", "deep"])
    .assert()
    .success();

  let output = g.cmd().args(["iteration", "meta", &id, "--raw"]).output().unwrap();
  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("outer.inner=deep"), "got: {stdout}");
}

#[test]
fn it_prints_bare_scalar_with_raw_on_get() {
  let g = GestCmd::new();
  let id = g.create_iteration("Meta raw get");

  g.cmd()
    .args(["iteration", "meta", "set", &id, "k", "hello"])
    .assert()
    .success();

  let output = g
    .cmd()
    .args(["iteration", "meta", "get", &id, "k", "--raw"])
    .output()
    .unwrap();
  assert!(output.status.success());
  assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "hello");
}

#[test]
fn it_prints_wrapped_json_shape_on_get() {
  let g = GestCmd::new();
  let id = g.create_iteration("Meta wrapped json");

  g.cmd()
    .args(["iteration", "meta", "set", &id, "outer.inner", "deep"])
    .assert()
    .success();

  let output = g
    .cmd()
    .args(["iteration", "meta", "get", &id, "outer.inner", "--json"])
    .output()
    .unwrap();
  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  let parsed: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid json");
  assert_eq!(parsed["outer.inner"], serde_json::json!("deep"));
}

#[test]
fn it_creates_iteration_with_scalar_pairs() {
  let g = GestCmd::new();

  let output = g
    .cmd()
    .args(["iteration", "create", "Title", "-m", "count=42", "-m", "name=alice"])
    .output()
    .unwrap();
  assert!(output.status.success());
  let id = extract_id_from_create_output(&String::from_utf8_lossy(&output.stdout)).unwrap();

  assert_eq!(read_meta(&g, &id, "count"), "42");
  assert_eq!(read_meta(&g, &id, "name"), "alice");
}

#[test]
fn it_creates_iteration_with_dot_path_keys() {
  let g = GestCmd::new();

  let output = g
    .cmd()
    .args(["iteration", "create", "Title", "--metadata", "outer.inner=deep"])
    .output()
    .unwrap();
  assert!(output.status.success());
  let id = extract_id_from_create_output(&String::from_utf8_lossy(&output.stdout)).unwrap();

  assert_eq!(read_meta(&g, &id, "outer.inner"), "deep");
}

#[test]
fn it_creates_iteration_with_metadata_json_merge() {
  let g = GestCmd::new();

  let output = g
    .cmd()
    .args([
      "iteration",
      "create",
      "Title",
      "-m",
      "k=1",
      "--metadata-json",
      r#"{"k":2,"extra":true}"#,
    ])
    .output()
    .unwrap();
  assert!(output.status.success());
  let id = extract_id_from_create_output(&String::from_utf8_lossy(&output.stdout)).unwrap();

  assert_eq!(read_meta(&g, &id, "k"), "2");
  assert_eq!(read_meta(&g, &id, "extra"), "true");
}

#[test]
fn it_updates_iteration_preserving_unrelated_keys() {
  let g = GestCmd::new();
  let id = g.create_iteration("Title");

  g.cmd()
    .args(["iteration", "meta", "set", &id, "keep", "yes"])
    .assert()
    .success();

  g.cmd()
    .args(["iteration", "update", &id, "-m", "added=new"])
    .assert()
    .success();

  assert_eq!(read_meta(&g, &id, "keep"), "yes");
  assert_eq!(read_meta(&g, &id, "added"), "new");
}

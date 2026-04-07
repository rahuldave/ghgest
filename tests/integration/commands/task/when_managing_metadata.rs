use crate::support::helpers::{GestCmd, extract_id_from_create_output};

fn read_meta(g: &GestCmd, id: &str, path: &str) -> String {
  let output = g
    .cmd()
    .args(["task", "meta", "get", id, path, "--raw"])
    .output()
    .unwrap();
  assert!(output.status.success(), "meta get failed");
  String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn it_sets_and_gets_a_nested_value() {
  let g = GestCmd::new();
  let id = g.create_task("Meta nested task");

  g.cmd()
    .args(["task", "meta", "set", &id, "outer.inner", "deep"])
    .assert()
    .success();

  let output = g
    .cmd()
    .args(["task", "meta", "get", &id, "outer.inner"])
    .output()
    .expect("task meta get failed");

  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("deep"), "got: {stdout}");
}

#[test]
fn it_infers_scalar_types() {
  let g = GestCmd::new();
  let id = g.create_task("Meta scalar inference");

  g.cmd()
    .args(["task", "meta", "set", &id, "flag", "true"])
    .assert()
    .success();
  g.cmd()
    .args(["task", "meta", "set", &id, "count", "42"])
    .assert()
    .success();
  g.cmd()
    .args(["task", "meta", "set", &id, "ratio", "3.14"])
    .assert()
    .success();
  g.cmd()
    .args(["task", "meta", "set", &id, "name", "alice"])
    .assert()
    .success();

  let output = g
    .cmd()
    .args(["task", "meta", "get", &id, "flag", "--json"])
    .output()
    .unwrap();
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("true") && !stdout.contains("\"true\""), "got: {stdout}");

  let output = g
    .cmd()
    .args(["task", "meta", "get", &id, "count", "--json"])
    .output()
    .unwrap();
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("42"), "got: {stdout}");

  let output = g
    .cmd()
    .args(["task", "meta", "get", &id, "ratio", "--json"])
    .output()
    .unwrap();
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("3.14"), "got: {stdout}");

  let output = g
    .cmd()
    .args(["task", "meta", "get", &id, "name", "--json"])
    .output()
    .unwrap();
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("\"alice\""), "got: {stdout}");
}

#[test]
fn it_parses_value_as_json_with_as_json_flag() {
  let g = GestCmd::new();
  let id = g.create_task("Meta as-json");

  g.cmd()
    .args(["task", "meta", "set", &id, "tags", "[\"a\",\"b\"]", "--as-json"])
    .assert()
    .success();

  let output = g
    .cmd()
    .args(["task", "meta", "get", &id, "tags", "--json"])
    .output()
    .unwrap();
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("\"a\""), "got: {stdout}");
  assert!(stdout.contains("\"b\""), "got: {stdout}");
}

#[test]
fn it_unsets_an_existing_path() {
  let g = GestCmd::new();
  let id = g.create_task("Meta unset");

  g.cmd().args(["task", "meta", "set", &id, "k", "v"]).assert().success();
  g.cmd().args(["task", "meta", "unset", &id, "k"]).assert().success();

  let output = g
    .cmd()
    .args(["task", "meta", "get", &id, "k"])
    .output()
    .expect("task meta get failed");

  assert!(!output.status.success(), "expected get to fail after unset");
}

#[test]
fn it_errors_on_unset_missing_path() {
  let g = GestCmd::new();
  let id = g.create_task("Meta unset missing");

  let output = g
    .cmd()
    .args(["task", "meta", "unset", &id, "missing"])
    .output()
    .unwrap();

  assert!(!output.status.success(), "expected unset of missing path to fail");
}

#[test]
fn it_supports_delete_alias_for_unset() {
  let g = GestCmd::new();
  let id = g.create_task("Meta delete alias");

  g.cmd().args(["task", "meta", "set", &id, "k", "v"]).assert().success();
  g.cmd().args(["task", "meta", "delete", &id, "k"]).assert().success();
}

#[test]
fn it_dumps_flat_pairs_with_raw_on_bare() {
  let g = GestCmd::new();
  let id = g.create_task("Meta raw bare");

  g.cmd()
    .args(["task", "meta", "set", &id, "outer.inner", "deep"])
    .assert()
    .success();
  g.cmd()
    .args(["task", "meta", "set", &id, "flat", "1"])
    .assert()
    .success();

  let output = g.cmd().args(["task", "meta", &id, "--raw"]).output().unwrap();

  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("outer.inner=deep"), "got: {stdout}");
  assert!(stdout.contains("flat=1"), "got: {stdout}");
}

#[test]
fn it_prints_bare_scalar_with_raw_on_get() {
  let g = GestCmd::new();
  let id = g.create_task("Meta raw get");

  g.cmd()
    .args(["task", "meta", "set", &id, "k", "hello"])
    .assert()
    .success();

  let output = g
    .cmd()
    .args(["task", "meta", "get", &id, "k", "--raw"])
    .output()
    .unwrap();

  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert_eq!(stdout.trim(), "hello");
}

#[test]
fn it_prints_wrapped_json_shape_on_get() {
  let g = GestCmd::new();
  let id = g.create_task("Meta wrapped json");

  g.cmd()
    .args(["task", "meta", "set", &id, "outer.inner", "deep"])
    .assert()
    .success();

  let output = g
    .cmd()
    .args(["task", "meta", "get", &id, "outer.inner", "--json"])
    .output()
    .unwrap();

  assert!(output.status.success());
  let stdout = String::from_utf8_lossy(&output.stdout);
  let parsed: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid json");
  assert_eq!(parsed["outer.inner"], serde_json::json!("deep"));
}

#[test]
fn it_creates_task_with_scalar_pairs() {
  let g = GestCmd::new();

  let output = g
    .cmd()
    .args(["task", "create", "Title", "-m", "count=42", "-m", "name=alice"])
    .output()
    .unwrap();
  assert!(output.status.success());
  let id = extract_id_from_create_output(&String::from_utf8_lossy(&output.stdout)).unwrap();

  assert_eq!(read_meta(&g, &id, "count"), "42");
  assert_eq!(read_meta(&g, &id, "name"), "alice");
}

#[test]
fn it_creates_task_with_dot_path_keys() {
  let g = GestCmd::new();

  let output = g
    .cmd()
    .args(["task", "create", "Title", "--metadata", "outer.inner=deep"])
    .output()
    .unwrap();
  assert!(output.status.success());
  let id = extract_id_from_create_output(&String::from_utf8_lossy(&output.stdout)).unwrap();

  assert_eq!(read_meta(&g, &id, "outer.inner"), "deep");
}

#[test]
fn it_creates_task_with_metadata_json_merge() {
  let g = GestCmd::new();

  let output = g
    .cmd()
    .args([
      "task",
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
fn it_updates_task_preserving_unrelated_keys() {
  let g = GestCmd::new();
  let id = g.create_task("Title");

  g.cmd()
    .args(["task", "meta", "set", &id, "keep", "yes"])
    .assert()
    .success();

  g.cmd()
    .args(["task", "update", &id, "-m", "added=new"])
    .assert()
    .success();

  assert_eq!(read_meta(&g, &id, "keep"), "yes");
  assert_eq!(read_meta(&g, &id, "added"), "new");
}

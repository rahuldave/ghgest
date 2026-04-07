use crate::support::helpers::GestCmd;

#[test]
fn it_gets_a_config_value() {
  let g = GestCmd::new();
  let output = g
    .cmd()
    .args(["config", "get", "log.level"])
    .output()
    .expect("config get failed to run");

  assert!(output.status.success(), "config get exited non-zero");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("warn"), "got: {stdout}");
}

#[test]
fn it_gets_a_config_value_as_json() {
  let g = GestCmd::new();
  let output = g
    .cmd()
    .args(["config", "get", "log.level", "--json"])
    .output()
    .expect("config get --json failed to run");

  assert!(output.status.success(), "config get --json exited non-zero");
  let stdout = String::from_utf8_lossy(&output.stdout);
  let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("output should be valid JSON");
  assert_eq!(parsed.as_str(), Some("warn"));
}

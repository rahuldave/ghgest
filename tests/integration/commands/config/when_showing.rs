use predicates::prelude::*;

use crate::support::helpers::GestCmd;

#[test]
fn it_shows_config() {
  let env = GestCmd::new();

  env
    .cmd()
    .args(["config", "show"])
    .assert()
    .success()
    .stdout(predicate::str::contains("configuration"))
    .stdout(predicate::str::contains("project_dir"))
    .stdout(predicate::str::contains("log_level"));
}

#[test]
fn it_shows_palette_count_when_palette_overrides_are_configured() {
  let env = GestCmd::new();

  let config_path = env.temp_dir_path().join("gest.toml");
  std::fs::write(&config_path, "[colors.palette]\nprimary = \"#9448C7\"\n").expect("failed to write config file");

  env
    .cmd()
    .args(["config", "show"])
    .assert()
    .success()
    .stdout(predicate::str::contains("palette"))
    .stdout(predicate::str::contains("1 palette color(s) set"));
}

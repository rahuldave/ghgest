use crate::support::helpers::GestCmd;

#[test]
fn it_generates_bash_completions() {
  let g = GestCmd::new_uninit();

  let output = g
    .cmd()
    .args(["generate", "completions", "bash"])
    .output()
    .expect("generate completions bash failed");

  assert!(output.status.success(), "bash completions should succeed");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(!stdout.is_empty(), "bash completions should produce output");
  assert!(
    stdout.contains("complete") || stdout.contains("_gest"),
    "bash completions should contain bash-complete markers, got preview: {}",
    &stdout[..stdout.len().min(200)]
  );
}

#[test]
fn it_generates_fish_completions() {
  let g = GestCmd::new_uninit();

  let output = g
    .cmd()
    .args(["generate", "completions", "fish"])
    .output()
    .expect("generate completions fish failed");

  assert!(output.status.success(), "fish completions should succeed");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(!stdout.is_empty(), "fish completions should produce output");
  assert!(
    stdout.contains("complete -c gest"),
    "fish completions should contain `complete -c gest`, got preview: {}",
    &stdout[..stdout.len().min(200)]
  );
}

#[test]
fn it_generates_zsh_completions() {
  let g = GestCmd::new_uninit();

  let output = g
    .cmd()
    .args(["generate", "completions", "zsh"])
    .output()
    .expect("generate completions zsh failed");

  assert!(output.status.success(), "zsh completions should succeed");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(!stdout.is_empty(), "zsh completions should produce output");
  assert!(
    stdout.contains("#compdef gest") || stdout.contains("_gest"),
    "zsh completions should contain compdef or _gest, got preview: {}",
    &stdout[..stdout.len().min(200)]
  );
}

#[test]
fn it_errors_on_unknown_shell() {
  let g = GestCmd::new_uninit();

  let output = g
    .cmd()
    .args(["generate", "completions", "definitely-not-a-shell"])
    .output()
    .expect("generate completions invalid failed");

  assert!(!output.status.success(), "unknown shell should exit non-zero");
}

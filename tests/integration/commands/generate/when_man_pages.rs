use crate::support::helpers::GestCmd;

#[test]
fn it_creates_expected_page_count() {
  let g = GestCmd::new_uninit();
  let out_dir = g.temp_dir_path().join("manpages-count");

  g.cmd()
    .args(["generate", "man-pages", out_dir.to_str().unwrap()])
    .assert()
    .success();

  let entries: Vec<_> = std::fs::read_dir(&out_dir)
    .expect("man pages dir should exist")
    .filter_map(|e| e.ok())
    .filter(|e| e.file_name().to_string_lossy().ends_with(".1"))
    .collect();

  assert!(!entries.is_empty(), "expected at least one .1 man page, got 0 entries");
  assert!(
    entries.iter().any(|e| e.file_name() == "gest.1"),
    "expected gest.1 root man page, got: {:?}",
    entries.iter().map(|e| e.file_name()).collect::<Vec<_>>()
  );
}

#[test]
fn it_generates_man_pages_to_directory() {
  let g = GestCmd::new_uninit();
  let out_dir = g.temp_dir_path().join("manpages");

  let output = g
    .cmd()
    .args(["generate", "man-pages", out_dir.to_str().unwrap()])
    .output()
    .expect("generate man-pages failed");

  assert!(
    output.status.success(),
    "generate man-pages should succeed: {}",
    String::from_utf8_lossy(&output.stderr)
  );
  assert!(out_dir.exists(), "man pages output directory should exist");

  // At least the top-level gest.1 should be generated.
  let entries: Vec<_> = std::fs::read_dir(&out_dir)
    .expect("man pages dir should be readable")
    .filter_map(|e| e.ok())
    .map(|e| e.file_name().to_string_lossy().to_string())
    .collect();
  assert!(
    entries.iter().any(|name| name == "gest.1"),
    "expected gest.1 in output directory, got: {entries:?}"
  );
}

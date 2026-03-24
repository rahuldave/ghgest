use std::process::Command;

fn main() {
  let date = chrono_free_date();
  println!("cargo:rustc-env=BUILD_DATE={date}");

  let sha = git_short_sha().unwrap_or_else(|| "unknown".to_string());
  println!("cargo:rustc-env=GIT_SHA={sha}");
}

fn chrono_free_date() -> String {
  // Use the `date` command to avoid pulling in chrono at build time.
  Command::new("date")
    .arg("+%Y-%m-%d")
    .output()
    .ok()
    .and_then(|o| String::from_utf8(o.stdout).ok())
    .map(|s| s.trim().to_string())
    .unwrap_or_else(|| "unknown".to_string())
}

fn git_short_sha() -> Option<String> {
  let output = Command::new("git")
    .args(["rev-parse", "--short", "HEAD"])
    .output()
    .ok()?;
  if !output.status.success() {
    return None;
  }
  let sha = String::from_utf8(output.stdout).ok()?;
  let trimmed = sha.trim();
  if trimmed.is_empty() {
    None
  } else {
    Some(trimmed.to_string())
  }
}

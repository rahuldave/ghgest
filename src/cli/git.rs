use std::process::Command;

/// Author identity resolved from git config.
pub struct GitAuthor {
  pub name: String,
  pub email: Option<String>,
}

/// Resolve the current git user identity from `git config`.
///
/// Reads `user.name` and `user.email` from git config. Returns `None` if
/// `user.name` is not configured. `email` is optional and may be `None`
/// even when `name` is present.
pub fn resolve_author() -> Option<GitAuthor> {
  let name = git_config("user.name")?;
  let email = git_config("user.email");
  Some(GitAuthor {
    name,
    email,
  })
}

fn git_config(key: &str) -> Option<String> {
  let output = Command::new("git").args(["config", key]).output().ok()?;
  if !output.status.success() {
    return None;
  }
  let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
  if value.is_empty() { None } else { Some(value) }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod git_config_fn {
    use super::*;

    #[test]
    fn it_returns_none_for_nonexistent_key() {
      assert!(git_config("gest.nonexistent.test.key").is_none());
    }
  }
}

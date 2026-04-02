use std::{
  io::{IsTerminal, Write},
  process::{Command, Stdio},
};

use crate::config::env::PAGER;

/// Pipe `content` through the user's preferred pager when stdout is a terminal.
///
/// Respects `$PAGER`, falling back to `less -R` (which passes ANSI escapes through).
/// When stdout is not a terminal (e.g. piped to another process) the content is
/// written directly to stdout without spawning a pager.
pub fn page(content: &str) -> crate::cli::Result<()> {
  if !std::io::stdout().is_terminal() {
    print!("{content}");
    return Ok(());
  }

  let pager_cmd = resolve_pager();
  let parts = shell_words::split(&pager_cmd)
    .map_err(|e| crate::cli::Error::Runtime(format!("Failed to parse pager command: {e}")))?;

  let (program, args) = match parts.split_first() {
    Some(pair) => pair,
    None => {
      // Empty pager command — fall back to direct output.
      print!("{content}");
      return Ok(());
    }
  };

  let mut child = match Command::new(program).args(args).stdin(Stdio::piped()).spawn() {
    Ok(child) => child,
    Err(_) => {
      // Pager not found — fall back to direct output.
      print!("{content}");
      return Ok(());
    }
  };

  if let Some(mut stdin) = child.stdin.take() {
    // Ignore broken-pipe errors (user quit the pager early).
    let _ = stdin.write_all(content.as_bytes());
  }

  child.wait()?;
  Ok(())
}

/// Return the pager command: `$PAGER` if set and non-empty, otherwise `less -R`.
fn resolve_pager() -> String {
  PAGER
    .value()
    .ok()
    .filter(|s| !s.is_empty())
    .unwrap_or_else(|| "less -R".to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  mod resolve_pager {
    use temp_env::with_var;

    use super::*;

    #[test]
    fn it_defaults_to_less() {
      with_var("PAGER", None::<&str>, || {
        let pager = resolve_pager();
        assert_eq!(pager, "less -R");
      });
    }

    #[test]
    fn it_respects_pager_env() {
      with_var("PAGER", Some("more"), || {
        let pager = resolve_pager();
        assert_eq!(pager, "more");
      });
    }

    #[test]
    fn it_ignores_empty_pager() {
      with_var("PAGER", Some(""), || {
        let pager = resolve_pager();
        assert_eq!(pager, "less -R");
      });
    }
  }

  mod page {
    use super::*;

    #[test]
    fn it_does_not_error_on_content() {
      // In test, stdout is not a terminal, so this should bypass the pager.
      let result = page("hello world");
      assert!(result.is_ok());
    }
  }
}

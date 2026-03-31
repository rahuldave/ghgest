use std::io;

use clap::{Args, CommandFactory, ValueEnum};
use clap_complete::Shell;

use crate::cli;

/// Print shell completion scripts to stdout.
#[derive(Args, Debug)]
pub struct Command {
  /// Target shell for which to generate completions.
  #[arg(long)]
  shell: ShellArg,
}

impl Command {
  /// Generate and write the completion script to stdout.
  pub fn call(&self) -> cli::Result<()> {
    let mut cmd = crate::cli::Cli::command();
    let shell: Shell = self.shell.clone().into();
    clap_complete::generate(shell, &mut cmd, "gest", &mut io::stdout());
    Ok(())
  }
}

/// Supported shell targets for completion generation.
#[derive(Clone, Debug, ValueEnum)]
enum ShellArg {
  /// GNU Bourne-Again SHell.
  Bash,
  /// Elvish shell.
  Elvish,
  /// Friendly Interactive SHell.
  Fish,
  /// Microsoft PowerShell.
  #[value(name = "powershell")]
  PowerShell,
  /// Z shell.
  Zsh,
}

impl From<ShellArg> for Shell {
  fn from(arg: ShellArg) -> Self {
    match arg {
      ShellArg::Bash => Shell::Bash,
      ShellArg::Elvish => Shell::Elvish,
      ShellArg::Fish => Shell::Fish,
      ShellArg::PowerShell => Shell::PowerShell,
      ShellArg::Zsh => Shell::Zsh,
    }
  }
}

#[cfg(test)]
mod tests {
  use clap::CommandFactory;
  use clap_complete::Shell;

  fn generate_to_vec(shell: Shell) -> Vec<u8> {
    let mut cmd = crate::cli::Cli::command();
    let mut buf = Vec::new();
    clap_complete::generate(shell, &mut cmd, "gest", &mut buf);
    buf
  }

  mod call {
    use super::*;

    #[test]
    fn it_generates_bash_completions() {
      let output = generate_to_vec(Shell::Bash);
      assert!(!output.is_empty());
    }

    #[test]
    fn it_generates_fish_completions() {
      let output = generate_to_vec(Shell::Fish);
      assert!(!output.is_empty());
    }

    #[test]
    fn it_generates_zsh_completions() {
      let output = generate_to_vec(Shell::Zsh);
      assert!(!output.is_empty());
    }
  }
}

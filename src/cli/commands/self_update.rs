use std::fmt;

use clap::Args;

use crate::{config::Config, ui::theme::Theme};

/// Binary name.
const BIN_NAME: &str = "gest";

/// Current version from Cargo.toml, used to detect whether an update is needed.
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// GitHub repository name.
const REPO_NAME: &str = "gest";

/// GitHub repository owner.
const REPO_OWNER: &str = "aaronmallen";

/// Update gest to the latest (or a specific) release
#[derive(Debug, Args)]
pub struct Command {
  /// Pin to a specific version (bare semver, e.g. 1.2.3)
  #[arg(long)]
  target: Option<String>,
}

impl Command {
  pub fn call(&self, _config: &Config, theme: &Theme) -> crate::Result<()> {
    let releases = self_update::backends::github::ReleaseList::configure()
      .repo_owner(REPO_OWNER)
      .repo_name(REPO_NAME)
      .build()?
      .fetch()?;

    let latest = releases
      .first()
      .ok_or_else(|| crate::Error::generic("no releases found on GitHub"))?;

    let target_version = self.target.as_deref().unwrap_or(&latest.version);

    if target_version == CURRENT_VERSION {
      use yansi::Paint;
      println!("{} Already on v{target_version}", "OK".paint(theme.success),);
      return Ok(());
    }

    let diff = VersionDiff {
      current: CURRENT_VERSION,
      target: target_version,
    };

    // Prompt for confirmation
    println!("Update available: {diff}");
    print!("Do you want to continue? [y/N] ");
    use std::io::Write;
    std::io::stdout().flush()?;

    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer)?;
    let answer = answer.trim().to_lowercase();

    if answer != "y" && answer != "yes" {
      println!("Update cancelled.");
      return Ok(());
    }

    let status = self_update::backends::github::Update::configure()
      .repo_owner(REPO_OWNER)
      .repo_name(REPO_NAME)
      .bin_name(BIN_NAME)
      .target_version_tag(target_version)
      .show_download_progress(true)
      .current_version(CURRENT_VERSION)
      .no_confirm(true)
      .build()?
      .update()?;

    use yansi::Paint;
    println!("{} Updated to v{}", "OK".paint(theme.success), status.version(),);

    Ok(())
  }
}

/// Displays a version transition like `v0.0.1 -> v0.1.0`.
struct VersionDiff<'a> {
  current: &'a str,
  target: &'a str,
}

impl fmt::Display for VersionDiff<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "v{} -> v{}", self.current, self.target)
  }
}

use std::io::Write;

use clap::Args;

use crate::{
  cli,
  ui::{composites::success_message::SuccessMessage, theme::Theme},
};

const BIN_NAME: &str = "gest";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO_NAME: &str = "gest";
const REPO_OWNER: &str = "aaronmallen";

/// Update gest to the latest (or a pinned) GitHub release.
#[derive(Debug, Args)]
pub struct Command {
  /// Pin to a specific version (bare semver, e.g. `1.2.3`).
  #[arg(long)]
  target: Option<String>,
}

impl Command {
  /// Fetch releases, prompt for confirmation, and perform the in-place binary update.
  pub fn call(&self, theme: &Theme) -> cli::Result<()> {
    let releases = self_update::backends::github::ReleaseList::configure()
      .repo_owner(REPO_OWNER)
      .repo_name(REPO_NAME)
      .build()
      .map_err(|e| cli::Error::generic(e.to_string()))?
      .fetch()
      .map_err(|e| cli::Error::generic(e.to_string()))?;

    let latest = releases
      .first()
      .ok_or_else(|| cli::Error::generic("no releases found on GitHub"))?;

    let target_version = self.target.as_deref().unwrap_or(&latest.version);

    if target_version == CURRENT_VERSION {
      let msg = format!("Already on version {CURRENT_VERSION}");
      println!("{}", SuccessMessage::new(&msg, theme));
      return Ok(());
    }

    println!("Update available: {CURRENT_VERSION} → {target_version}");
    print!("Proceed? [y/N] ");
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
      .identifier("tar.gz")
      .show_download_progress(true)
      .current_version(CURRENT_VERSION)
      .no_confirm(true)
      .build()
      .map_err(|e| cli::Error::generic(e.to_string()))?
      .update()
      .map_err(|e| cli::Error::generic(e.to_string()))?;

    let msg = format!("Updated to version {}", status.version());
    println!("{}", SuccessMessage::new(&msg, theme));

    Ok(())
  }
}

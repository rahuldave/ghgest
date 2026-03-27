use std::thread;

use clap::Args;

use crate::{
  Result,
  config::Config,
  ui::{components::Banner, theme::Theme},
};

/// Print the current gest version and build information
#[derive(Args, Debug)]
pub struct Command;

impl Command {
  pub fn call(&self, _config: &Config, _theme: &Theme) -> Result<()> {
    // Spawn a background thread to check for newer releases
    let check_handle = thread::spawn(|| -> Option<String> {
      let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("aaronmallen")
        .repo_name("gest")
        .build()
        .ok()?
        .fetch()
        .ok()?;
      let latest = releases.first()?;
      let current = semver::Version::parse(env!("CARGO_PKG_VERSION")).ok()?;
      let remote = semver::Version::parse(&latest.version).ok()?;
      if remote > current {
        Some(latest.version.clone())
      } else {
        None
      }
    });

    // Print banner immediately (no latency added)
    println!("{}", Banner::new().with_color().with_author().with_version());

    // Join the background thread and warn if a newer version is available
    if let Ok(Some(latest_version)) = check_handle.join() {
      log::warn!("A newer version of gest is available: v{latest_version}. Run 'gest self-update' to update.");
    }

    Ok(())
  }
}

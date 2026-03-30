use std::thread;

use clap::Args;

use crate::ui::{composites::banner::Banner, theme::Theme};

/// Print the current version, platform info, and check for available updates.
#[derive(Args, Debug)]
pub struct Command;

impl Command {
  /// Display a version banner, appending an update notice if a newer release exists.
  pub fn call(&self, theme: &Theme) -> crate::cli::Result<()> {
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

    let mut banner = Banner::new(
      env!("CARGO_PKG_VERSION"),
      std::env::consts::OS,
      "",
      "",
      "aaronmallen",
      theme,
    );

    if let Ok(Some(latest_version)) = check_handle.join() {
      banner = banner.update_version(latest_version);
    }

    println!("{banner}");
    Ok(())
  }
}

use clap::Args;

use crate::{Result, ui::components::Banner};

/// Print the current gest version and build information
#[derive(Args, Debug)]
pub struct Command;

impl Command {
  pub fn call(&self) -> Result<()> {
    println!("{}", Banner::new().with_color().with_author().with_version());
    Ok(())
  }
}

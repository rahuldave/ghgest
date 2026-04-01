use clap::Args;

use crate::{
  cli::{self, AppContext},
  store,
  ui::composites::empty_list::EmptyList,
};

/// List all unique tags used across tasks, artifacts, and iterations.
#[derive(Debug, Args)]
pub struct Command;

impl Command {
  /// Collect every tag in the project and print them, one per line.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    let tags = store::list_tags(&ctx.settings, None)?;

    if tags.is_empty() {
      println!("{}", EmptyList::new("tags", &ctx.theme));
      return Ok(());
    }

    for tag in &tags {
      println!("{tag}");
    }

    Ok(())
  }
}

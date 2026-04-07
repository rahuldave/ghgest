//! Shared output-mode helpers for `--json` and `--quiet` flags.

use clap::Args;
use serde::Serialize;

use crate::cli::Error;

/// Common output-mode flags that can be flattened into any command struct.
#[derive(Args, Clone, Debug, Default)]
pub struct Flags {
  /// Emit output as JSON.
  #[arg(short, long)]
  pub json: bool,
  /// Suppress normal output (print only entity ID on create/mutate, nothing on delete).
  #[arg(short, long)]
  pub quiet: bool,
}

impl Flags {
  /// Print a single entity. In JSON mode the entity is serialized; in quiet mode
  /// only the short ID is printed; otherwise `normal` is called to produce the
  /// human-readable output.
  pub fn print_entity<T: Serialize>(
    &self,
    entity: &T,
    short_id: &str,
    normal: impl FnOnce() -> String,
  ) -> Result<(), Error> {
    if self.json {
      let json = serde_json::to_string_pretty(entity)?;
      println!("{json}");
    } else if self.quiet {
      println!("{short_id}");
    } else {
      println!("{}", normal());
    }
    Ok(())
  }

  /// Print a JSON-serializable value for JSON mode, nothing for quiet mode,
  /// or the normal display for human mode.
  pub fn print_json_or<T: Serialize>(&self, value: &T, normal: impl FnOnce() -> String) -> Result<(), Error> {
    if self.json {
      let json = serde_json::to_string_pretty(value)?;
      println!("{json}");
    } else if self.quiet {
      // quiet on list commands: print nothing
    } else {
      println!("{}", normal());
    }
    Ok(())
  }

  /// Print a list of entities. In JSON mode the list is serialized; in quiet mode
  /// only IDs are printed (one per line); otherwise `normal` is called.
  pub fn print_list<T: Serialize>(
    &self,
    entities: &[T],
    ids: &[String],
    normal: impl FnOnce() -> String,
  ) -> Result<(), Error> {
    if self.json {
      let json = serde_json::to_string_pretty(entities)?;
      println!("{json}");
    } else if self.quiet {
      for id in ids {
        println!("{id}");
      }
    } else {
      println!("{}", normal());
    }
    Ok(())
  }

  /// Print nothing (for delete in quiet mode), or the normal message.
  pub fn print_delete(&self, normal: impl FnOnce() -> String) -> Result<(), Error> {
    if self.json {
      // JSON mode on delete: print empty object
      println!("{{}}");
    } else if self.quiet {
      // nothing
    } else {
      println!("{}", normal());
    }
    Ok(())
  }
}

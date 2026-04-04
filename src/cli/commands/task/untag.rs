use clap::Args;

use crate::{
  action,
  cli::{self, AppContext},
  model::Task,
  ui::composites::success_message::SuccessMessage,
};

/// Remove tags from a task.
#[derive(Debug, Args)]
pub struct Command {
  /// Task ID or unique prefix.
  pub id: String,
  /// Output the task as JSON after untagging.
  #[arg(short, long, conflicts_with = "quiet")]
  pub json: bool,
  /// Output only the task ID.
  #[arg(short, long, conflicts_with = "json")]
  pub quiet: bool,
  /// Tags to remove (space or comma-separated).
  #[arg(value_delimiter = ',')]
  pub tags: Vec<String>,
}

impl Command {
  /// Remove the specified tags from the task and persist.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    let task = action::untag::<Task>(&ctx.settings, &self.id, &self.tags)?;

    if self.json {
      println!("{}", serde_json::to_string_pretty(&task)?);
    } else if self.quiet {
      println!("{}", task.id.short());
    } else {
      let msg = format!("Untagged task {} from {}", task.id, self.tags.join(", "));
      println!("{}", SuccessMessage::new(&msg, &ctx.theme));
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helpers::{make_test_context, make_test_task};

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::store;

    #[test]
    fn it_handles_nonexistent_tags_gracefully() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());
      let mut task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      task.tags = vec!["rust".to_string()];
      store::write_task(&ctx.settings, &task).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        json: false,
        quiet: false,
        tags: vec!["nonexistent".to_string()],
      };
      cmd.call(&ctx).unwrap();

      let loaded = store::read_task(&ctx.settings, &task.id).unwrap();
      assert_eq!(loaded.tags, vec!["rust".to_string()]);
    }

    #[test]
    fn it_removes_tags() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());
      let mut task = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      task.tags = vec!["rust".to_string(), "cli".to_string(), "keep".to_string()];
      store::write_task(&ctx.settings, &task).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        json: false,
        quiet: false,
        tags: vec!["rust".to_string(), "cli".to_string()],
      };
      cmd.call(&ctx).unwrap();

      let loaded = store::read_task(&ctx.settings, &task.id).unwrap();
      assert_eq!(loaded.tags, vec!["keep".to_string()]);
    }
  }
}

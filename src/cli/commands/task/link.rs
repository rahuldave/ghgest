use clap::Args;

use crate::{
  action,
  cli::{self, AppContext},
  model::{Task, link::RelationshipType},
  store,
  ui::composites::success_message::SuccessMessage,
};

/// Create a relationship between a task and another task or artifact.
#[derive(Debug, Args)]
pub struct Command {
  /// Target is an artifact instead of a task (no reciprocal link is created).
  #[arg(long)]
  pub artifact: bool,
  /// Source task ID or unique prefix.
  pub id: String,
  /// Output the task as JSON after linking.
  #[arg(short, long, conflicts_with = "quiet")]
  pub json: bool,
  /// Output only the task ID.
  #[arg(short, long, conflicts_with = "json")]
  pub quiet: bool,
  /// Relationship type (e.g. blocks, blocked-by, relates-to).
  #[arg(value_enum)]
  pub rel: RelationshipType,
  /// Target task or artifact ID or unique prefix.
  pub target_id: String,
}

impl Command {
  /// Add a link on the source task, and a reciprocal link on the target when both are tasks.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    let config = &ctx.settings;
    let theme = &ctx.theme;

    let (id, target_id) = action::link::link::<Task>(config, &self.id, &self.target_id, &self.rel, self.artifact)?;

    if self.json {
      let task = store::read_task(config, &id)?;
      println!("{}", serde_json::to_string_pretty(&task)?);
    } else if self.quiet {
      println!("{}", id.short());
    } else {
      let msg = format!("Linked {} --{}--\u{003e} {}", id.short(), self.rel, target_id.short());
      println!("{}", SuccessMessage::new(&msg, theme));
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use tempfile::TempDir;

  use super::*;
  use crate::{
    store,
    test_helpers::{make_test_artifact, make_test_context, make_test_task},
  };

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_appends_multiple_links() {
      let (dir, ctx) = setup();
      let source = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      let target1 = make_test_task("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
      let target2 = make_test_task("nnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnn");
      store::write_task(&ctx.settings, &source).unwrap();
      store::write_task(&ctx.settings, &target1).unwrap();
      store::write_task(&ctx.settings, &target2).unwrap();

      let cmd1 = Command {
        artifact: false,
        id: "zyxw".to_string(),
        json: false,
        quiet: false,
        rel: RelationshipType::Blocks,
        target_id: "kkkk".to_string(),
      };
      cmd1.call(&ctx).unwrap();

      let cmd2 = Command {
        artifact: false,
        id: "zyxw".to_string(),
        json: false,
        quiet: false,
        rel: RelationshipType::RelatesTo,
        target_id: "nnnn".to_string(),
      };
      cmd2.call(&ctx).unwrap();

      let loaded = store::read_task(&ctx.settings, &source.id).unwrap();

      assert_eq!(loaded.links.len(), 2);
      let _ = dir;
    }

    #[test]
    fn it_links_task_to_artifact() {
      let (dir, ctx) = setup();
      let source = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      let target = make_test_artifact("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
      store::write_task(&ctx.settings, &source).unwrap();
      store::write_artifact(&ctx.settings, &target).unwrap();

      let cmd = Command {
        artifact: true,
        id: "zyxw".to_string(),
        json: false,
        quiet: false,
        rel: RelationshipType::RelatesTo,
        target_id: "kkkk".to_string(),
      };
      cmd.call(&ctx).unwrap();

      let loaded = store::read_task(&ctx.settings, &source.id).unwrap();
      assert_eq!(loaded.links.len(), 1);
      assert_eq!(loaded.links[0].ref_, "artifacts/kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
      let _ = dir;
    }

    #[test]
    fn it_links_task_to_task_with_reciprocal() {
      let (dir, ctx) = setup();
      let source = make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      let target = make_test_task("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
      store::write_task(&ctx.settings, &source).unwrap();
      store::write_task(&ctx.settings, &target).unwrap();

      let cmd = Command {
        artifact: false,
        id: "zyxw".to_string(),
        json: false,
        quiet: false,
        rel: RelationshipType::Blocks,
        target_id: "kkkk".to_string(),
      };
      cmd.call(&ctx).unwrap();

      let loaded = store::read_task(&ctx.settings, &source.id).unwrap();

      assert_eq!(loaded.links[0].rel, RelationshipType::Blocks);

      let loaded_target = store::read_task(&ctx.settings, &target.id).unwrap();
      assert_eq!(loaded_target.links[0].rel, RelationshipType::BlockedBy);
      let _ = dir;
    }
  }

  fn setup() -> (TempDir, crate::cli::AppContext) {
    let dir = TempDir::new().unwrap();
    let ctx = make_test_context(dir.path());
    (dir, ctx)
  }
}

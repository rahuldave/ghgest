use clap::Args;

use crate::{
  action,
  cli::{self, AppContext},
  model::{Iteration, link::RelationshipType},
  store,
  ui::composites::success_message::SuccessMessage,
};

/// Create a relationship between an iteration and another entity.
#[derive(Debug, Args)]
pub struct Command {
  /// Target is an artifact instead of an iteration.
  #[arg(long)]
  pub artifact: bool,
  /// Iteration ID or unique prefix.
  pub id: String,
  /// Output the iteration as JSON after linking.
  #[arg(short, long, conflicts_with = "quiet")]
  pub json: bool,
  /// Output only the iteration ID.
  #[arg(short, long, conflicts_with = "json")]
  pub quiet: bool,
  /// Relationship type (e.g. blocks, blocked-by, relates-to).
  #[arg(value_enum)]
  pub rel: RelationshipType,
  /// Target iteration or artifact ID or unique prefix.
  pub target_id: String,
}

impl Command {
  /// Write the link to the source iteration; for iteration targets, also write the reciprocal.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    let config = &ctx.settings;
    let theme = &ctx.theme;

    let (id, target_id) = action::link::link::<Iteration>(config, &self.id, &self.target_id, &self.rel, self.artifact)?;

    if self.json {
      let iteration = store::read_iteration(config, &id)?;
      println!("{}", serde_json::to_string_pretty(&iteration)?);
    } else if self.quiet {
      println!("{id}");
    } else {
      let msg = format!("Linked {} --{}--\u{003e} {}", id, self.rel, target_id);
      println!("{}", SuccessMessage::new(&msg, theme));
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    store,
    test_helpers::{make_test_artifact, make_test_context, make_test_iteration},
  };

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_links_iteration_to_artifact() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());
      let source = make_test_iteration("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      let target = make_test_artifact("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
      store::write_iteration(&ctx.settings, &source).unwrap();
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

      let loaded = store::read_iteration(&ctx.settings, &source.id).unwrap();
      assert_eq!(loaded.links.len(), 1);
      assert_eq!(loaded.links[0].ref_, "artifacts/kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
    }

    #[test]
    fn it_links_iteration_to_iteration_with_reciprocal() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());
      let source = make_test_iteration("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      let target = make_test_iteration("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
      store::write_iteration(&ctx.settings, &source).unwrap();
      store::write_iteration(&ctx.settings, &target).unwrap();

      let cmd = Command {
        artifact: false,
        id: "zyxw".to_string(),
        json: false,
        quiet: false,
        rel: RelationshipType::RelatesTo,
        target_id: "kkkk".to_string(),
      };
      cmd.call(&ctx).unwrap();

      let loaded = store::read_iteration(&ctx.settings, &source.id).unwrap();
      assert_eq!(loaded.links.len(), 1);
      assert_eq!(loaded.links[0].rel, RelationshipType::RelatesTo);

      let loaded_target = store::read_iteration(&ctx.settings, &target.id).unwrap();
      assert_eq!(loaded_target.links.len(), 1);
      assert_eq!(loaded_target.links[0].rel, RelationshipType::RelatesTo);
    }
  }
}

use clap::Args;

use crate::{
  cli::{self, AppContext},
  store,
};

/// Add tags to an iteration.
#[derive(Debug, Args)]
pub struct Command {
  /// Iteration ID or unique prefix.
  pub id: String,
  /// Tags to add (space-separated).
  pub tags: Vec<String>,
}

impl Command {
  /// Merge the provided tags into the iteration's existing tag set, deduplicating.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    super::super::tags::tag_entity(
      ctx,
      &self.id,
      &self.tags,
      "iteration",
      store::resolve_iteration_id,
      store::read_iteration,
      |i| &mut i.tags,
      |i, t| i.updated_at = t,
      store::write_iteration,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helpers::{make_test_context, make_test_iteration};

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_tags() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());
      let iteration = make_test_iteration("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_iteration(&ctx.settings, &iteration).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        tags: vec!["sprint".to_string(), "q1".to_string()],
      };
      cmd.call(&ctx).unwrap();

      let loaded = store::read_iteration(&ctx.settings, &iteration.id).unwrap();
      assert_eq!(loaded.tags, vec!["sprint".to_string(), "q1".to_string()]);
    }

    #[test]
    fn it_deduplicates_tags() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());
      let mut iteration = make_test_iteration("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      iteration.tags = vec!["sprint".to_string()];
      store::write_iteration(&ctx.settings, &iteration).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        tags: vec!["sprint".to_string(), "q1".to_string()],
      };
      cmd.call(&ctx).unwrap();

      let loaded = store::read_iteration(&ctx.settings, &iteration.id).unwrap();
      assert_eq!(loaded.tags, vec!["sprint".to_string(), "q1".to_string()]);
    }
  }
}

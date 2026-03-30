use std::path::Path;

use clap::Args;

use crate::{
  cli, store,
  ui::{composites::success_message::SuccessMessage, theme::Theme},
};

/// Add an existing task to an iteration.
#[derive(Debug, Args)]
pub struct Command {
  /// Iteration ID or unique prefix.
  pub id: String,
  /// Task ID or unique prefix to add.
  pub task_id: String,
}

impl Command {
  /// Resolve both IDs, then append the task reference to the iteration.
  pub fn call(&self, data_dir: &Path, theme: &Theme) -> cli::Result<()> {
    let iteration_id = store::resolve_iteration_id(data_dir, &self.id, false)?;
    let task_id = store::resolve_task_id(data_dir, &self.task_id, true)?;

    let task_ref = format!("tasks/{task_id}");
    store::add_iteration_task(data_dir, &iteration_id, &task_ref)?;

    let msg = format!("Added task {} to iteration {}", task_id, iteration_id);
    println!("{}", SuccessMessage::new(&msg, theme));
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helpers::{make_test_config, make_test_iteration, make_test_task};

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_adds_a_task_to_iteration() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let iteration = make_test_iteration("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      let task = make_test_task("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
      store::write_iteration(&data_dir, &iteration).unwrap();
      store::write_task(&data_dir, &task).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        task_id: "kkkk".to_string(),
      };
      cmd.call(&data_dir, &Theme::default()).unwrap();

      let loaded = store::read_iteration(&data_dir, &iteration.id).unwrap();
      assert_eq!(loaded.tasks, vec!["tasks/kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk"]);
    }

    #[test]
    fn it_is_idempotent() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let iteration = make_test_iteration("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      let task = make_test_task("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
      store::write_iteration(&data_dir, &iteration).unwrap();
      store::write_task(&data_dir, &task).unwrap();

      let cmd = Command {
        id: "zyxw".to_string(),
        task_id: "kkkk".to_string(),
      };
      cmd.call(&data_dir, &Theme::default()).unwrap();
      cmd.call(&data_dir, &Theme::default()).unwrap();

      let loaded = store::read_iteration(&data_dir, &iteration.id).unwrap();
      assert_eq!(loaded.tasks.len(), 1);
    }
  }
}

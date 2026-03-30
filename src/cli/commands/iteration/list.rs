use std::path::Path;

use clap::Args;

use crate::{
  cli,
  model::{IterationFilter, iteration::Status},
  store,
  ui::{
    composites::empty_list::EmptyList,
    theme::Theme,
    views::iteration::{IterationListData, IterationListView},
  },
};

/// List iterations, optionally filtered by status or tag.
#[derive(Debug, Args)]
pub struct Command {
  /// Output iteration list as JSON.
  #[arg(short, long)]
  pub json: bool,
  /// Include resolved (completed/failed) iterations.
  #[arg(short = 'a', long = "all")]
  pub show_all: bool,
  /// Filter by status: active, completed, or failed.
  #[arg(short, long)]
  pub status: Option<String>,
  /// Filter by tag.
  #[arg(long)]
  pub tag: Option<String>,
}

impl Command {
  /// Query iterations from the store and render as a table or JSON.
  pub fn call(&self, data_dir: &Path, theme: &Theme) -> cli::Result<()> {
    let status = match &self.status {
      Some(s) => Some(s.parse::<Status>().map_err(cli::Error::generic)?),
      None => None,
    };

    let filter = IterationFilter {
      all: self.show_all,
      status,
      tag: self.tag.clone(),
    };

    let iterations = store::list_iterations(data_dir, &filter)?;

    if self.json {
      let json = serde_json::to_string_pretty(&iterations).map_err(|e| cli::Error::generic(e.to_string()))?;
      println!("{json}");
      return Ok(());
    }

    if iterations.is_empty() {
      println!("{}", EmptyList::new("iterations", theme));
      return Ok(());
    }

    let id_strings: Vec<String> = iterations.iter().map(|i| i.id.to_string()).collect();

    let view_data: Vec<IterationListData> = iterations
      .iter()
      .enumerate()
      .map(|(idx, i)| {
        let phase_count = compute_phase_count(&i.tasks);
        IterationListData {
          id: &id_strings[idx],
          title: &i.title,
          phase_count,
          task_count: i.tasks.len(),
        }
      })
      .collect();

    println!("{}", IterationListView::new(view_data, theme));

    Ok(())
  }
}

/// Compute how many distinct phases the iteration's tasks span.
fn compute_phase_count(_tasks: &[String]) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_helpers::{make_test_config, make_test_iteration};

  mod call {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_filters_by_status() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let i1 = make_test_iteration("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      let mut i2 = make_test_iteration("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk");
      i2.title = "Failed one".to_string();
      i2.status = crate::model::iteration::Status::Failed;
      store::write_iteration(&data_dir, &i1).unwrap();
      store::write_iteration(&data_dir, &i2).unwrap();

      let cmd = Command {
        show_all: false,
        json: false,
        status: Some("failed".to_string()),
        tag: None,
      };

      cmd.call(&data_dir, &Theme::default()).unwrap();

      let filter = IterationFilter {
        status: Some(Status::Failed),
        ..Default::default()
      };
      let iterations = store::list_iterations(&data_dir, &filter).unwrap();
      assert_eq!(iterations.len(), 1);
      assert_eq!(iterations[0].title, "Failed one");
    }

    #[test]
    fn it_handles_empty_list() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();

      let cmd = Command {
        show_all: false,
        json: false,
        status: None,
        tag: None,
      };

      cmd.call(&data_dir, &Theme::default()).unwrap();
    }

    #[test]
    fn it_lists_iterations() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let i1 = make_test_iteration("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_iteration(&data_dir, &i1).unwrap();

      let cmd = Command {
        show_all: false,
        json: false,
        status: None,
        tag: None,
      };

      cmd.call(&data_dir, &Theme::default()).unwrap();
    }

    #[test]
    fn it_outputs_json() {
      let dir = tempfile::tempdir().unwrap();
      let config = make_test_config(dir.path().to_path_buf());
      let data_dir = config.storage().data_dir(dir.path().to_path_buf()).unwrap();
      let iteration = make_test_iteration("zyxwvutsrqponmlkzyxwvutsrqponmlk");
      store::write_iteration(&data_dir, &iteration).unwrap();

      let cmd = Command {
        show_all: false,
        json: true,
        status: None,
        tag: None,
      };

      cmd.call(&data_dir, &Theme::default()).unwrap();
    }
  }
}

use clap::Args;
use yansi::Paint;

use crate::{
  config,
  config::Config,
  model::{
    IterationFilter,
    iteration::{Iteration, STATUS_ORDER, Status},
  },
  store,
  ui::{
    components::{EmptyList, Group, GroupedList},
    theme::Theme,
    utils::{format_id, format_iteration_status, format_tags, shortest_unique_prefixes},
  },
};

/// List iterations grouped by status, optionally filtered
#[derive(Debug, Args)]
pub struct Command {
  /// Output iteration list as JSON
  #[arg(short, long)]
  pub json: bool,
  /// Include resolved (completed/failed) iterations
  #[arg(short = 'a', long = "all")]
  pub show_all: bool,
  /// Filter by status: active, completed, or failed
  #[arg(short, long)]
  pub status: Option<String>,
  /// Filter by tag
  #[arg(long)]
  pub tag: Option<String>,
}

impl Command {
  pub fn call(&self, config: &Config, theme: &Theme) -> crate::Result<()> {
    let status = match &self.status {
      Some(s) => Some(s.parse::<Status>().map_err(crate::Error::generic)?),
      None => None,
    };

    let filter = IterationFilter {
      all: self.show_all,
      status,
      tag: self.tag.clone(),
    };

    let data_dir = config::data_dir(config)?;
    let iterations = store::list_iterations(&data_dir, &filter)?;

    if self.json {
      let json = serde_json::to_string_pretty(&iterations)?;
      println!("{json}");
      return Ok(());
    }

    if iterations.is_empty() {
      EmptyList::new("iterations").write_to(&mut std::io::stdout())?;
      return Ok(());
    }

    let all_ids: Vec<String> = iterations.iter().map(|i| i.id.to_string()).collect();
    let prefixes = shortest_unique_prefixes(&all_ids);
    let max_display = prefixes.iter().copied().max().unwrap_or(8);

    let groups: Vec<Group> = STATUS_ORDER
      .iter()
      .map(|status| {
        let rows: Vec<Vec<String>> = iterations
          .iter()
          .enumerate()
          .filter(|(_, i)| &i.status == status)
          .map(|(idx, i)| format_row(i, prefixes[idx], max_display, theme))
          .collect();
        Group::new(format_iteration_status(status, theme), rows)
      })
      .collect();

    GroupedList::new(groups, theme).write_to(&mut std::io::stdout())?;
    Ok(())
  }
}

fn format_row(iteration: &Iteration, prefix_len: usize, max_display: usize, theme: &Theme) -> Vec<String> {
  let id = format_id(&iteration.id, prefix_len, Some(max_display), theme);
  let title = iteration.title.paint(theme.md_heading).to_string();
  let task_count = format!("[{} tasks]", iteration.tasks.len())
    .paint(theme.muted)
    .to_string();

  let mut row = vec![id, title, task_count];

  if !iteration.tags.is_empty() {
    row.push(format_tags(&iteration.tags, theme));
  }

  row
}

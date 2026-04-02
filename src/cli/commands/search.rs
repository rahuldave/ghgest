use clap::Args;

use crate::{
  cli::{self, AppContext},
  config::Settings,
  store,
  ui::{
    composites::{artifact_list_row::ArtifactListRow, task_list_row::TaskListRow},
    theming::theme::Theme,
    views::search::{EntityType, SearchExpandedView, SearchResultItem, SearchView},
  },
};

/// Search across tasks and artifacts by keyword.
#[derive(Debug, Args)]
pub struct Command {
  /// Text matched against titles, descriptions, and body content.
  #[arg(long_help = "\
Text matched against titles, descriptions, and body content.

Supports filter prefixes that narrow results:

  is:<type>        Scope by entity type (task, artifact, iteration)
  tag:<name>       Filter by exact tag (case-insensitive)
  status:<status>  Filter by status (open, in-progress, done,
                    cancelled, active, completed, failed)
  type:<kind>      Filter artifacts by kind (spec, rfc, adr, ...)

Prefix any filter with - to negate it:

  -tag:wip         Exclude entities tagged \"wip\"
  -is:iteration    Exclude iterations from results

Combination rules:

  Same filter type OR-combines:   is:task is:artifact → tasks OR artifacts
  Different types AND-combine:    is:task tag:urgent  → tasks WITH tag \"urgent\"
  Free text AND-combines with filters

Examples:

  gest search \"login bug\"
  gest search \"is:task status:open\"
  gest search \"is:artifact type:spec\"
  gest search \"tag:urgent -tag:wip fix\"
  gest search \"is:task is:artifact tag:cli\"")]
  pub query: String,
  /// Show full detail for each result.
  #[arg(short, long)]
  pub expand: bool,
  /// Emit results as JSON.
  #[arg(short, long)]
  pub json: bool,
  /// Include archived and resolved items.
  #[arg(short = 'a', long = "all")]
  pub show_all: bool,
}

impl Command {
  /// Execute the search and render results to stdout.
  pub fn call(&self, ctx: &AppContext) -> cli::Result<()> {
    let results = store::search(&ctx.settings, &self.query, self.show_all)?;

    if self.json {
      let json_value = serde_json::json!({
        "query": self.query,
        "tasks": results.tasks,
        "artifacts": results.artifacts,
        "iterations": results.iterations,
      });
      let json = serde_json::to_string_pretty(&json_value)?;
      println!("{json}");
      return Ok(());
    }

    let items = build_search_items(&ctx.settings, &results, &ctx.theme, self.expand);

    let output = if self.expand {
      SearchExpandedView::new(&self.query, &items, &ctx.theme).to_string()
    } else {
      SearchView::new(&self.query, &items, &ctx.theme).to_string()
    };

    cli::pager::page(&output)?;

    Ok(())
  }
}

/// Convert raw search results into view-layer items with pre-rendered row content.
fn build_search_items(
  config: &Settings,
  results: &store::SearchResults,
  theme: &Theme,
  expand: bool,
) -> Vec<SearchResultItem> {
  let mut items = Vec::with_capacity(results.tasks.len() + results.artifacts.len() + results.iterations.len());

  for task in &results.tasks {
    let id_str = task.id.to_string();
    let status_str = task.status.as_str();

    let resolved = store::resolve_blocking(config, task);
    let blocked_by = resolved.blocked_by_ids.first().map(String::as_str);

    let row_content = TaskListRow::new(status_str, &id_str, &task.title, theme)
      .priority(task.priority)
      .tags(&task.tags)
      .blocking(resolved.is_blocking)
      .blocked_by(blocked_by)
      .to_string();

    let snippet = if task.description.is_empty() {
      None
    } else if expand {
      Some(task.description.clone())
    } else {
      Some(truncate_snippet(&task.description, 200))
    };

    items.push(SearchResultItem {
      entity_type: EntityType::Task,
      id: task.id.short(),
      row_content,
      snippet,
    });
  }

  for artifact in &results.artifacts {
    let id_str = artifact.id.to_string();
    let is_archived = artifact.archived_at.is_some();

    let row_content = ArtifactListRow::new(&id_str, &artifact.title, &artifact.tags, theme)
      .kind(artifact.kind.as_deref())
      .archived(is_archived)
      .to_string();

    let snippet = if artifact.body.is_empty() {
      None
    } else if expand {
      Some(artifact.body.clone())
    } else {
      Some(truncate_snippet(&artifact.body, 200))
    };

    items.push(SearchResultItem {
      entity_type: EntityType::Artifact,
      id: artifact.id.short(),
      row_content,
      snippet,
    });
  }

  for iteration in &results.iterations {
    let id_str = iteration.id.to_string();
    let status_str = iteration.status.as_str();

    let row_content = format!("{status_str}  {id_str}  {}", iteration.title);

    let snippet = if iteration.description.is_empty() {
      None
    } else if expand {
      Some(iteration.description.clone())
    } else {
      Some(truncate_snippet(&iteration.description, 200))
    };

    items.push(SearchResultItem {
      entity_type: EntityType::Iteration,
      id: iteration.id.short(),
      row_content,
      snippet,
    });
  }

  items
}

/// Take up to three lines of `text`, truncating at `max_chars` with an ellipsis if needed.
fn truncate_snippet(text: &str, max_chars: usize) -> String {
  let first_line_or_all = text.lines().take(3).collect::<Vec<_>>().join("\n");
  if first_line_or_all.chars().count() <= max_chars {
    first_line_or_all
  } else {
    let truncated: String = first_line_or_all.chars().take(max_chars).collect();
    format!("{truncated}...")
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    model::{Artifact, Task, task::Status},
    store,
    test_helpers::{make_test_artifact, make_test_context, make_test_task},
  };

  mod call {
    use super::*;

    #[test]
    fn it_handles_no_results() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());

      let cmd = Command {
        query: "nonexistent".to_string(),
        json: false,
        show_all: false,
        expand: false,
      };

      cmd.call(&ctx).unwrap();
    }

    #[test]
    fn it_includes_resolved_with_all_flag() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());

      let task = Task {
        title: "done task".to_string(),
        status: Status::Done,
        ..make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk")
      };
      store::write_task(&ctx.settings, &task).unwrap();

      let cmd = Command {
        query: "done".to_string(),
        json: false,
        show_all: true,
        expand: false,
      };

      cmd.call(&ctx).unwrap();
    }

    #[test]
    fn it_outputs_json() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());

      let task = Task {
        title: "json task".to_string(),
        status: Status::Open,
        ..make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk")
      };
      store::write_task(&ctx.settings, &task).unwrap();

      let cmd = Command {
        query: "json".to_string(),
        json: true,
        show_all: false,
        expand: false,
      };

      cmd.call(&ctx).unwrap();
    }

    #[test]
    fn it_renders_expanded_view() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());

      let task = Task {
        title: "expanded task".to_string(),
        description: "A longer description for expand mode".to_string(),
        status: Status::InProgress,
        ..make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk")
      };
      store::write_task(&ctx.settings, &task).unwrap();

      let cmd = Command {
        query: "expanded".to_string(),
        json: false,
        show_all: false,
        expand: true,
      };

      cmd.call(&ctx).unwrap();
    }

    #[test]
    fn it_returns_matching_artifacts() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());

      let artifact = Artifact {
        title: "schema design".to_string(),
        body: "Defines the canonical probe schema".to_string(),
        ..make_test_artifact("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk")
      };
      store::write_artifact(&ctx.settings, &artifact).unwrap();

      let cmd = Command {
        query: "schema".to_string(),
        json: false,
        show_all: false,
        expand: false,
      };

      cmd.call(&ctx).unwrap();
    }

    #[test]
    fn it_returns_matching_tasks() {
      let dir = tempfile::tempdir().unwrap();
      let ctx = make_test_context(dir.path());

      let task = Task {
        title: "streaming adapter".to_string(),
        status: Status::Open,
        ..make_test_task("zyxwvutsrqponmlkzyxwvutsrqponmlk")
      };
      store::write_task(&ctx.settings, &task).unwrap();

      let cmd = Command {
        query: "streaming".to_string(),
        json: false,
        show_all: false,
        expand: false,
      };

      cmd.call(&ctx).unwrap();
    }
  }

  mod truncate_snippet {
    use pretty_assertions::assert_eq;

    #[test]
    fn it_keeps_short_text() {
      let result = super::truncate_snippet("short text", 200);

      assert_eq!(result, "short text");
    }

    #[test]
    fn it_limits_to_three_lines() {
      let text = "line one\nline two\nline three\nline four\nline five";
      let result = super::truncate_snippet(text, 200);

      assert_eq!(result, "line one\nline two\nline three");
    }

    #[test]
    fn it_truncates_long_text() {
      let long_text = "a".repeat(300);
      let result = super::truncate_snippet(&long_text, 200);

      assert!(result.ends_with("..."));
      assert_eq!(result.chars().count(), 203);
    }
  }
}

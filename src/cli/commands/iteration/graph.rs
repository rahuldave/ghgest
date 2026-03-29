use std::collections::BTreeMap;

use clap::Args;
use yansi::Paint;

use crate::{
  config,
  config::Config,
  model::{Task, link::RelationshipType},
  store,
  ui::theme::Theme,
};

/// Display the phased execution graph for an iteration
#[derive(Debug, Args)]
pub struct Command {
  /// Iteration ID or unique prefix
  pub id: String,
  /// Output graph data as JSON
  #[arg(short, long)]
  pub json: bool,
}

impl Command {
  pub fn call(&self, config: &Config, theme: &Theme) -> crate::Result<()> {
    let data_dir = config::data_dir(config)?;
    let id = store::resolve_iteration_id(&data_dir, &self.id, true)?;
    let iteration = store::read_iteration(&data_dir, &id)?;

    // Resolve all task references to actual tasks
    let mut tasks: Vec<Task> = Vec::new();
    for task_ref in &iteration.tasks {
      let task_id_str = task_ref.strip_prefix("tasks/").unwrap_or(task_ref);
      match task_id_str.parse::<crate::model::Id>() {
        Ok(task_id) => match store::read_task(&data_dir, &task_id) {
          Ok(task) => tasks.push(task),
          Err(e) => {
            eprintln!("Warning: could not read task {}: {e}", task_id.short());
          }
        },
        Err(e) => {
          eprintln!("Warning: invalid task reference '{task_ref}': {e}");
        }
      }
    }

    if self.json {
      let graph = build_json_graph(&tasks);
      let json = serde_json::to_string_pretty(&graph)?;
      println!("{json}");
      return Ok(());
    }

    render_graph(&mut std::io::stdout(), &iteration.title, &tasks, theme)?;
    Ok(())
  }
}

fn build_json_graph(tasks: &[Task]) -> serde_json::Value {
  let mut phases: BTreeMap<u16, Vec<serde_json::Value>> = BTreeMap::new();
  let mut unphased: Vec<serde_json::Value> = Vec::new();

  for task in tasks {
    let entry = serde_json::json!({
      "id": task.id.short(),
      "title": task.title,
      "status": task.status.to_string(),
      "priority": task.priority,
      "assigned_to": task.assigned_to,
      "blocked_by": blocked_by_ids(task),
    });

    match task.phase {
      Some(phase) => phases.entry(phase).or_default().push(entry),
      None => unphased.push(entry),
    }
  }

  let mut result = serde_json::Map::new();
  let phase_entries: Vec<serde_json::Value> = phases
    .into_iter()
    .map(|(phase, tasks)| {
      serde_json::json!({
        "phase": phase,
        "tasks": tasks,
      })
    })
    .collect();

  result.insert("phases".to_string(), serde_json::Value::Array(phase_entries));
  if !unphased.is_empty() {
    result.insert("unphased".to_string(), serde_json::Value::Array(unphased));
  }

  serde_json::Value::Object(result)
}

fn blocked_by_ids(task: &Task) -> Vec<String> {
  task
    .links
    .iter()
    .filter(|l| l.rel == RelationshipType::BlockedBy)
    .map(|l| {
      let id_str = l.ref_.strip_prefix("tasks/").unwrap_or(&l.ref_);
      id_str[..8.min(id_str.len())].to_string()
    })
    .collect()
}

fn render_graph(w: &mut impl std::io::Write, title: &str, tasks: &[Task], theme: &Theme) -> std::io::Result<()> {
  writeln!(w, "{}", title.paint(theme.md_heading))?;
  writeln!(w)?;

  // Group tasks by phase
  let mut phases: BTreeMap<u16, Vec<&Task>> = BTreeMap::new();
  let mut unphased: Vec<&Task> = Vec::new();

  for task in tasks {
    match task.phase {
      Some(phase) => phases.entry(phase).or_default().push(task),
      None => unphased.push(task),
    }
  }

  // Sort tasks within each phase by priority (P0 first)
  for tasks in phases.values_mut() {
    tasks.sort_by_key(|t| t.priority.unwrap_or(u8::MAX));
  }
  unphased.sort_by_key(|t| t.priority.unwrap_or(u8::MAX));

  let total_phases = phases.len() + if unphased.is_empty() { 0 } else { 1 };
  let mut phase_idx = 0;

  for (phase_num, phase_tasks) in &phases {
    phase_idx += 1;
    let is_last = phase_idx == total_phases;
    render_phase(w, &format!("Phase {phase_num}"), phase_tasks, is_last, theme)?;
  }

  if !unphased.is_empty() {
    render_phase(w, "Unphased", &unphased, true, theme)?;
  }

  Ok(())
}

fn render_phase(
  w: &mut impl std::io::Write,
  label: &str,
  tasks: &[&Task],
  is_last: bool,
  theme: &Theme,
) -> std::io::Result<()> {
  writeln!(
    w,
    "{}  {}",
    "◆".paint(theme.md_heading),
    label.paint(theme.list_heading)
  )?;

  for (i, task) in tasks.iter().enumerate() {
    let is_last_task = i == tasks.len() - 1;
    let connector = if is_last_task { "└─" } else { "├─" };

    let status_glyph = match task.status {
      crate::model::task::Status::Done => "●",
      crate::model::task::Status::InProgress => "◐",
      crate::model::task::Status::Cancelled => "✗",
      crate::model::task::Status::Open => "○",
    };

    let mut parts = Vec::new();

    // Priority
    if let Some(p) = task.priority {
      parts.push(format!("[P{}]", p).paint(theme.muted).to_string());
    }

    // Short ID
    parts.push(task.id.short().paint(theme.id_prefix).to_string());

    // Title
    parts.push(format!("\"{}\"", task.title));

    let main = parts.join(" ");

    // Annotations
    let mut annotations = Vec::new();
    if let Some(ref assigned) = task.assigned_to {
      annotations.push(format!("assigned: {assigned}"));
    }
    let blockers = blocked_by_ids(task);
    if !blockers.is_empty() {
      annotations.push(format!("blocked-by: {}", blockers.join(", ")));
    }

    let annotation_str = if annotations.is_empty() {
      String::new()
    } else {
      format!("  {}", format!("({})", annotations.join(", ")).paint(theme.muted))
    };

    writeln!(w, "{connector} {status_glyph}  {main}{annotation_str}")?;
  }

  if !is_last {
    writeln!(w, "│")?;
  }

  Ok(())
}

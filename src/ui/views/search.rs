use std::fmt;

use crate::ui::{
  composites::search_result::{SearchResultExpanded, SearchResultRow, SearchSummary},
  theme::Theme,
};

/// The kind of entity a search result refers to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
  Artifact,
  Task,
}

impl EntityType {
  /// Returns the lowercase display label for this entity type.
  pub fn label(self) -> &'static str {
    match self {
      Self::Artifact => "artifact",
      Self::Task => "task",
    }
  }
}

/// Renders search results in expanded form, showing per-result separators and optional snippets.
pub struct SearchExpandedView<'a> {
  query: &'a str,
  results: &'a [SearchResultItem],
  theme: &'a Theme,
}

impl<'a> SearchExpandedView<'a> {
  pub fn new(query: &'a str, results: &'a [SearchResultItem], theme: &'a Theme) -> Self {
    Self {
      query,
      results,
      theme,
    }
  }
}

impl fmt::Display for SearchExpandedView<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let summary = SearchSummary::new(self.results.len(), self.query, self.theme);
    write!(f, "{summary}")?;

    if self.results.is_empty() {
      return Ok(());
    }

    for item in self.results {
      writeln!(f)?;
      writeln!(f)?;
      let expanded = SearchResultExpanded::new(item.entity_type.label(), &item.id, &item.row_content, self.theme)
        .snippet(item.snippet.as_deref());
      write!(f, "{expanded}")?;
    }

    Ok(())
  }
}

/// A single search hit, carrying its entity type, id, formatted row, and optional body snippet.
#[derive(Debug, Clone)]
pub struct SearchResultItem {
  pub entity_type: EntityType,
  pub id: String,
  /// Pre-formatted content for the compact row display.
  pub row_content: String,
  /// Optional body excerpt shown only in expanded view.
  pub snippet: Option<String>,
}

/// Renders search results in compact form as a flat list of rows.
pub struct SearchView<'a> {
  query: &'a str,
  results: &'a [SearchResultItem],
  theme: &'a Theme,
}

impl<'a> SearchView<'a> {
  pub fn new(query: &'a str, results: &'a [SearchResultItem], theme: &'a Theme) -> Self {
    Self {
      query,
      results,
      theme,
    }
  }
}

impl fmt::Display for SearchView<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let summary = SearchSummary::new(self.results.len(), self.query, self.theme);
    write!(f, "{summary}")?;

    if self.results.is_empty() {
      return Ok(());
    }

    writeln!(f)?;
    for item in self.results {
      writeln!(f)?;
      let row = SearchResultRow::new(item.entity_type.label(), &item.row_content, self.theme);
      write!(f, "{row}")?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn theme() -> Theme {
    Theme::default()
  }

  fn render(item: &impl fmt::Display) -> String {
    yansi::disable();
    let out = item.to_string();
    yansi::enable();
    out
  }

  fn sample_items() -> Vec<SearchResultItem> {
    vec![
      SearchResultItem {
        entity_type: EntityType::Task,
        id: "nfkbqmrx".into(),
        row_content: "nfkbqmrx  openai streaming adapter".into(),
        snippet: Some("Implements the streaming adapter for OpenAI.".into()),
      },
      SearchResultItem {
        entity_type: EntityType::Artifact,
        id: "fsahdqlt".into(),
        row_content: "fsahdqlt  probe-schema-v2".into(),
        snippet: None,
      },
    ]
  }

  #[test]
  fn it_renders_compact_view_summary_and_rows() {
    let theme = theme();
    let items = sample_items();
    let view = SearchView::new("streaming", &items, &theme);
    let out = render(&view);

    assert!(out.contains("2 results for"), "should show count");
    assert!(out.contains("\"streaming\""), "should show query");
    assert!(out.contains("task"), "should contain task type label");
    assert!(out.contains("artifact"), "should contain artifact type label");
    assert!(
      out.contains("nfkbqmrx  openai streaming adapter"),
      "should contain first row content"
    );
    assert!(
      out.contains("fsahdqlt  probe-schema-v2"),
      "should contain second row content"
    );
  }

  #[test]
  fn it_renders_compact_view_with_empty_results() {
    let theme = theme();
    let items: Vec<SearchResultItem> = vec![];
    let view = SearchView::new("auth", &items, &theme);
    let out = render(&view);

    assert!(out.contains("0 results for"), "should show zero count");
    assert!(out.contains("try broadening your query"), "should show hint");
    assert!(!out.contains("task"), "no rows for empty results");
  }

  #[test]
  fn it_renders_compact_view_with_single_result() {
    let theme = theme();
    let items = vec![SearchResultItem {
      entity_type: EntityType::Task,
      id: "abcdefgh".into(),
      row_content: "abcdefgh  single task".into(),
      snippet: None,
    }];
    let view = SearchView::new("single", &items, &theme);
    let out = render(&view);

    assert!(out.contains("1 result for"), "singular form");
    assert!(out.contains("abcdefgh  single task"), "row content present");
  }

  #[test]
  fn it_renders_expanded_view_item_without_snippet() {
    let theme = theme();
    let items = vec![SearchResultItem {
      entity_type: EntityType::Artifact,
      id: "fsahdqlt".into(),
      row_content: "fsahdqlt  probe-schema-v2".into(),
      snippet: None,
    }];
    let view = SearchExpandedView::new("schema", &items, &theme);
    let out = render(&view);

    assert!(out.contains("artifact fsahdqlt"), "separator label present");
    assert!(out.contains("fsahdqlt  probe-schema-v2"), "row content present");
  }

  #[test]
  fn it_renders_expanded_view_summary_and_items() {
    let theme = theme();
    let items = sample_items();
    let view = SearchExpandedView::new("streaming", &items, &theme);
    let out = render(&view);

    assert!(out.contains("2 results for"), "should show count");
    assert!(out.contains("\"streaming\""), "should show query");
    assert!(out.contains("task nfkbqmrx"), "should contain task separator label");
    assert!(
      out.contains("artifact fsahdqlt"),
      "should contain artifact separator label"
    );
    assert!(out.contains('\u{254C}'), "should use dashed character");
    assert!(
      out.contains("nfkbqmrx  openai streaming adapter"),
      "should contain first row content"
    );
    assert!(
      out.contains("Implements the streaming adapter"),
      "should contain snippet"
    );
  }

  #[test]
  fn it_renders_expanded_view_with_empty_results() {
    let theme = theme();
    let items: Vec<SearchResultItem> = vec![];
    let view = SearchExpandedView::new("auth", &items, &theme);
    let out = render(&view);

    assert!(out.contains("0 results for"), "should show zero count");
    assert!(out.contains("try broadening your query"), "should show hint");
  }

  #[test]
  fn it_renders_expanded_view_with_multiline_snippet() {
    let theme = theme();
    let items = vec![SearchResultItem {
      entity_type: EntityType::Task,
      id: "abcdefgh".into(),
      row_content: "abcdefgh  multi-line task".into(),
      snippet: Some("Line one.\nLine two.".into()),
    }];
    let view = SearchExpandedView::new("multi", &items, &theme);
    let out = render(&view);

    assert!(out.contains("  Line one."), "first snippet line indented");
    assert!(out.contains("  Line two."), "second snippet line indented");
  }

  #[test]
  fn it_returns_correct_entity_type_labels() {
    assert_eq!(EntityType::Task.label(), "task");
    assert_eq!(EntityType::Artifact.label(), "artifact");
  }
}

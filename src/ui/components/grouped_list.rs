use std::{fmt, io};

use yansi::Paint;

use crate::ui::{theme::Theme, utils::display_width};

/// A group of rows under a heading for use in a [`GroupedList`].
pub struct Group {
  pub heading: String,
  pub rows: Vec<Vec<String>>,
}

impl Group {
  pub fn new(heading: impl Into<String>, rows: Vec<Vec<String>>) -> Self {
    Self {
      heading: heading.into(),
      rows,
    }
  }
}

/// A grouped, column-aligned plain-text list.
///
/// Each group has a styled heading and rows of pre-formatted column values.
/// Empty groups are skipped. Groups are separated by a blank line.
pub struct GroupedList {
  groups: Vec<Group>,
  theme: Theme,
}

impl GroupedList {
  pub fn new(groups: Vec<Group>, theme: &Theme) -> Self {
    Self {
      groups,
      theme: theme.clone(),
    }
  }

  /// Write the grouped list to the given writer.
  pub fn write_to(&self, w: &mut impl io::Write) -> io::Result<()> {
    let mut first = true;

    for group in &self.groups {
      if group.rows.is_empty() {
        continue;
      }

      if !first {
        writeln!(w)?;
      }
      first = false;

      // Write styled heading followed by a blank line
      writeln!(w, "{}", group.heading.paint(self.theme.list_heading))?;
      writeln!(w)?;

      // Calculate column widths using ANSI-aware measurement
      let col_count = group.rows.iter().map(|r| r.len()).max().unwrap_or(0);
      let mut widths = vec![0usize; col_count];
      for row in &group.rows {
        for (i, cell) in row.iter().enumerate() {
          if i < widths.len() {
            widths[i] = widths[i].max(display_width(cell));
          }
        }
      }

      // Write rows with aligned columns
      for row in &group.rows {
        let mut line = String::new();
        for (i, cell) in row.iter().enumerate() {
          if i > 0 {
            line.push_str("   ");
          }
          let visible = display_width(cell);
          let target = widths.get(i).copied().unwrap_or(visible);
          line.push_str(cell);
          if i < row.len() - 1 {
            // Pad to align columns (account for ANSI sequences)
            let pad = target.saturating_sub(visible);
            for _ in 0..pad {
              line.push(' ');
            }
          }
        }
        writeln!(w, "  {line}")?;
      }
    }

    Ok(())
  }
}

impl fmt::Display for GroupedList {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut buf = Vec::new();
    self.write_to(&mut buf).map_err(|_| fmt::Error)?;
    let s = String::from_utf8(buf).map_err(|_| fmt::Error)?;
    // Display should not include trailing newline since println! adds one
    write!(f, "{}", s.trim_end())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod grouped_list {
    use super::*;

    mod display {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_matches_write_to_output() {
        let groups = vec![Group::new("Open", vec![vec!["abc".to_string(), "A task".to_string()]])];
        let theme = Theme::default();
        let list = GroupedList::new(groups, &theme);
        let display = list.to_string();
        let mut buf = Vec::new();
        list.write_to(&mut buf).unwrap();
        let write_output = String::from_utf8(buf).unwrap();
        assert_eq!(display, write_output.trim_end());
      }
    }

    mod write_to {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_renders_a_single_group() {
        let groups = vec![Group::new(
          "Open",
          vec![
            vec!["abc".to_string(), "First task".to_string()],
            vec!["def".to_string(), "Second task".to_string()],
          ],
        )];
        let theme = Theme::default();
        let list = GroupedList::new(groups, &theme);
        let mut buf = Vec::new();
        list.write_to(&mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();

        assert!(output.contains("Open"), "Should contain group heading");
        assert!(output.contains("abc"), "Should contain first row ID");
        assert!(output.contains("First task"), "Should contain first row title");
        assert!(output.contains("def"), "Should contain second row ID");
        assert!(output.contains("Second task"), "Should contain second row title");

        // Heading should be the first line (stripping ANSI)
        let first_line = output.lines().next().unwrap();
        assert_eq!(display_width(first_line), 4, "Heading visible width should be 'Open'");
      }

      #[test]
      fn it_separates_multiple_groups_with_blank_line() {
        let groups = vec![
          Group::new("Open", vec![vec!["abc".to_string(), "Task A".to_string()]]),
          Group::new("In Progress", vec![vec!["def".to_string(), "Task B".to_string()]]),
        ];
        let theme = Theme::default();
        let list = GroupedList::new(groups, &theme);
        let mut buf = Vec::new();
        list.write_to(&mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();

        // There should be a blank line between the two groups
        assert!(output.contains("\n\n"), "Groups should be separated by a blank line");

        assert!(output.contains("Open"), "Should contain first heading");
        assert!(output.contains("In Progress"), "Should contain second heading");
        assert!(output.contains("abc"), "Should contain first group data");
        assert!(output.contains("def"), "Should contain second group data");
      }

      #[test]
      fn it_hides_empty_groups() {
        let groups = vec![
          Group::new("Ideas", Vec::new()),
          Group::new("Open", vec![vec!["abc".to_string(), "Task A".to_string()]]),
          Group::new("Done", Vec::new()),
        ];
        let theme = Theme::default();
        let list = GroupedList::new(groups, &theme);
        let mut buf = Vec::new();
        list.write_to(&mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();

        assert!(
          !output.contains("Ideas"),
          "Should not contain empty group heading 'Ideas'"
        );
        assert!(
          !output.contains("Done"),
          "Should not contain empty group heading 'Done'"
        );
        assert!(output.contains("Open"), "Should contain non-empty group heading");
        assert!(output.contains("abc"), "Should contain row data");
      }

      #[test]
      fn it_aligns_columns_with_ansi_styled_text() {
        // Simulate ANSI-styled text in columns: red "abc" vs plain "defghi"
        let styled_short = "\x1b[31mabc\x1b[0m".to_string(); // visible width 3
        let plain_long = "defghi".to_string(); // visible width 6

        let groups = vec![Group::new(
          "Test",
          vec![
            vec![styled_short.clone(), "First".to_string()],
            vec![plain_long.clone(), "Second".to_string()],
          ],
        )];
        let theme = Theme::default();
        let list = GroupedList::new(groups, &theme);
        let mut buf = Vec::new();
        list.write_to(&mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();

        // Find the two data lines (skip the heading and blank line)
        let lines: Vec<&str> = output.lines().collect();
        // lines[0] = heading, lines[1] = blank, lines[2] = first row, lines[3] = second row
        assert!(
          lines.len() >= 4,
          "Should have heading + blank + 2 rows, got {}",
          lines.len()
        );

        // The "First" and "Second" text should start at the same column
        // First row: indent (2) + styled_short (visible 3) + padding (3) + "   " (3 space separator) + "First"
        // Second row: indent (2) + plain_long (visible 6) + "   " (3 space separator) + "Second"
        // Both second columns should start at the same visual position
        let row1 = lines[2];
        let row2 = lines[3];

        // Find where "First" and "Second" start visually by measuring up to the separator
        // The first column should be padded to width 6 in both cases
        let row1_parts: Vec<&str> = row1.splitn(2, "First").collect();
        let row2_parts: Vec<&str> = row2.splitn(2, "Second").collect();

        let row1_prefix_width = display_width(row1_parts[0]);
        let row2_prefix_width = display_width(row2_parts[0]);

        assert_eq!(
          row1_prefix_width, row2_prefix_width,
          "Second column should start at same visual position (row1 prefix: {}, row2 prefix: {})",
          row1_prefix_width, row2_prefix_width
        );
      }

      #[test]
      fn it_produces_no_output_when_all_groups_empty() {
        let groups = vec![Group::new("Ideas", Vec::new()), Group::new("Done", Vec::new())];
        let theme = Theme::default();
        let list = GroupedList::new(groups, &theme);
        let mut buf = Vec::new();
        list.write_to(&mut buf).unwrap();
        let output = String::from_utf8(buf).unwrap();
        assert!(output.is_empty(), "Should produce no output when all groups are empty");
      }
    }
  }
}

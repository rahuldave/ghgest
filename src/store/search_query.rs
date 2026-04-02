//! Parsed representation of a search query string with structured filters.
//!
//! Filter syntax:
//! - `is:<type>` — filter by entity type (artifact, iteration, task)
//! - `tag:<name>` — filter by tag
//! - `status:<status>` — filter by status
//! - `type:<kind>` — filter by artifact kind
//! - `-<filter>` — negate any filter (e.g. `-tag:wip`)
//!
//! Same filter types OR-combine; different filter types AND-combine.
//! Unknown prefixes are treated as free text.

/// A single filter extracted from the query.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Filter {
  Is(String),
  Status(String),
  Tag(String),
  Type(String),
}

/// The result of parsing a search query string.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParsedQuery {
  pub exclude: Vec<Filter>,
  pub include: Vec<Filter>,
  pub text: String,
}

impl ParsedQuery {
  /// Returns `true` when the query has no filters and no text.
  #[allow(dead_code)]
  pub fn is_empty(&self) -> bool {
    self.text.is_empty() && self.include.is_empty() && self.exclude.is_empty()
  }
}

/// Known filter prefixes (case-insensitive matching is handled by lowercasing the prefix).
const KNOWN_PREFIXES: &[&str] = &["is", "status", "tag", "type"];

/// Parse a search query string into a [`ParsedQuery`].
pub fn parse(query: &str) -> ParsedQuery {
  let mut include = Vec::new();
  let mut exclude = Vec::new();
  let mut text_parts: Vec<&str> = Vec::new();

  for token in query.split_whitespace() {
    let (negated, body) = if let Some(rest) = token.strip_prefix('-') {
      // A bare `-` or `-` followed by no colon is just free text.
      if rest.is_empty() || !rest.contains(':') {
        text_parts.push(token);
        continue;
      }
      (true, rest)
    } else {
      (false, token)
    };

    if let Some((prefix, value)) = body.split_once(':') {
      let prefix_lower = prefix.to_lowercase();
      if KNOWN_PREFIXES.contains(&prefix_lower.as_str()) && !value.is_empty() {
        let value_lower = value.to_lowercase();
        let filter = match prefix_lower.as_str() {
          "is" => Filter::Is(value_lower),
          "status" => Filter::Status(value_lower),
          "tag" => Filter::Tag(value_lower),
          "type" => Filter::Type(value_lower),
          _ => unreachable!(),
        };
        if negated {
          exclude.push(filter);
        } else {
          include.push(filter);
        }
        continue;
      }
    }

    // Unknown prefix or no colon — treat as free text.
    text_parts.push(token);
  }

  ParsedQuery {
    exclude,
    include,
    text: text_parts.join(" "),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod parse {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn it_collects_exclude_filters_with_negation_prefix() {
      let q = parse("-tag:wip -status:done");

      assert_eq!(
        q.exclude,
        vec![Filter::Tag("wip".into()), Filter::Status("done".into())]
      );
      assert!(q.include.is_empty());
      assert!(q.text.is_empty());
    }

    #[test]
    fn it_collects_multiple_same_type_filters_for_or_semantics() {
      let q = parse("is:artifact is:task");

      assert_eq!(
        q.include,
        vec![Filter::Is("artifact".into()), Filter::Is("task".into())]
      );
      assert!(q.text.is_empty());
    }

    #[test]
    fn it_combines_filters_and_free_text() {
      let q = parse("is:task tag:urgent fix login bug");

      assert_eq!(q.include, vec![Filter::Is("task".into()), Filter::Tag("urgent".into())]);
      assert_eq!(q.text, "fix login bug");
    }

    #[test]
    fn it_handles_empty_query() {
      let q = parse("");

      assert!(q.is_empty());
    }

    #[test]
    fn it_handles_whitespace_only_query() {
      let q = parse("   ");

      assert!(q.is_empty());
    }

    #[test]
    fn it_is_case_insensitive_for_prefixes() {
      let q = parse("IS:task TAG:Urgent");

      assert_eq!(q.include, vec![Filter::Is("task".into()), Filter::Tag("urgent".into())]);
    }

    #[test]
    fn it_lowercases_filter_values() {
      let q = parse("status:InProgress type:RFC");

      assert_eq!(
        q.include,
        vec![Filter::Status("inprogress".into()), Filter::Type("rfc".into())]
      );
    }

    #[test]
    fn it_parses_all_supported_filter_types() {
      let q = parse("is:artifact tag:foo status:open type:spec");

      assert_eq!(
        q.include,
        vec![
          Filter::Is("artifact".into()),
          Filter::Tag("foo".into()),
          Filter::Status("open".into()),
          Filter::Type("spec".into()),
        ]
      );
    }

    #[test]
    fn it_parses_plain_text_query() {
      let q = parse("hello world");

      assert!(q.include.is_empty());
      assert!(q.exclude.is_empty());
      assert_eq!(q.text, "hello world");
    }

    #[test]
    fn it_treats_bare_dash_as_free_text() {
      let q = parse("- foo");

      assert_eq!(q.text, "- foo");
    }

    #[test]
    fn it_treats_dash_without_colon_as_free_text() {
      let q = parse("-nocolon foo");

      assert_eq!(q.text, "-nocolon foo");
    }

    #[test]
    fn it_treats_empty_value_filter_as_free_text() {
      let q = parse("tag: foo");

      assert_eq!(q.text, "tag: foo");
      assert!(q.include.is_empty());
    }

    #[test]
    fn it_treats_unknown_prefixes_as_free_text() {
      let q = parse("foo:bar baz:qux hello");

      assert!(q.include.is_empty());
      assert_eq!(q.text, "foo:bar baz:qux hello");
    }
  }

  mod parsed_query {
    use super::*;

    #[test]
    fn it_is_empty_when_default() {
      let q = ParsedQuery::default();

      assert!(q.is_empty());
    }

    #[test]
    fn it_is_not_empty_with_text() {
      let q = ParsedQuery {
        text: "hello".into(),
        ..Default::default()
      };

      assert!(!q.is_empty());
    }

    #[test]
    fn it_is_not_empty_with_filters() {
      let q = ParsedQuery {
        include: vec![Filter::Tag("foo".into())],
        ..Default::default()
      };

      assert!(!q.is_empty());
    }
  }
}

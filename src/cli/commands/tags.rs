/// Add tags to a tag list, skipping duplicates.
pub fn apply_tags(tags: &mut Vec<String>, to_add: &[String]) {
  for tag in to_add {
    if !tags.contains(tag) {
      tags.push(tag.clone());
    }
  }
}

/// Remove tags from a tag list.
pub fn remove_tags(tags: &mut Vec<String>, to_remove: &[String]) {
  tags.retain(|t| !to_remove.contains(t));
}

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;

  use super::*;

  #[test]
  fn apply_tags_adds_new() {
    let mut tags = vec!["a".to_string()];
    apply_tags(&mut tags, &["b".to_string(), "c".to_string()]);
    assert_eq!(tags, vec!["a", "b", "c"]);
  }

  #[test]
  fn apply_tags_skips_duplicates() {
    let mut tags = vec!["a".to_string(), "b".to_string()];
    apply_tags(&mut tags, &["b".to_string(), "c".to_string()]);
    assert_eq!(tags, vec!["a", "b", "c"]);
  }

  #[test]
  fn remove_tags_filters() {
    let mut tags = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    remove_tags(&mut tags, &["b".to_string()]);
    assert_eq!(tags, vec!["a", "c"]);
  }

  #[test]
  fn remove_tags_ignores_absent() {
    let mut tags = vec!["a".to_string()];
    remove_tags(&mut tags, &["z".to_string()]);
    assert_eq!(tags, vec!["a"]);
  }
}

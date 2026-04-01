use std::{
  fmt::{self, Display, Formatter},
  str::FromStr,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::id::Id;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum AuthorType {
  #[serde(rename = "agent")]
  Agent,
  #[default]
  #[serde(rename = "human")]
  Human,
}

impl AuthorType {
  pub fn as_str(&self) -> &'static str {
    match self {
      Self::Agent => "agent",
      Self::Human => "human",
    }
  }
}

impl Display for AuthorType {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.write_str(self.as_str())
  }
}

impl FromStr for AuthorType {
  type Err = String;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    match s {
      "agent" => Ok(Self::Agent),
      "human" => Ok(Self::Human),
      other => Err(format!("unknown author type: {other}")),
    }
  }
}

#[derive(Clone, Debug, Default)]
pub struct NewNote {
  pub author: String,
  pub author_email: Option<String>,
  pub author_type: AuthorType,
  pub body: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Note {
  pub author: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub author_email: Option<String>,
  #[serde(default)]
  pub author_type: AuthorType,
  pub body: String,
  pub created_at: DateTime<Utc>,
  pub id: Id,
  pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Default)]
pub struct NotePatch {
  pub body: Option<String>,
}

#[cfg(test)]
mod tests {
  use super::*;

  mod author_type {
    use super::*;

    mod display {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_formats_as_lowercase() {
        assert_eq!(AuthorType::Agent.to_string(), "agent");
        assert_eq!(AuthorType::Human.to_string(), "human");
      }
    }

    mod from_str {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_parses_valid_types() {
        assert_eq!("agent".parse::<AuthorType>().unwrap(), AuthorType::Agent);
        assert_eq!("human".parse::<AuthorType>().unwrap(), AuthorType::Human);
      }

      #[test]
      fn it_rejects_unknown() {
        assert!("invalid".parse::<AuthorType>().is_err());
      }
    }
  }

  mod note {
    use super::*;

    mod roundtrip {
      use pretty_assertions::assert_eq;

      use super::*;

      #[test]
      fn it_omits_author_email_when_none() {
        let now = Utc::now();
        let note = Note {
          author: "claude".to_string(),
          author_email: None,
          author_type: AuthorType::Agent,
          body: "test".to_string(),
          created_at: now,
          id: "zyxwvutsrqponmlkzyxwvutsrqponmlk".parse().unwrap(),
          updated_at: now,
        };

        let toml_str = toml::to_string(&note).unwrap();
        assert!(!toml_str.contains("author_email"));
      }

      #[test]
      fn it_roundtrips_agent_note_through_toml() {
        let now = Utc::now();
        let note = Note {
          author: "claude".to_string(),
          author_email: None,
          author_type: AuthorType::Agent,
          body: "Agent observation".to_string(),
          created_at: now,
          id: "zyxwvutsrqponmlkzyxwvutsrqponmlk".parse().unwrap(),
          updated_at: now,
        };

        let toml_str = toml::to_string(&note).unwrap();
        let roundtripped: Note = toml::from_str(&toml_str).unwrap();
        assert_eq!(note, roundtripped);
      }

      #[test]
      fn it_roundtrips_through_toml() {
        let now = Utc::now();
        let note = Note {
          author: "alice".to_string(),
          author_email: Some("alice@example.com".to_string()),
          author_type: AuthorType::Human,
          body: "This is a note".to_string(),
          created_at: now,
          id: "zyxwvutsrqponmlkzyxwvutsrqponmlk".parse().unwrap(),
          updated_at: now,
        };

        let toml_str = toml::to_string(&note).unwrap();
        let roundtripped: Note = toml::from_str(&toml_str).unwrap();
        assert_eq!(note, roundtripped);
      }
    }
  }
}

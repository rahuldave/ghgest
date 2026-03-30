//! Serde helpers for `Option<DateTime<Utc>>` fields that serialize `None` as an empty string.

use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(value: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  match value {
    Some(dt) => serializer.serialize_str(&dt.to_rfc3339()),
    None => serializer.serialize_str(""),
  }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  if s.is_empty() {
    Ok(None)
  } else {
    DateTime::parse_from_rfc3339(&s)
      .map(|dt| Some(dt.with_timezone(&Utc)))
      .map_err(serde::de::Error::custom)
  }
}

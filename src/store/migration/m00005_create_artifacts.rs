use super::Migration;

pub const MIGRATION: Migration = Migration {
  name: "create_artifacts",
  sql: "\
    CREATE TABLE artifacts (\
      id          TEXT PRIMARY KEY,\
      project_id  TEXT NOT NULL REFERENCES projects(id),\
      archived_at TEXT,\
      body        TEXT NOT NULL DEFAULT '',\
      created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),\
      metadata    TEXT NOT NULL DEFAULT '{}',\
      title       TEXT NOT NULL,\
      updated_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))\
    );\
    CREATE INDEX idx_artifacts_project_id ON artifacts (project_id);\
  ",
  version: 5,
};

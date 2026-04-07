use super::Migration;

pub const MIGRATION: Migration = Migration {
  name: "create_tasks",
  sql: "\
    CREATE TABLE tasks (\
      id          TEXT PRIMARY KEY,\
      project_id  TEXT NOT NULL REFERENCES projects(id),\
      assigned_to TEXT REFERENCES authors(id),\
      created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),\
      description TEXT NOT NULL DEFAULT '',\
      metadata    TEXT NOT NULL DEFAULT '{}',\
      priority    INTEGER,\
      resolved_at TEXT,\
      status      TEXT NOT NULL DEFAULT 'open',\
      title       TEXT NOT NULL,\
      updated_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))\
    );\
    CREATE INDEX idx_tasks_project_id ON tasks (project_id);\
    CREATE INDEX idx_tasks_status ON tasks (project_id, status);\
  ",
  version: 4,
};

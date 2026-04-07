use super::Migration;

pub const MIGRATION: Migration = Migration {
  name: "add_transaction_author",
  sql: "\
    ALTER TABLE transactions ADD COLUMN author_id TEXT REFERENCES authors(id);\
  ",
  version: 16,
};

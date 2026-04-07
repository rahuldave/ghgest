//! Integration tests organized by command. Each subdirectory covers one command family;
//! files within follow the `when_<behavior>.rs` pattern with `it_<does_something>` test
//! functions.

mod artifact;
mod config;
mod generate;
mod init;
mod iteration;
mod project;
mod search;
mod tag;
mod task;
mod undo;
mod version;

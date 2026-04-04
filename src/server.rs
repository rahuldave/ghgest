//! Built-in web server for browsing gest entities in a browser.

mod assets;
mod handlers;
mod request_log;
mod routes;
mod security_headers;
mod state;
mod templates;
pub mod watcher;

pub use routes::router;
pub use state::ServerState;

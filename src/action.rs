//! Action abstraction layer providing entity traits for generic operations.

pub mod link;
mod set_status;
mod tag;
mod traits;
mod untag;

pub use set_status::set_status;
pub use tag::tag;
pub use traits::{HasStatus, Linkable, Resolvable, Storable, Taggable};
pub use untag::untag;

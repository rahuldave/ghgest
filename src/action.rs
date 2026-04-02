//! Action abstraction layer providing entity traits for generic operations.

mod tag;
mod traits;
mod untag;

pub use tag::tag;
pub use traits::{HasStatus, Linkable, Resolvable, Storable, Taggable};
pub use untag::untag;

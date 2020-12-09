
mod vstore;
mod index;
mod error;
mod commit;
mod tree;
pub mod diff;

pub use vstore::VStore;
pub use error::Error;
pub use tree::{Tree, ImmutableTree};

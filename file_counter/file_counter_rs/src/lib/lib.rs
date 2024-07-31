#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate ifile_counter.
//!
//!

pub use files::files;
pub use predicates::{build_glob_match_fn, build_predicate_contains_fn, build_predicate_ext_fn};
pub use types::PredicatePathFn;

mod files;
mod predicates;
mod types;

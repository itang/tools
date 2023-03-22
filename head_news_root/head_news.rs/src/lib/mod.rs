#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! head_news lib.

mod sina;
mod types;

pub use sina::Sina;
pub use types::*;

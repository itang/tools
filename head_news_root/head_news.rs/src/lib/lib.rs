#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
//#![feature(async_fn_in_trait)]

//! head_news lib.

pub use sina::Sina;

pub use types::{News, Portal};

mod sina;
mod types;

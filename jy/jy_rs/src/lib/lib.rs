#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! jy.
//!
//! jy lib.
//!

mod application;
mod domain;
mod infrastructure;

pub use self::application::{run, show_info, RunOptions};

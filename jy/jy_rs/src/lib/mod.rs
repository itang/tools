#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! jy.
//!
//! jy lib.
//!

mod app;
mod browser;
mod config;
mod constants;

pub use self::app::{run, show_info, RunOptions};

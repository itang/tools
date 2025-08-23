//! rdict lib
//!
#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(warnings)]

mod dict;
mod util;

pub use dict::{DictResult, PronResult, TranResult, dict};

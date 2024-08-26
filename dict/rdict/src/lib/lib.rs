//! rdict lib
//!
#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(warnings)]

pub use dict::{dict, DictResult, PronResult, TranResult};

mod dict;
mod util;

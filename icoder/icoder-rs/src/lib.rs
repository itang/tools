#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate icoder.
//!
//! add doc here

pub use coder::{base64::Base64, hex::Hex, i2hex::I2Hex, now, random_str, uuid, Coder, CoderResult};
pub use read_line::ReadLine;

mod coder;
mod read_line;

#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate icoder.
//!
//! add doc here

pub use coder::{CoderResult, Coder, hex::Hex, i2hex::I2Hex, base64::Base64, uuid, random_str};
pub use read_line::{ReadLine};

mod coder;
mod read_line;

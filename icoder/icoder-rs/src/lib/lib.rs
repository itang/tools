#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate icoder.
//!
//! add doc here

pub use coder::{base64::Base64, hex::Hex, i2hex::I2Hex, i2binary::I2Binary,
                md5, now, random_str, uuid, Coder, CoderResult, pretty_print_url};
pub use read_line::ReadLine;

mod coder;
mod read_line;

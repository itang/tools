#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate icoder.
//!
//! add doc here

mod coder;
mod read_line;

pub use coder::{
    base64::Base64, hex::Hex, i2binary::I2Binary, i2hex::I2Hex, md5, now, random_str, split_string_whitespace,
    unum::UNum, url, uuid, Coder, CoderResult,
};

pub use read_line::ReadLine;

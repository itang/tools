#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate httpin-rs.
//!
//! add doc here

/// greet.
pub fn greet(name: &str) -> String {
    format!("Hello, {name}")
}

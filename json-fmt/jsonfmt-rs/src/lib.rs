#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate jsonfmt-rs.
//!
//! add doc here

use serde_json::Value;

/// format json string pretty.
pub fn fmt_json_string_pretty(s: &str) -> anyhow::Result<String> {
    let value: Value = serde_json::from_str(s)?;
    let json = serde_json::to_string_pretty(&value)?;

    Ok(json)
}

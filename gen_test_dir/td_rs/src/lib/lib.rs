#![deny(clippy::unwrap_used)]
//#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![feature(provide_any)]
#![feature(error_generic_member_access)]

//! td lib.

mod dir;
mod macros;
mod state;

pub use dir::{gen_dir_str, DirCreate, DirCreateError};
pub use state::SILENT as SILENT_MODE;

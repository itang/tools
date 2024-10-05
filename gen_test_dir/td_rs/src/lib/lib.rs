#![deny(clippy::unwrap_used)]
//#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![feature(error_generic_member_access)]

//! td lib.

mod dir;
mod macros;
mod state;

pub use dir::{gen_dir, DirCreate, DirCreateError};
pub use state::{get_silent_mode, init_silent_mode};

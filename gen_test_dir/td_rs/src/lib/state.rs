//! state module.

use once_cell::sync::OnceCell;

/// silent mode.
pub static SILENT: OnceCell<bool> = OnceCell::new();

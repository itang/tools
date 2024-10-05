//! state module.

//use once_cell::sync::OnceCell;

use std::sync::OnceLock;

/// silent mode.
static SILENT: OnceLock<bool> = OnceLock::new();

/// init silent mode.
pub fn init_silent_mode(silent: bool) {
    SILENT.set(silent).expect("set value");
}

/// get silent mode.
pub fn get_silent_mode() -> &'static bool {
    SILENT.get().expect("get value")
}

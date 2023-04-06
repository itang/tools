//! state module.

use once_cell::sync::OnceCell;

/// silent mode.
static SILENT: OnceCell<bool> = OnceCell::new();

/// init silent mode.
pub fn init_silent_mode(silent: bool) {
    SILENT.set(silent).expect("set value");
}

/// get silent mode.
pub fn get_silent_mode() -> &'static bool {
    SILENT.get().expect("get value")
}

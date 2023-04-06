//! macros module.

/// log info.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use colored::Colorize;
        if !td::SILENT_MODE.get().expect("get") {
            let res = std::fmt::format(std::format_args!($($arg)*));
            println!("{}: {}", "INFO".blue(), res.green());
        }
    }}
}

/// log warn.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        use colored::Colorize;
        if !td::SILENT_MODE.get().expect("get") {
            let res = std::fmt::format(std::format_args!($($arg)*));
            println!("{}: {}", "WARN".yellow(), res.red());
        }
    }}
}

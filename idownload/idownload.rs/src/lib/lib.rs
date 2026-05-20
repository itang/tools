#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! # Idownload Crate
//!
//! `idownload` is a simple file download tool with proxy support, progress display, and retry mechanism.

pub mod app;

mod downloader;

pub use downloader::Downloader;

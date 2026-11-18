//! Windows process scan implemented entirely with Win32 FFI: no child
//! process, no PowerShell, so nothing depends on the nsh process allow-list.
//!
//! Only `Scanner` leaves the crate; everything else is internal, one module
//! per concern:
//!
//! - [`sys`]: the Win32 declarations, handles and debug privilege.
//! - [`net`]: the TCP/UDP tables as owned sockets with printable addresses.
//! - [`process`]: process identity, image path, lifetime, CPU and memory.
//! - [`scan`]: joining the two, sampling CPU, and the row formats the app's
//!   parser expects (`pid|name|ppid|path|tcp|udp|started|cpu_x10|mem_kb|addrs`).

mod net;
mod process;
mod scan;
mod sys;

use nsh_core::prelude::*;

#[native_module("portradar/native")]
fn native(registry: &mut Registry) {
    registry.module::<scan::Scanner>();
}

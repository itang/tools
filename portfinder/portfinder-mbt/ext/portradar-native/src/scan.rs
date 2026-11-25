//! The scan itself: join sockets to processes, sample CPU over a real window,
//! and serialize the result into the two line formats the MoonBit side reads.
//!
//! Process rows: `pid|name|ppid|path|tcp|udp|started|cpu_x10|mem_kb|addrs`
//! Port rows:    `name|pid|proto|state|addr`

use std::collections::{BTreeMap, BTreeSet};
use std::thread::sleep;
use std::time::{Duration, Instant};

use nsh_core::prelude::*;

use crate::net::{self, Socket};
use crate::process::{self, ProcInfo, ProcStats};
use crate::sys;

/// Native process scan, exported to TSX.
#[module]
pub(crate) struct Scanner;

#[module]
impl Scanner {
    /// Lists processes behind listening TCP / bound UDP endpoints.
    /// Returns newline-separated `pid|name|ppid|path|tcp|udp|started|cpu_x10|mem_kb|addrs` rows.
    /// Runs on a blocking thread so the UI can keep rendering progress.
    pub(crate) async fn scan_processes() -> String {
        blocking(process_rows).await.unwrap_or_default()
    }

    /// Processes owning `port`, as `name|pid|proto|state|addr` rows.
    pub(crate) async fn scan_port(port: u32) -> String {
        blocking(move || port_rows(port)).await.unwrap_or_default()
    }

    /// Ends a process with Win32 `TerminateProcess`; true on success.
    pub(crate) fn kill_process(pid: u32) -> bool {
        process::terminate(pid)
    }
}

/// Every process holding a socket, one row each, ordered by first port.
fn process_rows() -> String {
    // Best effort: when the host runs elevated this unlocks metrics for
    // SYSTEM-owned services; otherwise those rows are reported as limited.
    sys::enable_debug_privilege();
    let procs = process::processes();
    let mut tcp: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    let mut udp: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    let mut addrs: BTreeMap<u32, BTreeSet<String>> = BTreeMap::new();

    for family in net::FAMILIES {
        for socket in net::tcp_sockets(family) {
            tcp.entry(socket.pid).or_default().insert(socket.port);
            addrs
                .entry(socket.pid)
                .or_default()
                .insert(format!("TCP {}", socket.endpoint()));
        }
        for socket in net::udp_sockets(family) {
            udp.entry(socket.pid).or_default().insert(socket.port);
            addrs
                .entry(socket.pid)
                .or_default()
                .insert(format!("UDP {}", socket.endpoint()));
        }
    }

    let pids: BTreeSet<u32> = tcp.keys().chain(udp.keys()).copied().collect();

    // CPU: two snapshots separated by a real sampling window. A short sleep
    // gives the delta meaning even when metadata queries return quickly.
    let first: BTreeMap<u32, u64> = pids
        .iter()
        .filter_map(|&pid| process::process_stats(pid).map(|s| (pid, s.cpu_ticks)))
        .collect();
    let started_at = Instant::now();
    sleep(Duration::from_millis(250));
    let stats: BTreeMap<u32, ProcStats> = pids
        .iter()
        .filter_map(|&pid| process::process_stats(pid).map(|s| (pid, s)))
        .collect();
    let elapsed = started_at.elapsed().as_secs_f64().max(0.05);

    let cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1) as f64;

    let mut rows: Vec<(u32, String)> = Vec::new();
    for pid in pids {
        let info = procs.get(&pid);
        let name = info.map(|p| p.name.clone()).unwrap_or_default();
        let ppid = info.map(|p| p.ppid.to_string()).unwrap_or_default();
        let path = process::process_path(pid);
        let tcp_ports = join_ports(tcp.get(&pid));
        let udp_ports = join_ports(udp.get(&pid));
        let addr_list = addrs
            .get(&pid)
            .map(|s| s.iter().cloned().collect::<Vec<_>>().join(";"))
            .unwrap_or_default();
        let st = stats.get(&pid);
        let started = st.map(|s| s.started.to_string()).unwrap_or_default();
        // "!" marks a visible process whose handle could not be opened, so the
        // UI can say "受限" instead of showing an ambiguous blank.
        let mem_kb = match st {
            Some(s) if s.mem_kb > 0 => s.mem_kb.to_string(),
            Some(_) => String::new(),
            None => "!".to_string(),
        };
        let cpu_x10 = match (st, first.get(&pid)) {
            (Some(s), Some(&before)) => {
                let delta = s.cpu_ticks.saturating_sub(before) as f64;
                let pct = (delta / (cores * elapsed * 10_000_000.0) * 100.0).min(100.0);
                (pct * 10.0).round().to_string()
            }
            _ => "!".to_string(),
        };
        let line = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            pid, name, ppid, path, tcp_ports, udp_ports, started, cpu_x10, mem_kb, addr_list
        );
        let first_port = tcp
            .get(&pid)
            .and_then(|s| s.iter().next())
            .or_else(|| udp.get(&pid).and_then(|s| s.iter().next()))
            .copied()
            .unwrap_or(0);
        rows.push((first_port, line));
    }
    rows.sort_by_key(|(port, _)| *port);
    rows.into_iter()
        .map(|(_, line)| line)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Rows for one queried port, in the `name|pid|proto|state|addr` format the
/// app's `parse_entries` reads. TCP listeners and UDP bindings of both
/// families come straight from the Win32 tables; no child process is involved.
fn port_rows(port: u32) -> String {
    let procs = process::processes();
    let mut lines: Vec<String> = Vec::new();
    for family in net::FAMILIES {
        for socket in net::tcp_sockets(family) {
            if socket.port == port {
                lines.push(entry_line(&procs, &socket, "TCP|Listen"));
            }
        }
        for socket in net::udp_sockets(family) {
            if socket.port == port {
                lines.push(entry_line(&procs, &socket, "UDP|"));
            }
        }
    }
    lines.join("\n")
}

/// One `name|pid|proto|state|addr` line. `proto_state` carries the two
/// middle fields, which differ only by protocol.
fn entry_line(procs: &BTreeMap<u32, ProcInfo>, socket: &Socket, proto_state: &str) -> String {
    let name = procs
        .get(&socket.pid)
        .map(|p| p.name.clone())
        .unwrap_or_default();
    format!(
        "{}|{}|{}|{}",
        name,
        socket.pid,
        proto_state,
        socket.endpoint()
    )
}

fn join_ports(set: Option<&BTreeSet<u32>>) -> String {
    set.map(|s| {
        s.iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(",")
    })
    .unwrap_or_default()
}

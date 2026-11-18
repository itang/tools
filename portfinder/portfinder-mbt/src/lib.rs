//! Windows process scan implemented entirely with Win32 FFI: no child
//! process, no PowerShell, so nothing depends on the nsh process allow-list.
//! Emits rows in the exact format the app's parser expects:
//!   pid|name|ppid|path|tcp|udp|started|cpu_x10|mem_kb|addrs

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::c_void;
use std::net::Ipv6Addr;
use std::ptr::null_mut;
use std::thread::sleep;
use std::time::{Duration, Instant};

use nsh_core::prelude::*;

const AF_INET: u32 = 2;
const AF_INET6: u32 = 23;
const TCP_TABLE_OWNER_PID_ALL: u32 = 5;
const UDP_TABLE_OWNER_PID: u32 = 1;
const ERROR_INSUFFICIENT_BUFFER: u32 = 122;
const MIB_TCP_STATE_LISTEN: u32 = 2;
const TH32CS_SNAPPROCESS: u32 = 0x2;
const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
const PROCESS_TERMINATE: u32 = 0x0001;
const TOKEN_ADJUST_PRIVILEGES: u32 = 0x0002;
const TOKEN_QUERY: u32 = 0x0008;
const SE_PRIVILEGE_ENABLED: u32 = 0x0002;
const ERROR_NOT_ALL_ASSIGNED: u32 = 1300;
const INVALID_HANDLE: isize = -1;

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct MibTcpRowOwnerPid {
    state: u32,
    local_addr: u32,
    local_port: u32,
    remote_addr: u32,
    remote_port: u32,
    owning_pid: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct MibTcp6RowOwnerPid {
    local_addr: [u8; 16],
    local_scope_id: u32,
    local_port: u32,
    remote_addr: [u8; 16],
    remote_scope_id: u32,
    remote_port: u32,
    state: u32,
    owning_pid: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct MibUdpRowOwnerPid {
    local_addr: u32,
    local_port: u32,
    owning_pid: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct MibUdp6RowOwnerPid {
    local_addr: [u8; 16],
    local_scope_id: u32,
    local_port: u32,
    owning_pid: u32,
}

// PROCESSENTRY32 (ANSI): szExeFile is a 260-byte char array.
#[repr(C)]
#[derive(Clone, Copy)]
struct ProcessEntry32A {
    dw_size: u32,
    cnt_usage: u32,
    th32_process_id: u32,
    th32_default_heap_id: usize,
    th32_module_id: u32,
    cnt_threads: u32,
    th32_parent_process_id: u32,
    pc_pri_class_base: i32,
    dw_flags: u32,
    sz_exe_file: [i8; 260],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct FileTime {
    low: u32,
    high: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct ProcessMemoryCounters {
    cb: u32,
    page_fault_count: u32,
    peak_working_set_size: usize,
    working_set_size: usize,
    quota_peak_paged_pool_usage: usize,
    quota_paged_pool_usage: usize,
    quota_peak_non_paged_pool_usage: usize,
    quota_non_paged_pool_usage: usize,
    page_file_usage: usize,
    peak_page_file_usage: usize,
    private_page_count: usize,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct Luid {
    low: u32,
    high: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct LuidAndAttributes {
    luid: Luid,
    attributes: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TokenPrivileges {
    count: u32,
    privileges: [LuidAndAttributes; 1],
}

#[link(name = "iphlpapi")]
unsafe extern "system" {
    fn GetExtendedTcpTable(
        table: *mut c_void,
        size: *mut u32,
        order: i32,
        af: u32,
        class: u32,
        reserved: u32,
    ) -> u32;
    fn GetExtendedUdpTable(
        table: *mut c_void,
        size: *mut u32,
        order: i32,
        af: u32,
        class: u32,
        reserved: u32,
    ) -> u32;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn CreateToolhelp32Snapshot(flags: u32, process_id: u32) -> isize;
    fn Process32First(snapshot: isize, entry: *mut ProcessEntry32A) -> i32;
    fn Process32Next(snapshot: isize, entry: *mut ProcessEntry32A) -> i32;
    fn CloseHandle(handle: isize) -> i32;
    fn OpenProcess(desired_access: u32, inherit: i32, pid: u32) -> isize;
    fn QueryFullProcessImageNameW(handle: isize, flags: u32, name: *mut u16, size: *mut u32)
        -> i32;
    fn GetProcessTimes(
        handle: isize,
        create: *mut FileTime,
        exit: *mut FileTime,
        kernel: *mut FileTime,
        user: *mut FileTime,
    ) -> i32;
    fn TerminateProcess(handle: isize, exit_code: u32) -> i32;
    fn GetCurrentProcess() -> isize;
    fn SetLastError(error: u32);
    fn GetLastError() -> u32;
}

#[link(name = "advapi32")]
unsafe extern "system" {
    fn OpenProcessToken(process: isize, desired_access: u32, token: *mut isize) -> i32;
    fn LookupPrivilegeValueW(system: *const u16, name: *const u16, luid: *mut Luid) -> i32;
    fn AdjustTokenPrivileges(
        token: isize,
        disable_all: i32,
        new_state: *const TokenPrivileges,
        buffer_length: u32,
        previous: *mut TokenPrivileges,
        return_length: *mut u32,
    ) -> i32;
}

#[link(name = "psapi")]
unsafe extern "system" {
    fn GetProcessMemoryInfo(handle: isize, counters: *mut ProcessMemoryCounters, cb: u32) -> i32;
}

// Reads a growable Win32 table (TCP or UDP) into a Vec of rows.
unsafe fn read_table<T: Copy>(get: unsafe extern "system" fn(*mut c_void, *mut u32, i32, u32, u32, u32) -> u32, af: u32, class: u32) -> Vec<T> {
    let mut size: u32 = 0;
    let rc = get(null_mut(), &mut size, 0, af, class, 0);
    if rc != ERROR_INSUFFICIENT_BUFFER {
        return Vec::new();
    }
    let mut buf = vec![0u64; (size as usize + 7) / 8];
    let mut actual = size;
    let rc = get(buf.as_mut_ptr() as *mut c_void, &mut actual, 0, af, class, 0);
    if rc != 0 {
        return Vec::new();
    }
    let count = *(buf.as_ptr() as *const u32) as usize;
    let rows = (buf.as_ptr() as *const u8).add(4) as *const T;
    (0..count).map(|i| *rows.add(i)).collect()
}

fn tcp_rows(af: u32) -> Vec<(u32, u32, String)> {
    unsafe {
        if af == AF_INET {
            read_table::<MibTcpRowOwnerPid>(GetExtendedTcpTable, af, TCP_TABLE_OWNER_PID_ALL)
                .into_iter()
                .filter(|r| r.state == MIB_TCP_STATE_LISTEN)
                .map(|r| (r.owning_pid, r.local_port, ipv4(r.local_addr)))
                .collect()
        } else {
            read_table::<MibTcp6RowOwnerPid>(GetExtendedTcpTable, af, TCP_TABLE_OWNER_PID_ALL)
                .into_iter()
                .filter(|r| r.state == MIB_TCP_STATE_LISTEN)
                .map(|r| {
                    (
                        r.owning_pid,
                        r.local_port,
                        Ipv6Addr::from(r.local_addr).to_string(),
                    )
                })
                .collect()
        }
    }
}

fn udp_rows(af: u32) -> Vec<(u32, u32, String)> {
    unsafe {
        if af == AF_INET {
            read_table::<MibUdpRowOwnerPid>(GetExtendedUdpTable, af, UDP_TABLE_OWNER_PID)
                .into_iter()
                .map(|r| (r.owning_pid, r.local_port, ipv4(r.local_addr)))
                .collect()
        } else {
            read_table::<MibUdp6RowOwnerPid>(GetExtendedUdpTable, af, UDP_TABLE_OWNER_PID)
                .into_iter()
                .map(|r| {
                    (
                        r.owning_pid,
                        r.local_port,
                        Ipv6Addr::from(r.local_addr).to_string(),
                    )
                })
                .collect()
        }
    }
}

fn net_order_port(raw: u32) -> u32 {
    ((raw & 0xFF) << 8) | ((raw >> 8) & 0xFF)
}

fn ipv4(raw: u32) -> String {
    format!(
        "{}.{}.{}.{}",
        raw & 0xFF,
        (raw >> 8) & 0xFF,
        (raw >> 16) & 0xFF,
        (raw >> 24) & 0xFF
    )
}

fn c_string(bytes: &[i8]) -> String {
    let mut data = Vec::with_capacity(bytes.len());
    for &b in bytes {
        if b == 0 {
            break;
        }
        data.push(b as u8);
    }
    String::from_utf8_lossy(&data).into_owned()
}

struct ProcInfo {
    name: String,
    ppid: u32,
}

fn processes() -> BTreeMap<u32, ProcInfo> {
    let mut out = BTreeMap::new();
    unsafe {
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snap == INVALID_HANDLE {
            return out;
        }
        let mut entry: ProcessEntry32A = std::mem::zeroed();
        entry.dw_size = std::mem::size_of::<ProcessEntry32A>() as u32;
        if Process32First(snap, &mut entry) != 0 {
            loop {
                out.insert(
                    entry.th32_process_id,
                    ProcInfo {
                        name: c_string(&entry.sz_exe_file),
                        ppid: entry.th32_parent_process_id,
                    },
                );
                if Process32Next(snap, &mut entry) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snap);
    }
    out
}

fn process_path(pid: u32) -> String {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if handle == 0 || handle == INVALID_HANDLE {
            return String::new();
        }
        let mut buf = [0u16; 32768];
        let mut len = buf.len() as u32;
        let ok = QueryFullProcessImageNameW(handle, 0, buf.as_mut_ptr(), &mut len);
        CloseHandle(handle);
        if ok == 0 {
            return String::new();
        }
        String::from_utf16_lossy(&buf[..len as usize])
    }
}

#[derive(Clone, Copy)]
struct ProcStats {
    started: i64,
    cpu_ticks: u64,
    mem_kb: i64,
}

fn filetime_value(ft: &FileTime) -> u64 {
    ((ft.high as u64) << 32) | (ft.low as u64)
}

fn filetime_unix(ticks: u64) -> i64 {
    (ticks / 10_000_000) as i64 - 11_644_473_600
}

fn process_stats(pid: u32) -> Option<ProcStats> {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if handle == 0 || handle == INVALID_HANDLE {
            return None;
        }
        let mut create: FileTime = std::mem::zeroed();
        let mut exit: FileTime = std::mem::zeroed();
        let mut kernel: FileTime = std::mem::zeroed();
        let mut user: FileTime = std::mem::zeroed();
        if GetProcessTimes(handle, &mut create, &mut exit, &mut kernel, &mut user) == 0 {
            CloseHandle(handle);
            return None;
        }
        let mut counters: ProcessMemoryCounters = std::mem::zeroed();
        counters.cb = std::mem::size_of::<ProcessMemoryCounters>() as u32;
        let mem_ok = GetProcessMemoryInfo(handle, &mut counters, counters.cb);
        CloseHandle(handle);
        Some(ProcStats {
            started: filetime_unix(filetime_value(&create)),
            cpu_ticks: filetime_value(&kernel) + filetime_value(&user),
            mem_kb: if mem_ok != 0 {
                (counters.working_set_size / 1024) as i64
            } else {
                -1
            },
        })
    }
}

// Enables SeDebugPrivilege on the current process so OpenProcess can reach
// SYSTEM-owned services (most svchost.exe). AdjustTokenPrivileges returns TRUE
// even when the caller is not elevated; ERROR_NOT_ALL_ASSIGNED is the honest
// signal, so it is checked explicitly. Kernel/protected processes (System,
// PPL) stay unreadable even with this privilege and are reported as limited.
fn enable_debug_privilege() -> bool {
    unsafe {
        let mut token: isize = 0;
        if OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
            &mut token,
        ) == 0
        {
            return false;
        }
        let name: Vec<u16> = "SeDebugPrivilege\0".encode_utf16().collect();
        let mut luid: Luid = std::mem::zeroed();
        let mut ok = LookupPrivilegeValueW(std::ptr::null(), name.as_ptr(), &mut luid) != 0;
        if ok {
            let state = TokenPrivileges {
                count: 1,
                privileges: [LuidAndAttributes {
                    luid,
                    attributes: SE_PRIVILEGE_ENABLED,
                }],
            };
            SetLastError(0);
            ok = AdjustTokenPrivileges(
                token,
                0,
                &state,
                std::mem::size_of::<TokenPrivileges>() as u32,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ) != 0
                && GetLastError() != ERROR_NOT_ALL_ASSIGNED;
        }
        CloseHandle(token);
        ok
    }
}

/// Native process scan, exported to TSX.
#[module]
pub struct Scanner;

#[module]
impl Scanner {
    /// Lists processes behind listening TCP / bound UDP endpoints.
    /// Returns newline-separated `pid|name|ppid|path|tcp|udp|started|cpu_x10|mem_kb|addrs` rows.
    /// Runs on a blocking thread so the UI can keep rendering progress.
    pub async fn scan_processes() -> String {
        blocking(scan).await.unwrap_or_default()
    }

    /// Processes owning `port`, as `name|pid|proto|state|addr` rows.
    pub async fn scan_port(port: u32) -> String {
        blocking(move || port_rows(port)).await.unwrap_or_default()
    }

    /// Ends a process with Win32 `TerminateProcess`; true on success.
    pub fn kill_process(pid: u32) -> bool {
        terminate(pid)
    }
}

// Rows for one queried port, in the `name|pid|proto|state|addr` format the
// app's `parse_entries` reads. TCP listeners and UDP bindings of both families
// are scanned directly from the Win32 tables; no child process is involved.
fn port_rows(port: u32) -> String {
    let procs = processes();
    let mut lines: Vec<String> = Vec::new();
    for af in [AF_INET, AF_INET6] {
        for (pid, raw_port, addr) in tcp_rows(af) {
            if net_order_port(raw_port) != port {
                continue;
            }
            let name = procs.get(&pid).map(|p| p.name.clone()).unwrap_or_default();
            let shown = if addr.contains(':') {
                format!("[{}]:{}", addr, port)
            } else {
                format!("{}:{}", addr, port)
            };
            lines.push(format!("{}|{}|TCP|Listen|{}", name, pid, shown));
        }
        for (pid, raw_port, addr) in udp_rows(af) {
            if net_order_port(raw_port) != port {
                continue;
            }
            let name = procs.get(&pid).map(|p| p.name.clone()).unwrap_or_default();
            let shown = if addr.contains(':') {
                format!("[{}]:{}", addr, port)
            } else {
                format!("{}:{}", addr, port)
            };
            lines.push(format!("{}|{}|UDP||{}", name, pid, shown));
        }
    }
    lines.join("\n")
}

// Forcefully ends a process. SYSTEM / protected processes cannot be opened
// with TERMINATE access even by an elevated caller, so failure is an honest
// `false` rather than a panic.
fn terminate(pid: u32) -> bool {
    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if handle == 0 || handle == INVALID_HANDLE {
            return false;
        }
        let ok = TerminateProcess(handle, 1);
        CloseHandle(handle);
        ok != 0
    }
}

fn scan() -> String {
    // Best effort: when the host runs elevated this unlocks metrics for
    // SYSTEM-owned services; otherwise those rows are reported as limited.
    enable_debug_privilege();
    let procs = processes();
    let mut tcp: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    let mut udp: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    let mut addrs: BTreeMap<u32, BTreeSet<String>> = BTreeMap::new();

    for af in [AF_INET, AF_INET6] {
        for (pid, raw_port, addr) in tcp_rows(af) {
            let port = net_order_port(raw_port);
            tcp.entry(pid).or_default().insert(port);
            let shown = if addr.contains(':') {
                format!("[{}]:{}", addr, port)
            } else {
                format!("{}:{}", addr, port)
            };
            addrs.entry(pid).or_default().insert(format!("TCP {shown}"));
        }
        for (pid, raw_port, addr) in udp_rows(af) {
            let port = net_order_port(raw_port);
            udp.entry(pid).or_default().insert(port);
            let shown = if addr.contains(':') {
                format!("[{}]:{}", addr, port)
            } else {
                format!("{}:{}", addr, port)
            };
            addrs.entry(pid).or_default().insert(format!("UDP {shown}"));
        }
    }

    let pids: BTreeSet<u32> = tcp.keys().chain(udp.keys()).copied().collect();

    // CPU: two snapshots separated by a real sampling window. A short sleep
    // gives the delta meaning even when metadata queries return quickly.
    let cpu1: BTreeMap<u32, u64> = pids
        .iter()
        .filter_map(|&pid| process_stats(pid).map(|s| (pid, s.cpu_ticks)))
        .collect();
    let started_at = Instant::now();
    sleep(Duration::from_millis(250));
    let stats: BTreeMap<u32, ProcStats> = pids
        .iter()
        .filter_map(|&pid| process_stats(pid).map(|s| (pid, s)))
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
        let path = process_path(pid);
        let tcp_ports = join_ports(tcp.get(&pid));
        let udp_ports = join_ports(udp.get(&pid));
        let addr_list = addrs
            .get(&pid)
            .map(|s| s.iter().cloned().collect::<Vec<_>>().join(";"))
            .unwrap_or_default();
        let st = stats.get(&pid);
        let started = st
            .map(|s| s.started.to_string())
            .unwrap_or_default();
        // "!" marks a visible process whose handle could not be opened, so the
        // UI can say "受限" instead of showing an ambiguous blank.
        let mem_kb = match st {
            Some(s) if s.mem_kb > 0 => s.mem_kb.to_string(),
            Some(_) => String::new(),
            None => "!".to_string(),
        };
        let cpu_x10 = match (st, cpu1.get(&pid)) {
            (Some(s), Some(&first)) => {
                let delta = s.cpu_ticks.saturating_sub(first) as f64;
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

fn join_ports(set: Option<&BTreeSet<u32>>) -> String {
    set.map(|s| s.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(","))
        .unwrap_or_default()
}

#[native_module("portradar/native")]
fn native(registry: &mut Registry) {
    registry.module::<Scanner>();
}

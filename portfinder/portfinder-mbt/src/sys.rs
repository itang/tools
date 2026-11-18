//! The Win32 surface: constants, `repr(C)` structures and the `extern`
//! declarations the scan is built on, plus the two Win32 chores that keep
//! handles honest (open one, close it once) and debug privilege enabled.
//! No other module declares FFI, so every `unsafe` above this layer is a
//! call into it rather than a second, drifting copy of a prototype.

use std::ffi::c_void;
use std::ptr::null_mut;

// Address families and table classes the iphlpapi calls take.
pub(crate) const AF_INET: u32 = 2;
pub(crate) const AF_INET6: u32 = 23;
pub(crate) const TCP_TABLE_OWNER_PID_ALL: u32 = 5;
pub(crate) const UDP_TABLE_OWNER_PID: u32 = 1;
pub(crate) const MIB_TCP_STATE_LISTEN: u32 = 2;
const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

// Process and token access rights.
pub(crate) const TH32CS_SNAPPROCESS: u32 = 0x2;
pub(crate) const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
pub(crate) const PROCESS_TERMINATE: u32 = 0x0001;
const TOKEN_ADJUST_PRIVILEGES: u32 = 0x0002;
const TOKEN_QUERY: u32 = 0x0008;
const SE_PRIVILEGE_ENABLED: u32 = 0x0002;
const ERROR_NOT_ALL_ASSIGNED: u32 = 1300;
const INVALID_HANDLE: isize = -1;

// The row structures of the tables. The fields nothing reads are still part
// of the layout, so they stay: what Windows writes is not ours to shrink.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct MibTcpRowOwnerPid {
    pub(crate) state: u32,
    pub(crate) local_addr: u32,
    pub(crate) local_port: u32,
    pub(crate) remote_addr: u32,
    pub(crate) remote_port: u32,
    pub(crate) owning_pid: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct MibTcp6RowOwnerPid {
    pub(crate) local_addr: [u8; 16],
    pub(crate) local_scope_id: u32,
    pub(crate) local_port: u32,
    pub(crate) remote_addr: [u8; 16],
    pub(crate) remote_scope_id: u32,
    pub(crate) remote_port: u32,
    pub(crate) state: u32,
    pub(crate) owning_pid: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct MibUdpRowOwnerPid {
    pub(crate) local_addr: u32,
    pub(crate) local_port: u32,
    pub(crate) owning_pid: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct MibUdp6RowOwnerPid {
    pub(crate) local_addr: [u8; 16],
    pub(crate) local_scope_id: u32,
    pub(crate) local_port: u32,
    pub(crate) owning_pid: u32,
}

// PROCESSENTRY32 (ANSI): szExeFile is a 260-byte char array.
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct ProcessEntry32A {
    pub(crate) dw_size: u32,
    pub(crate) cnt_usage: u32,
    pub(crate) th32_process_id: u32,
    pub(crate) th32_default_heap_id: usize,
    pub(crate) th32_module_id: u32,
    pub(crate) cnt_threads: u32,
    pub(crate) th32_parent_process_id: u32,
    pub(crate) pc_pri_class_base: i32,
    pub(crate) dw_flags: u32,
    pub(crate) sz_exe_file: [i8; 260],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct FileTime {
    pub(crate) low: u32,
    pub(crate) high: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct ProcessMemoryCounters {
    pub(crate) cb: u32,
    page_fault_count: u32,
    peak_working_set_size: usize,
    pub(crate) working_set_size: usize,
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

/// Either `GetExtendedTcpTable` or `GetExtendedUdpTable`: same parameters,
/// same "size first, then fill" contract.
pub(crate) type TableCall =
    unsafe extern "system" fn(*mut c_void, *mut u32, i32, u32, u32, u32) -> u32;

#[link(name = "iphlpapi")]
unsafe extern "system" {
    pub(crate) fn GetExtendedTcpTable(
        table: *mut c_void,
        size: *mut u32,
        order: i32,
        af: u32,
        class: u32,
        reserved: u32,
    ) -> u32;
    pub(crate) fn GetExtendedUdpTable(
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
    pub(crate) fn CreateToolhelp32Snapshot(flags: u32, process_id: u32) -> isize;
    pub(crate) fn Process32First(snapshot: isize, entry: *mut ProcessEntry32A) -> i32;
    pub(crate) fn Process32Next(snapshot: isize, entry: *mut ProcessEntry32A) -> i32;
    fn CloseHandle(handle: isize) -> i32;
    fn OpenProcess(desired_access: u32, inherit: i32, pid: u32) -> isize;
    pub(crate) fn QueryFullProcessImageNameW(
        handle: isize,
        flags: u32,
        name: *mut u16,
        size: *mut u32,
    ) -> i32;
    pub(crate) fn GetProcessTimes(
        handle: isize,
        create: *mut FileTime,
        exit: *mut FileTime,
        kernel: *mut FileTime,
        user: *mut FileTime,
    ) -> i32;
    pub(crate) fn TerminateProcess(handle: isize, exit_code: u32) -> i32;
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
    pub(crate) fn GetProcessMemoryInfo(
        handle: isize,
        counters: *mut ProcessMemoryCounters,
        cb: u32,
    ) -> i32;
}

/// An owned Win32 handle. `None` means Windows refused to hand one over,
/// which is the normal answer for protected and SYSTEM-owned processes.
pub(crate) struct Handle(isize);

impl Handle {
    /// Opens `pid` with `access`; `None` when the kernel says no.
    pub(crate) fn open(access: u32, pid: u32) -> Option<Self> {
        // SAFETY: a plain open call, and the handle it returns is owned by
        // the value built here and closed once in `drop`.
        let raw = unsafe { OpenProcess(access, 0, pid) };
        Self::adopt(raw)
    }

    /// Takes over a handle Windows already returned (a snapshot).
    pub(crate) fn adopt(raw: isize) -> Option<Self> {
        (raw != 0 && raw != INVALID_HANDLE).then_some(Self(raw))
    }

    /// The raw handle for the calls that take one.
    pub(crate) fn raw(&self) -> isize {
        self.0
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        // SAFETY: `self.0` is a live handle this value owns and closes once.
        unsafe {
            CloseHandle(self.0);
        }
    }
}

/// Reads a growable iphlpapi table (TCP or UDP) into its rows.
///
/// # Safety
///
/// `get` must be one of the extended table readers, `af`/`class` its
/// documented constants, and `T` the row type that reader fills.
pub(crate) unsafe fn read_table<T: Copy>(get: TableCall, af: u32, class: u32) -> Vec<T> {
    // SAFETY: with a null table the first call only reports the size it
    // needs; the second fills a buffer of exactly that size, and the count
    // in its first four bytes bounds the rows read.
    unsafe {
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
}

/// Enables SeDebugPrivilege on the current process so `OpenProcess` can reach
/// SYSTEM-owned services (most svchost.exe). AdjustTokenPrivileges returns
/// TRUE even when the caller is not elevated; ERROR_NOT_ALL_ASSIGNED is the
/// honest signal, so it is checked explicitly. Kernel/protected processes
/// (System, PPL) stay unreadable even with this privilege and are reported as
/// limited.
pub(crate) fn enable_debug_privilege() -> bool {
    // SAFETY: every call below is handed pointers to locals that outlive it.
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
        let Some(token) = Handle::adopt(token) else {
            return false;
        };
        let name: Vec<u16> = "SeDebugPrivilege\0".encode_utf16().collect();
        let mut luid = Luid::default();
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
                token.raw(),
                0,
                &state,
                std::mem::size_of::<TokenPrivileges>() as u32,
                null_mut(),
                null_mut(),
            ) != 0
                && GetLastError() != ERROR_NOT_ALL_ASSIGNED;
        }
        ok
    }
}

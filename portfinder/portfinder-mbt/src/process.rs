//! Process metadata: who is running, where its image lives, when it started,
//! how much CPU and memory it has used, and how to end it. Windows refuses
//! some of this for protected processes; the honest answer there is `None`
//! or `false`, never a panic and never a guess.

use std::collections::BTreeMap;
use std::mem::{size_of, zeroed};

use crate::sys::{
    self, FileTime, Handle, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_TERMINATE,
    ProcessEntry32A, ProcessMemoryCounters, TH32CS_SNAPPROCESS,
};

/// A process as the snapshot knows it: image name and parent.
pub(crate) struct ProcInfo {
    pub(crate) name: String,
    pub(crate) ppid: u32,
}

/// What the counters say about one process.
#[derive(Clone, Copy)]
pub(crate) struct ProcStats {
    /// Unix seconds; Windows counts from 1601.
    pub(crate) started: i64,
    /// Kernel plus user time, in 100 ns ticks.
    pub(crate) cpu_ticks: u64,
    /// Working set in KiB, or `-1` when the counters were refused.
    pub(crate) mem_kb: i64,
}

/// Every process the toolhelp snapshot lists, by pid.
pub(crate) fn processes() -> BTreeMap<u32, ProcInfo> {
    let mut out = BTreeMap::new();
    // SAFETY: a snapshot handle, owned by `Handle` from here on.
    let Some(snapshot) = Handle::adopt(unsafe { sys::CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) })
    else {
        return out;
    };
    // SAFETY: PROCESSENTRY32 is plain data; `dw_size` is what Windows checks.
    let mut entry: ProcessEntry32A = unsafe { zeroed() };
    entry.dw_size = size_of::<ProcessEntry32A>() as u32;
    // SAFETY: `entry` is a live, correctly sized PROCESSENTRY32 for each call.
    if unsafe { sys::Process32First(snapshot.raw(), &mut entry) } != 0 {
        loop {
            out.insert(
                entry.th32_process_id,
                ProcInfo {
                    name: c_string(&entry.sz_exe_file),
                    ppid: entry.th32_parent_process_id,
                },
            );
            // SAFETY: as above; a zero return means the walk is over.
            if unsafe { sys::Process32Next(snapshot.raw(), &mut entry) } == 0 {
                break;
            }
        }
    }
    out
}

/// The full image path, or empty when the process denies the query.
pub(crate) fn process_path(pid: u32) -> String {
    let Some(handle) = Handle::open(PROCESS_QUERY_LIMITED_INFORMATION, pid) else {
        return String::new();
    };
    let mut buf = [0u16; 32768];
    let mut len = buf.len() as u32;
    // SAFETY: the buffer is `len` UTF-16 units and `len` is what Windows
    // fills; on success it is the written length, so the slice is in range.
    let ok = unsafe { sys::QueryFullProcessImageNameW(handle.raw(), 0, buf.as_mut_ptr(), &mut len) };
    if ok == 0 {
        return String::new();
    }
    String::from_utf16_lossy(&buf[..len as usize])
}

/// Start time, CPU time and working set; `None` when the handle is refused.
pub(crate) fn process_stats(pid: u32) -> Option<ProcStats> {
    let handle = Handle::open(PROCESS_QUERY_LIMITED_INFORMATION, pid)?;
    let mut create = FileTime::default();
    let mut exit = FileTime::default();
    let mut kernel = FileTime::default();
    let mut user = FileTime::default();
    // SAFETY: four distinct FILETIME outs, owned by this frame.
    let ok = unsafe {
        sys::GetProcessTimes(
            handle.raw(),
            &mut create,
            &mut exit,
            &mut kernel,
            &mut user,
        )
    };
    if ok == 0 {
        return None;
    }
    let mut counters = ProcessMemoryCounters::default();
    counters.cb = size_of::<ProcessMemoryCounters>() as u32;
    // SAFETY: `cb` says how large the structure is, which is what psapi reads.
    let mem_ok = unsafe { sys::GetProcessMemoryInfo(handle.raw(), &mut counters, counters.cb) };
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

/// Forcefully ends a process. SYSTEM / protected processes cannot be opened
/// with TERMINATE access even by an elevated caller, so failure is an honest
/// `false` rather than a panic.
pub(crate) fn terminate(pid: u32) -> bool {
    let Some(handle) = Handle::open(PROCESS_TERMINATE, pid) else {
        return false;
    };
    // SAFETY: the handle was opened for exactly this right.
    unsafe { sys::TerminateProcess(handle.raw(), 1) != 0 }
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

fn filetime_value(ft: &FileTime) -> u64 {
    ((ft.high as u64) << 32) | (ft.low as u64)
}

/// 100 ns ticks since 1601 -> Unix seconds.
fn filetime_unix(ticks: u64) -> i64 {
    (ticks / 10_000_000) as i64 - 11_644_473_600
}

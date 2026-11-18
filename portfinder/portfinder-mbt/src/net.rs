//! The socket tables: which endpoints listen (TCP) or are bound (UDP), who
//! owns them, and how their addresses read. Byte order and address formatting
//! stop here, so no caller ever sees a raw Win32 row.

use std::net::Ipv6Addr;

use crate::sys::{
    self, AF_INET, AF_INET6, MIB_TCP_STATE_LISTEN, MibTcp6RowOwnerPid, MibTcpRowOwnerPid,
    MibUdp6RowOwnerPid, MibUdpRowOwnerPid, TCP_TABLE_OWNER_PID_ALL, UDP_TABLE_OWNER_PID,
};

/// Which address family a table is read for.
#[derive(Clone, Copy)]
pub(crate) enum Family {
    V4,
    V6,
}

/// Both families, in the order rows are scanned.
pub(crate) const FAMILIES: [Family; 2] = [Family::V4, Family::V6];

/// One endpoint, as the tables report it.
pub(crate) struct Socket {
    /// The process the endpoint belongs to.
    pub(crate) pid: u32,
    /// Host byte order, so it compares with a port the user typed.
    pub(crate) port: u32,
    /// The bind address, wildcards (`0.0.0.0`, `::`) included.
    pub(crate) addr: String,
}

impl Socket {
    /// `addr:port`, IPv6 bracketed, as the UI shows an address.
    pub(crate) fn endpoint(&self) -> String {
        if self.addr.contains(':') {
            format!("[{}]:{}", self.addr, self.port)
        } else {
            format!("{}:{}", self.addr, self.port)
        }
    }
}

/// TCP endpoints in the `Listen` state: what actually serves a port.
pub(crate) fn tcp_sockets(family: Family) -> Vec<Socket> {
    match family {
        Family::V4 => table::<MibTcpRowOwnerPid>(
            sys::GetExtendedTcpTable,
            AF_INET,
            TCP_TABLE_OWNER_PID_ALL,
        )
        .into_iter()
        .filter(|r| r.state == MIB_TCP_STATE_LISTEN)
        .map(|r| Socket {
            pid: r.owning_pid,
            port: host_port(r.local_port),
            addr: ipv4(r.local_addr),
        })
        .collect(),
        Family::V6 => table::<MibTcp6RowOwnerPid>(
            sys::GetExtendedTcpTable,
            AF_INET6,
            TCP_TABLE_OWNER_PID_ALL,
        )
        .into_iter()
        .filter(|r| r.state == MIB_TCP_STATE_LISTEN)
        .map(|r| Socket {
            pid: r.owning_pid,
            port: host_port(r.local_port),
            addr: Ipv6Addr::from(r.local_addr).to_string(),
        })
        .collect(),
    }
}

/// Bound UDP endpoints. A bound socket has no state, so every row counts:
/// a UDP port someone holds is a UDP port someone occupies.
pub(crate) fn udp_sockets(family: Family) -> Vec<Socket> {
    match family {
        Family::V4 => table::<MibUdpRowOwnerPid>(
            sys::GetExtendedUdpTable,
            AF_INET,
            UDP_TABLE_OWNER_PID,
        )
        .into_iter()
        .map(|r| Socket {
            pid: r.owning_pid,
            port: host_port(r.local_port),
            addr: ipv4(r.local_addr),
        })
        .collect(),
        Family::V6 => table::<MibUdp6RowOwnerPid>(
            sys::GetExtendedUdpTable,
            AF_INET6,
            UDP_TABLE_OWNER_PID,
        )
        .into_iter()
        .map(|r| Socket {
            pid: r.owning_pid,
            port: host_port(r.local_port),
            addr: Ipv6Addr::from(r.local_addr).to_string(),
        })
        .collect(),
    }
}

fn table<T: Copy>(get: sys::TableCall, af: u32, class: u32) -> Vec<T> {
    // SAFETY: `get` is one of the extended table readers, `af`/`class` the
    // constants they document, and `T` the row type that reader fills.
    unsafe { sys::read_table(get, af, class) }
}

/// The tables report ports big-endian; a port the user compares against is not.
fn host_port(raw: u32) -> u32 {
    ((raw & 0xFF) << 8) | ((raw >> 8) & 0xFF)
}

/// The tables report IPv4 addresses in host order, low byte first.
fn ipv4(raw: u32) -> String {
    format!(
        "{}.{}.{}.{}",
        raw & 0xFF,
        (raw >> 8) & 0xFF,
        (raw >> 16) & 0xFF,
        (raw >> 24) & 0xFF
    )
}

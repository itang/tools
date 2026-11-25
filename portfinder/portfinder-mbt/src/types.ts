// The two record shapes shared by the scanners, the store and the views.

export interface Entry {
  name: string;
  pid: number;
  proto: string;
  addr: string;
}

export interface ProcessRow {
  pid: number;
  name: string;
  ppid: number;
  path: string;
  tcp_ports: string;
  udp_ports: string;
  listeners: string;
  started_s: number;
  cpu_x10: number;
  mem_kb: number;
}

// The scan's completion states: 0 idle, 1 loading, 2 ready, 3 failed.
export type ScanStatus = 0 | 1 | 2 | 3;

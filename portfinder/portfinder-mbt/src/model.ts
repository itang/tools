// Pure data work: formatting, sorting and filtering. No platform calls, so
// these functions stay testable and the views only describe what to show.
import type { Entry, ProcessRow } from "./types";

export function words(line: string): string[] {
  return line.split(/[ \t]+/).filter((part) => part.length > 0);
}

export function portList(raw: string): string {
  return raw.split(",").map((part) => part.trim()).filter((part) => part.length > 0).join(", ");
}

// A process may listen on TCP and bind UDP at once, so the column says which
// list a port comes from.
export function portsLabel(tcp: string, udp: string): string {
  const tcpText = portList(tcp);
  const udpText = portList(udp);
  if (tcpText === "" && udpText === "") return "";
  if (udpText === "") return `TCP ${tcpText}`;
  if (tcpText === "") return `UDP ${udpText}`;
  return `TCP ${tcpText} · UDP ${udpText}`;
}

export function cpuLabel(cpuX10: number): string {
  if (cpuX10 === -2) return "受限";
  if (cpuX10 < 0) return "—";
  return `${Math.trunc(cpuX10 / 10)}.${cpuX10 % 10}%`;
}

export function memLabel(memKb: number): string {
  if (memKb === -2) return "受限";
  if (memKb < 0) return "—";
  if (memKb >= 1048576) {
    const whole = Math.trunc(memKb / 1048576);
    const tenth = Math.trunc((memKb % 1048576) * 10 / 1048576);
    return `${whole}.${tenth} GB`;
  }
  const whole = Math.trunc(memKb / 1024);
  const tenth = Math.trunc((memKb % 1024) * 10 / 1024);
  return `${whole}.${tenth} MB`;
}

// IPv4 or IPv6 from the local address; empty when the socket is a wildcard
// the system does not say the family of.
export function addrFamily(addr: string): string {
  if (addr === "") return "";
  if (addr.startsWith("[") || addr.split(":").length > 2) return "IPv6";
  if (addr.includes("*")) return "";
  return "IPv4";
}

export function entryShort(entry: Entry): string {
  const family = addrFamily(entry.addr);
  const base = `${entry.proto} ${entry.addr}`;
  return family === "" ? base : `${base}（${family}）`;
}

export function sinceLabel(seconds: number): string {
  if (seconds < 0) return "";
  const minutes = Math.trunc(seconds / 60);
  const days = Math.trunc(minutes / 1440);
  const hours = Math.trunc((minutes % 1440) / 60);
  const mins = minutes % 60;
  if (days > 0) return hours > 0 ? `${days}天${hours}小时` : `${days}天`;
  if (hours > 0) return mins > 0 ? `${hours}小时${mins}分` : `${hours}小时`;
  if (mins > 0) return `${mins}分`;
  return "不到 1 分钟";
}

export function firstPort(raw: string): number {
  return parseNumber(raw.split(",")[0]?.trim() ?? "");
}

export function minPort(proc: ProcessRow): number {
  const tcp = firstPort(proc.tcp_ports);
  const udp = firstPort(proc.udp_ports);
  if (tcp === 0) return udp;
  if (udp === 0) return tcp;
  return Math.min(tcp, udp);
}

// Rows the system withholds a start time for sort after the known ones.
export function compareStart(a: ProcessRow, b: ProcessRow): number {
  if (a.started_s === 0 && b.started_s === 0) return 0;
  if (a.started_s === 0) return 1;
  if (b.started_s === 0) return -1;
  return a.started_s - b.started_s;
}

// Unknown numbers (-1) sort after the known ones.
export function compareMetric(a: number, b: number): number {
  if (a < 0 && b < 0) return 0;
  if (a < 0) return 1;
  if (b < 0) return -1;
  return a - b;
}

export function compareRows(a: ProcessRow, b: ProcessRow, key: string): number {
  switch (key) {
    case "name": return a.name.localeCompare(b.name, undefined, { sensitivity: "base" });
    case "pid": return a.pid - b.pid;
    case "ports": return minPort(a) - minPort(b);
    case "cpu": return compareMetric(a.cpu_x10, b.cpu_x10);
    case "mem": return compareMetric(a.mem_kb, b.mem_kb);
    case "uptime": return -compareStart(a, b);
    default: return minPort(a) - minPort(b);
  }
}

export function indexOfPid(rows: ProcessRow[], pid: number): number {
  return rows.findIndex((row) => row.pid === pid);
}

// The rows the query condition leaves: a case-insensitive substring of the
// process name, or every row when the field is empty.
export function visibleRows(procs: ProcessRow[], filter: string): ProcessRow[] {
  const needle = filter.trim().toLowerCase();
  if (needle === "") return procs;
  return procs.filter((proc) => proc.name.toLowerCase().includes(needle));
}

// The list's default order (by first port) and the user's sort share one
// function, so a refresh keeps the order the table shows.
export function sortProcesses(
  rows: ProcessRow[],
  key: string,
  ascending: boolean,
): void {
  rows.sort((a, b) => {
    const order = compareRows(a, b, key);
    return ascending ? order : -order;
  });
}

export function answerText(entries: Entry[], port: number): string {
  if (entries.length === 0) return `端口 ${port} 空闲`;
  const owners: string[] = [];
  const pids: number[] = [];
  const anyTcp = entries.some((entry) => entry.proto === "TCP");
  for (const entry of entries) {
    if (!pids.includes(entry.pid)) {
      pids.push(entry.pid);
      owners.push(entry.name === "" ? `PID ${entry.pid}` : `${entry.name}（PID ${entry.pid}）`);
    }
  }
  const verb = anyTcp ? "监听" : "绑定";
  return `端口 ${port} 正被 ${owners.join("、")} ${verb}`;
}

export function addressDetail(entries: Entry[]): string {
  const text = entries.slice(0, 3).map(entryShort).join(" · ");
  return entries.length > 3 ? `${text} 等` : text;
}

// Plain-number helpers matching the old byte-wise parsers: a leading run of
// digits, and `!` handled by the caller.
export function parseNumber(value: string): number {
  const match = /^\d+/.exec(value);
  return match ? Number(match[0]) : 0;
}

// A decimal (`0.3`, `12`) in tenths; -1 when the text has no digits.
export function parseX10(value: string): number {
  const match = /^(\d+)(?:\.(\d))?/.exec(value);
  if (!match) return -1;
  return Number(match[1]) * 10 + (match[2] ? Number(match[2]) : 0);
}

export function allDigits(value: string): boolean {
  return /^\d+$/.test(value);
}

export function isDecimal(value: string): boolean {
  return /^\d+(?:\.\d+)?$/.test(value);
}

export function leadingDigits(value: string): string {
  const match = /^\d*/.exec(value);
  return match ? match[0] : "";
}

export function parseEtime(text: string): number {
  const dash = text.indexOf("-");
  let days = 0;
  let rest = text;
  if (dash >= 0) {
    days = parseNumber(rest.slice(0, dash));
    rest = rest.slice(dash + 1);
  }
  let seconds = 0;
  for (const part of rest.split(":")) {
    seconds = seconds * 60 + parseNumber(part);
  }
  return days * 86400 + seconds;
}

export function portAfterLastColon(addr: string): string {
  const at = addr.lastIndexOf(":");
  return at < 0 ? "" : addr.slice(at + 1);
}

// Splits a ps line into leading whitespace fields, keeping the rest — the
// command, which may contain spaces — as the final field.
export function psFields(line: string): string[] {
  const fields: string[] = [];
  let rest = line;
  for (let i = 0; i < 5; i++) {
    const trimmed = rest.trimStart();
    if (trimmed === "") break;
    const at = trimmed.search(/[ \t]/);
    if (at < 0) {
      fields.push(trimmed);
      rest = "";
    } else {
      fields.push(trimmed.slice(0, at));
      rest = trimmed.slice(at);
    }
  }
  const last = rest.trim();
  if (last !== "") fields.push(last);
  return fields;
}

// lsof's NAME column (from field 8 on); words() already made `(LISTEN)` its
// own token, so dropping a trailing one mirrors the old regex.
export function lsofAddress(fields: string[]): string {
  const parts: string[] = [];
  for (let i = 8; i < fields.length; i++) {
    if (i === fields.length - 1 && fields[i] === "(LISTEN)") continue;
    parts.push(fields[i]);
  }
  return parts.join(" ");
}

// Every `pid=NNNN` in an ss line.
export function ssPids(line: string): number[] {
  const out: number[] = [];
  const re = /pid=(\d+)/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(line)) !== null) {
    out.push(Number(m[1]));
  }
  return out;
}

export interface PsInfo {
  ppid: string;
  name: string;
  path: string;
  started: string;
  cpu: string;
  mem: string;
}

export interface ScanRow {
  first_port: number;
  line: string;
}

export function addAddr(addrs: Map<number, string[]>, pid: number, label: string): void {
  const list = addrs.get(pid) ?? [];
  addrs.set(pid, list);
  if (!list.includes(label)) list.push(label);
}

// lsof's `-iTCP -sTCP:LISTEN` / `-iUDP` output.
export function addLsof(
  text: string,
  proto: string,
  tcp: Map<number, number[]>,
  udp: Map<number, number[]>,
  addrs: Map<number, string[]>,
): void {
  for (const raw of text.split("\n")) {
    const fields = words(raw);
    if (fields.length < 9 || fields[0] === "COMMAND") continue;
    const pid = parseNumber(fields[1]);
    if (pid === 0) continue;
    const address = lsofAddress(fields);
    if (address.includes("->") || !address.includes(":")) continue;
    const portView = portAfterLastColon(address);
    if (!allDigits(portView)) continue;
    const port = parseNumber(portView);
    const ports = proto === "TCP" ? tcp : udp;
    const list = ports.get(pid) ?? [];
    ports.set(pid, list);
    if (!list.includes(port)) list.push(port);
    addAddr(addrs, pid, `${proto} ${address}`);
  }
}

function joinPortsSorted(ports: number[]): string {
  return [...ports].sort((a, b) => a - b).join(",");
}

// Turns the accumulated maps into the `|`-delimited rows parseProcesses
// reads, ordered by first port like the Rust scanner's output.
export function emitRows(
  info: Map<number, PsInfo>,
  tcp: Map<number, number[]>,
  udp: Map<number, number[]>,
  addrs: Map<number, string[]>,
): string {
  const pids: number[] = [];
  for (const pid of [...tcp.keys(), ...udp.keys()]) {
    if (!pids.includes(pid)) pids.push(pid);
  }
  const rows: ScanRow[] = [];
  for (const pid of pids) {
    const tcpPorts = tcp.get(pid) ?? [];
    const udpPorts = udp.get(pid) ?? [];
    const first = tcpPorts.length > 0 ? tcpPorts[0] : udpPorts.length > 0 ? udpPorts[0] : 0;
    const meta = info.get(pid);
    const line = [
      pid, meta?.name ?? "", meta?.ppid ?? "", meta?.path ?? "",
      joinPortsSorted(tcpPorts), joinPortsSorted(udpPorts),
      meta?.started ?? "", meta?.cpu ?? "", meta?.mem ?? "",
      (addrs.get(pid) ?? []).join(";"),
    ].join("|");
    rows.push({ first_port: first, line });
  }
  rows.sort((a, b) => a.first_port - b.first_port);
  return rows.map((row) => row.line).join("\n");
}

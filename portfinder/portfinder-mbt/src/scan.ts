// Process and port scanning. Windows uses the native nshm (`Scanner`) that
// reads the Win32 tables directly; macOS pairs lsof with ps, Linux pairs ss
// with ps. Every parser is a plain function over text.
import { run } from "process";
import { platform } from "os";
import { Scanner, type ScannerModule } from "portradar-native";
import type { Entry, ProcessRow } from "./types";
import {
  addAddr, addLsof, allDigits, emitRows, isDecimal, lsofAddress, parseEtime,
  parseNumber, parseX10, portAfterLastColon, psFields, ssPids, words, type PsInfo,
} from "./model";

// Collapses a native scan's text to non-empty, newline-joined rows.
function keepLines(text: string): string {
  return text.split("\n").map((line) => line.trim()).filter((line) => line !== "").join("\n");
}

function addrHasPort(addr: string, port: number): boolean {
  const parts = addr.split(":");
  if (parts.length < 2) return false;
  return parseNumber(parts[parts.length - 1]) === port;
}

// `lsof -nP -iPROTO:port` rows: COMMAND PID USER FD TYPE DEVICE SIZE/OFF NODE NAME.
function lsofEntries(text: string, port: number): Entry[] {
  const entries: Entry[] = [];
  for (const raw of text.split("\n")) {
    const line = raw.trim();
    if (line === "") continue;
    const fields = words(line);
    const protoAt = fields.findIndex((field) => field === "TCP" || field === "UDP");
    if (protoAt < 0 || protoAt + 1 >= fields.length) continue;
    const addr = fields[protoAt + 1];
    if (!addrHasPort(addr, port)) continue;
    entries.push({
      name: fields[0], pid: parseNumber(fields[1]), proto: fields[protoAt], addr,
    });
  }
  return entries;
}

function ssQuoted(line: string, index: number): string {
  return line.split('"')[index] ?? "";
}

function ssPid(line: string): number {
  const rest = ssQuoted(line, 2);
  const parts = rest.split("pid=");
  return parts.length >= 2 ? parseNumber(parts[1]) : 0;
}

// `ss -H -l -n -t -u -p` rows: PROTO STATE RECV-Q SEND-Q LOCAL PEER PROCESS.
function ssEntries(text: string, port: number): Entry[] {
  const entries: Entry[] = [];
  for (const raw of text.split("\n")) {
    const line = raw.trim();
    if (line === "") continue;
    const fields = words(line);
    if (fields.length < 5) continue;
    const addr = fields[4];
    if (!addrHasPort(addr, port)) continue;
    entries.push({
      name: ssQuoted(line, 1), pid: ssPid(line), proto: fields[0].toUpperCase(), addr,
    });
  }
  return entries;
}

async function runQuiet(exe: string, args: string[], timeoutMs: number): Promise<string> {
  try {
    const out = await run(exe, args, { timeoutMs });
    return out.stdout;
  } catch {
    return "";
  }
}

// The macOS/Linux fallback: TCP listeners and UDP bindings from lsof.
async function scanLsof(port: number): Promise<Entry[]> {
  const tcp = await runQuiet("lsof", ["-nP", `-iTCP:${port}`, "-sTCP:LISTEN"], 8000);
  const udp = await runQuiet("lsof", ["-nP", `-iUDP:${port}`], 8000);
  return [...lsofEntries(tcp, port), ...lsofEntries(udp, port)];
}

// Runs the platform's port lookup; a permission-limited lookup reports the
// socket with an empty name and pid 0.
export async function scanPort(port: number, scanner: ScannerModule): Promise<Entry[]> {
  const os = platform();
  if (os === "windows") return parseEntries(await scanner.scanPort(port));
  if (os === "macos") return scanLsof(port);
  try {
    const out = await run("ss", ["-H", "-l", "-n", "-t", "-u", "-p"], { timeoutMs: 8000 });
    return ssEntries(out.stdout, port);
  } catch {
    return scanLsof(port);
  }
}

// Ends the process that owns a listener: forced on Windows, SIGTERM on POSIX.
export async function killProcess(
  pid: number,
  scanner: ScannerModule,
): Promise<void> {
  if (platform() === "windows") {
    if (!scanner.killProcess(pid)) {
      throw new Error("进程可能已退出，或需要更高权限");
    }
    return;
  }
  const out = await run("kill", ["-TERM", `${pid}`], { timeoutMs: 8000 });
  if (out.code !== 0) {
    const message = out.stderr.trim();
    throw new Error(message === "" ? `kill 退出码 ${out.code}` : message);
  }
}

// Runs several lookup commands and resolves with their stdout in job order.
async function runMany(jobs: [string, string[]][]): Promise<string[]> {
  return Promise.all(jobs.map(async ([exe, args]) => {
    try {
      const out = await run(exe, args, { timeoutMs: 20000 });
      return out.stdout;
    } catch {
      return "";
    }
  }));
}

// macOS: lsof (TCP listeners + UDP bindings) paired with ps.
export function macosRows(tcpText: string, udpText: string, psText: string, nowS: number): string {
  const info = new Map<number, PsInfo>();
  for (const raw of psText.split("\n")) {
    const fields = psFields(raw);
    if (fields.length < 6) continue;
    if (!allDigits(fields[0]) || !allDigits(fields[1]) || !isDecimal(fields[4])) continue;
    const pid = parseNumber(fields[0]);
    if (pid === 0) continue;
    const comm = fields[5];
    const slash = comm.lastIndexOf("/");
    info.set(pid, {
      ppid: fields[1],
      name: slash >= 0 ? comm.slice(slash + 1) : comm,
      path: slash >= 0 ? comm : "",
      started: String(nowS - parseEtime(fields[2])),
      cpu: String(parseX10(fields[4])),
      mem: String(parseNumber(fields[3])),
    });
  }
  const tcp = new Map<number, number[]>();
  const udp = new Map<number, number[]>();
  const addrs = new Map<number, string[]>();
  addLsof(tcpText, "TCP", tcp, udp, addrs);
  addLsof(udpText, "UDP", tcp, udp, addrs);
  return emitRows(info, tcp, udp, addrs);
}

// Linux: ss paired with ps. `etimes` is already seconds.
export function linuxRows(ssText: string, psText: string, nowS: number): string {
  const info = new Map<number, PsInfo>();
  for (const raw of psText.split("\n")) {
    const fields = psFields(raw);
    if (fields.length < 6) continue;
    if (!allDigits(fields[0]) || !allDigits(fields[1]) ||
        !allDigits(fields[2]) || !isDecimal(fields[4])) continue;
    const pid = parseNumber(fields[0]);
    if (pid === 0) continue;
    info.set(pid, {
      ppid: fields[1], name: fields[5], path: "",
      started: String(nowS - parseNumber(fields[2])),
      cpu: String(parseX10(fields[4])),
      mem: String(parseNumber(fields[3])),
    });
  }
  const tcp = new Map<number, number[]>();
  const udp = new Map<number, number[]>();
  const addrs = new Map<number, string[]>();
  for (const raw of ssText.split("\n")) {
    const fields = words(raw);
    if (fields.length < 5) continue;
    const proto = fields[0].toUpperCase();
    if (proto !== "TCP" && proto !== "UDP") continue;
    const localAddr = fields[4];
    const portView = portAfterLastColon(localAddr);
    if (!allDigits(portView)) continue;
    const port = parseNumber(portView);
    const ports = proto === "TCP" ? tcp : udp;
    for (const pid of ssPids(raw)) {
      const list = ports.get(pid) ?? [];
      ports.set(pid, list);
      if (!list.includes(port)) list.push(port);
      addAddr(addrs, pid, `${proto} ${localAddr}`);
    }
  }
  return emitRows(info, tcp, udp, addrs);
}

// Runs the platform's process scan, returning the `|`-delimited text.
export async function scanProcesses(scanner: ScannerModule): Promise<string> {
  const os = platform();
  if (os === "windows") return keepLines(await scanner.scanProcesses());
  const nowS = Math.floor(Date.now() / 1000);
  if (os === "macos") {
    const out = await runMany([
      ["lsof", ["-nP", "-iTCP", "-sTCP:LISTEN"]],
      ["lsof", ["-nP", "-iUDP"]],
      ["ps", ["-axo", "pid=,ppid=,etime=,rss=,pcpu=,comm="]],
    ]);
    return macosRows(out[0], out[1], out[2], nowS);
  }
  const out = await runMany([
    ["ss", ["-H", "-l", "-n", "-t", "-u", "-p"]],
    ["ps", ["-eo", "pid=,ppid=,etimes=,rss=,pcpu=,comm="]],
  ]);
  return linuxRows(out[0], out[1], nowS);
}

export function parseEntries(text: string): Entry[] {
  const entries: Entry[] = [];
  for (const raw of text.split("\n")) {
    const line = raw.trim();
    if (line === "") continue;
    const fields = line.split("|");
    if (fields.length >= 5) {
      entries.push({
        name: fields[0].trim(), pid: parseNumber(fields[1]),
        proto: fields[2].trim(), addr: fields[4].trim(),
      });
    }
  }
  return entries;
}

export function parseProcesses(text: string): ProcessRow[] {
  const rows: ProcessRow[] = [];
  for (const raw of text.split("\n")) {
    const line = raw.trim();
    if (line === "") continue;
    const fields = line.split("|");
    if (fields.length >= 10) {
      const mem = parseNumber(fields[8]);
      const cpuText = fields[7].trim();
      const memText = fields[8].trim();
      // "!" is emitted when a process is visible but its handle cannot be
      // opened; -2 renders as "受限" instead of the ambiguous "—".
      rows.push({
        pid: parseNumber(fields[0]), name: fields[1].trim(),
        ppid: parseNumber(fields[2]), path: fields[3].trim(),
        tcp_ports: fields[4].trim(), udp_ports: fields[5].trim(),
        started_s: parseNumber(fields[6]),
        cpu_x10: cpuText === "!" ? -2 : parseX10(cpuText),
        mem_kb: memText === "!" ? -2 : mem > 0 ? mem : -1,
        listeners: fields[9].trim(),
      });
    }
  }
  return rows;
}

// Seconds `started_s` has been running, or -1 when the start is unknown.
export function runningSeconds(startedS: number): number {
  if (startedS === 0) return -1;
  const delta = Math.floor(Date.now() / 1000) - startedS;
  return delta > 0 ? delta : 0;
}

// `started_s` as local `YYYY-MM-DD HH:MM`; empty when unknown.
export function formatTime(startedS: number): string {
  if (startedS === 0) return "";
  const d = new Date(startedS * 1000);
  const pad = (n: number) => (n < 10 ? `0${n}` : `${n}`);
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}`;
}

export { Scanner };

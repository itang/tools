// One listener: the process owning it (name and pid may be unknown without
// permission), the protocol and the local address.
priv struct Entry {
  name : String
  pid : Int
  proto : String
  addr : String
}

// The PowerShell lookup: one `name|pid|proto|state|addr` line per listener on
// `port`. The address uses `$($a)` rather than `${a}` so the script has no
// `${` sequence, which the JS backend would read as its own interpolation.
fn windows_port_script(port : Int) -> String {
  "$OutputEncoding=[Console]::OutputEncoding=[Text.Encoding]::UTF8; $p=" +
  port.to_string() +
  ";\n" +
  (
    #|Get-NetTCPConnection -LocalPort $p -State Listen -ErrorAction SilentlyContinue | ForEach-Object { $n=''; try { $n=(Get-Process -Id $_.OwningProcess -ErrorAction Stop).ProcessName } catch {}; $a=$_.LocalAddress; if ($a.Contains(':')) { $a='[' + $a + ']' }; "$n|$($_.OwningProcess)|TCP|Listen|$($a):$($_.LocalPort)" };
    #|Get-NetUDPEndpoint -LocalPort $p -ErrorAction SilentlyContinue | ForEach-Object { $n=''; try { $n=(Get-Process -Id $_.OwningProcess -ErrorAction Stop).ProcessName } catch {}; $a=$_.LocalAddress; if ($a.Contains(':')) { $a='[' + $a + ']' }; "$n|$($_.OwningProcess)|UDP||$($a):$($_.LocalPort)" }; exit 0
  )
}

// Splits one line of `lsof`/`ss` output into its non-empty fields.
fn words(line : StringView) -> Array[StringView] {
  let fields : Array[StringView] = []
  for chunk in line.split(" ") {
    for part in chunk.split("\t") {
      if !part.is_empty() {
        fields.push(part)
      }
    }
  }
  fields
}

// True when `addr` names `port` (`*:8080`, `0.0.0.0:8080`, `[::1]:8080`).
fn addr_has_port(addr : StringView, port : Int) -> Bool {
  let parts = addr.split(":").to_array()
  if parts.length() < 2 {
    false
  } else {
    parse_number(parts[parts.length() - 1]) == port
  }
}

// `lsof -nP -iPROTO:port` rows: COMMAND PID USER FD TYPE DEVICE SIZE/OFF NODE NAME.
fn lsof_entries(text : String, port : Int) -> Array[Entry] {
  let entries : Array[Entry] = []
  for raw in text.split("\n") {
    let line = raw.trim()
    if line.is_empty() {
      continue
    }
    let fields = words(line)
    let mut proto_at = -1
    let mut index = 0
    for field in fields {
      if field == "TCP" || field == "UDP" {
        proto_at = index
        break
      }
      index = index + 1
    }
    if proto_at < 0 || proto_at + 1 >= fields.length() {
      continue
    }
    let addr = fields[proto_at + 1].to_owned()
    if !addr_has_port(addr, port) {
      continue
    }
    entries.push({
      name: fields[0].to_owned(),
      pid: if fields.length() > 1 { parse_number(fields[1]) } else { 0 },
      proto: fields[proto_at].to_owned(),
      addr,
    })
  }
  entries
}

// The `"name"` quoted by `ss`'s `users:(("name",pid=1234,...))` column.
fn ss_quoted(line : StringView, index : Int) -> String {
  let parts = line.split("\"").to_array()
  if parts.length() > index {
    parts[index].to_owned()
  } else {
    ""
  }
}

fn ss_pid(line : StringView) -> Int {
  let rest = ss_quoted(line, 2)
  let parts = rest.split("pid=").to_array()
  if parts.length() >= 2 {
    parse_number(parts[1])
  } else {
    0
  }
}

// `ss -H -l -n -t -u -p` rows: PROTO STATE RECV-Q SEND-Q LOCAL PEER PROCESS.
fn ss_entries(text : String, port : Int) -> Array[Entry] {
  let entries : Array[Entry] = []
  for raw in text.split("\n") {
    let line = raw.trim()
    if line.is_empty() {
      continue
    }
    let fields = words(line)
    if fields.length() < 5 {
      continue
    }
    let addr = fields[4].to_owned()
    if !addr_has_port(addr, port) {
      continue
    }
    entries.push({
      name: ss_quoted(line, 1),
      pid: ss_pid(line),
      proto: fields[0].to_owned().to_upper(),
      addr,
    })
  }
  entries
}

// Runs a lookup tool, treating a missing tool or a failed call as empty
// output so the caller can fall back to another source.
fn Host::run_quiet(
  self : Host,
  exe : String,
  args : FixedArray[String],
  timeout_ms : Int,
  done : (String) -> Unit,
) -> Unit {
  self.run_command(
    exe,
    args,
    timeout_ms,
    fn(_code, stdout, _stderr) { done(stdout) },
    fn(_message) { done("") },
  )
}

// The macOS/Linux fallback: TCP listeners and UDP bindings from lsof.
fn Host::scan_lsof(self : Host, port : Int, done : (Array[Entry]) -> Unit) -> Unit {
  let entries : Array[Entry] = []
  self.run_quiet("lsof", ["-nP", "-iTCP:\{port}", "-sTCP:LISTEN"], 8000, fn(tcp) {
    self.run_quiet("lsof", ["-nP", "-iUDP:\{port}"], 8000, fn(udp) {
      for entry in lsof_entries(tcp, port) {
        entries.push(entry)
      }
      for entry in lsof_entries(udp, port) {
        entries.push(entry)
      }
      done(entries)
    })
  })
}

// Runs the platform's port lookup and reports the listeners it finds; a
// permission-limited lookup reports the socket with an empty name and pid 0.
fn Host::scan_port(
  self : Host,
  port : Int,
  done : (Array[Entry]) -> Unit,
  fail : (String) -> Unit,
) -> Unit {
  let os = self.platform()
  if os == "windows" {
    self.run_command(
      "powershell",
      ["-NoProfile", "-NonInteractive", "-Command", windows_port_script(port)],
      8000,
      fn(code, stdout, _stderr) {
        if code != 0 {
          fail("系统查询命令失败（退出码 \{code}）")
        } else {
          done(parse_entries(stdout))
        }
      },
      fail,
    )
    return
  }
  if os == "macos" {
    self.scan_lsof(port, done)
    return
  }
  self.run_command(
    "ss",
    ["-H", "-l", "-n", "-t", "-u", "-p"],
    8000,
    fn(_code, stdout, _stderr) {
      done(ss_entries(stdout, port))
    },
    fn(_message) {
      self.scan_lsof(port, done)
    },
  )
}

// Ends the process that owns a listener: forced on Windows, SIGTERM on
// macOS and Linux. The platform's error text is only shown on POSIX, where
// it arrives as UTF-8; Windows' localized output would not decode cleanly.
fn Host::kill_process(
  self : Host,
  pid : Int,
  done : () -> Unit,
  fail : (String) -> Unit,
) -> Unit {
  if self.platform() == "windows" {
    self.run_command(
      "taskkill",
      ["/PID", "\{pid}", "/F"],
      8000,
      fn(code, _stdout, _stderr) {
        if code == 0 {
          done()
        } else {
          fail("进程可能已退出，或需要更高权限")
        }
      },
      fail,
    )
    return
  }
  self.run_command(
    "kill",
    ["-TERM", "\{pid}"],
    8000,
    fn(code, _stdout, stderr) {
      if code == 0 {
        done()
      } else {
        let message = stderr.trim()
        if message.is_empty() {
          fail("kill 退出码 \{code}")
        } else {
          fail(message.to_owned())
        }
      }
    },
    fail,
  )
}

// One process that holds listening TCP sockets or bound UDP endpoints, with
// whatever metadata the system exposes; fields it withholds stay empty
// (`-1` for an unknown number).
priv struct ProcessRow {
  pid : Int
  name : String
  ppid : Int
  path : String
  tcp_ports : String
  udp_ports : String
  listeners : String
  started_s : Int
  cpu_x10 : Int
  mem_kb : Int
}

// Lists every process behind a listening TCP socket or UDP endpoint. Windows
// reads CIM and the performance counters, macOS pairs lsof with ps, Linux
// pairs ss with ps; a permission limit only empties fields. Rows are
// `pid|name|ppid|path|tcp_ports|udp_ports|started_s|cpu_x10|mem_kb|listeners`,
// the listeners being `PROTO addr` items joined by `;`.
extern "js" fn Host::scan_processes(
  self : Host,
  done : (String) -> Unit,
  fail : (String) -> Unit,
) -> Unit =
  #| (h, done, fail) => {
  #|   const failed = (e) => fail(e && e.message ? e.message : String(e));
  #|   const keep = (text) =>
  #|     text.split(/\r?\n/).filter((line) => line.trim() !== "").join("\n");
  #|   const nowSeconds = Math.floor(Date.now() / 1000);
  #|   const firstPort = (r) =>
  #|     Number(String(r.tcp).split(",")[0]) ||
  #|     Number(String(r.udp).split(",")[0]) ||
  #|     0;
  #|   const emit = (rows) =>
  #|     done(
  #|       rows
  #|         .sort((a, b) => firstPort(a) - firstPort(b))
  #|         .map((r) =>
  #|           [
  #|             r.pid,
  #|             r.name === undefined ? "" : r.name,
  #|             r.ppid === undefined ? "" : r.ppid,
  #|             r.path === undefined ? "" : r.path,
  #|             r.tcp === undefined ? "" : r.tcp,
  #|             r.udp === undefined ? "" : r.udp,
  #|             r.started === undefined ? "" : r.started,
  #|             r.cpu === undefined ? "" : r.cpu,
  #|             r.mem === undefined ? "" : r.mem,
  #|             r.addrs === undefined ? "" : r.addrs,
  #|           ].join("|"),
  #|         )
  #|         .join("\n"),
  #|     );
  #|   const os = h.platform;
  #|   if (os === "windows") {
  #|     const script = `$OutputEncoding=[Console]::OutputEncoding=[Text.Encoding]::UTF8; $ErrorActionPreference='SilentlyContinue';
  #| $cores=[Environment]::ProcessorCount; if (-not $cores) { $cores=1 }
  #| $procs=@{}; Get-CimInstance Win32_Process | ForEach-Object { $procs[[int]$_.ProcessId]=$_ };
  #| $perf=@{}; Get-CimInstance Win32_PerfFormattedData_PerfProc_Process | ForEach-Object { $perf[[int]$_.IDProcess]=$_.PercentProcessorTime };
  #| $tcp=@{}; $udp=@{}; $addrs=@{};
  #| Get-NetTCPConnection -State Listen | ForEach-Object {
  #|   $k=[int]$_.OwningProcess;
  #|   if (-not $tcp.ContainsKey($k)) { $tcp[$k]=New-Object System.Collections.Generic.HashSet[int] };
  #|   if (-not $addrs.ContainsKey($k)) { $addrs[$k]=New-Object System.Collections.Generic.HashSet[string] };
  #|   [void]$tcp[$k].Add([int]$_.LocalPort);
  #|   $a=$_.LocalAddress; if ($a.Contains(':')) { $a='[' + $a + ']' };
  #|   [void]$addrs[$k].Add("TCP $($a):$($_.LocalPort)")
  #| };
  #| Get-NetUDPEndpoint | ForEach-Object {
  #|   $k=[int]$_.OwningProcess;
  #|   if (-not $udp.ContainsKey($k)) { $udp[$k]=New-Object System.Collections.Generic.HashSet[int] };
  #|   if (-not $addrs.ContainsKey($k)) { $addrs[$k]=New-Object System.Collections.Generic.HashSet[string] };
  #|   [void]$udp[$k].Add([int]$_.LocalPort);
  #|   $a=$_.LocalAddress; if ($a.Contains(':')) { $a='[' + $a + ']' };
  #|   [void]$addrs[$k].Add("UDP $($a):$($_.LocalPort)")
  #| };
  #| $rows=@();
  #| foreach ($k in (@($tcp.Keys) + @($udp.Keys) | Sort-Object -Unique)) {
  #|   $c=$procs[$k]; $name=''; $ppid=''; $path=''; $started=''; $mem=''; $cpu='';
  #|   if ($c) {
  #|     $name=$c.Name; $ppid=[string]$c.ParentProcessId; $path=$c.ExecutablePath;
  #|     if ($c.CreationDate) { $started=[string]([DateTimeOffset]$c.CreationDate).ToUnixTimeSeconds() };
  #|     if ($c.WorkingSetSize) { $mem=[int][math]::Round([double]$c.WorkingSetSize/1024) }
  #|   };
  #|   if ($perf.ContainsKey($k)) { $cpu=[int][math]::Round([math]::Min(100.0,[double]$perf[$k]/$cores)*10) };
  #|   $tl=@(); if ($tcp.ContainsKey($k)) { $tl=@($tcp[$k] | Sort-Object) };
  #|   $ul=@(); if ($udp.ContainsKey($k)) { $ul=@($udp[$k] | Sort-Object) };
  #|   $al=@(); if ($addrs.ContainsKey($k)) { $al=@($addrs[$k] | Sort-Object) };
  #|   $rows += [pscustomobject]@{ pid=$k; name=$name; ppid=$ppid; path=$path; tcp=($tl -join ','); udp=($ul -join ','); started=$started; cpu=$cpu; mem=$mem; addrs=($al -join ';') }
  #| };
  #| $rows | Sort-Object { if ($_.tcp) { [int](($_.tcp -split ',')[0]) } else { [int](($_.udp -split ',')[0]) } } | ForEach-Object { "$($_.pid)|$($_.name)|$($_.ppid)|$($_.path)|$($_.tcp)|$($_.udp)|$($_.started)|$($_.cpu)|$($_.mem)|$($_.addrs)" }; exit 0`;
  #|     h.process
  #|       .run("powershell", ["-NoProfile", "-NonInteractive", "-Command", script], { timeoutMs: 20000 })
  #|       .then((out) => {
  #|         if (out.code !== 0) {
  #|           fail("系统进程列表查询失败（退出码 " + out.code + "）");
  #|         } else {
  #|           done(keep(out.stdout));
  #|         }
  #|       }, failed);
  #|     return;
  #|   }
  #|   const run = (name, args) =>
  #|     h.process.run(name, args, { timeoutMs: 20000 }).then((out) => out.stdout).catch(() => "");
  #|   const parseEtime = (text) => {
  #|     let rest = text;
  #|     let days = 0;
  #|     if (rest.includes("-")) {
  #|       const at = rest.indexOf("-");
  #|       days = Number(rest.slice(0, at)) || 0;
  #|       rest = rest.slice(at + 1);
  #|     }
  #|     let seconds = 0;
  #|     for (const part of rest.split(":")) seconds = seconds * 60 + (Number(part) || 0);
  #|     return days * 86400 + seconds;
  #|   };
  #|   if (os === "macos") {
  #|     Promise.all([
  #|       run("lsof", ["-nP", "-iTCP", "-sTCP:LISTEN"]),
  #|       run("lsof", ["-nP", "-iUDP"]),
  #|       run("ps", ["-axo", "pid=,ppid=,etime=,rss=,pcpu=,comm="]),
  #|     ]).then(([tcp, udp, psOut]) => {
  #|       const info = new Map();
  #|       for (const line of psOut.split(/\r?\n/)) {
  #|         const m = /^\s*(\d+)\s+(\d+)\s+(\S+)\s+(\d+)\s+([\d.]+)\s+(.+)$/.exec(line);
  #|         if (!m) continue;
  #|         const comm = m[6].trim();
  #|         const slash = comm.lastIndexOf("/");
  #|         info.set(Number(m[1]), {
  #|           ppid: m[2],
  #|           name: slash >= 0 ? comm.slice(slash + 1) : comm,
  #|           path: slash >= 0 ? comm : "",
  #|           etime: m[3],
  #|           cpu: Math.round((Number(m[5]) || 0) * 10),
  #|           mem: Number(m[4]) || 0,
  #|         });
  #|       }
  #|       const tcpPorts = new Map();
  #|       const udpPorts = new Map();
  #|       const addrs = new Map();
  #|       const add = (text, proto) => {
  #|         for (const line of text.split(/\r?\n/)) {
  #|           const t = line.trim().split(/\s+/);
  #|           if (t.length < 9 || t[0] === "COMMAND") continue;
  #|           const pid = Number(t[1]);
  #|           if (!pid) continue;
  #|           const address = t.slice(8).join(" ").replace(/\s+\(LISTEN\)$/, "");
  #|           if (address.includes("->") || !address.includes(":")) continue;
  #|           const port = address.slice(address.lastIndexOf(":") + 1);
  #|           if (!/^\d+$/.test(port)) continue;
  #|           const ports = proto === "TCP" ? tcpPorts : udpPorts;
  #|           if (!ports.has(pid)) ports.set(pid, new Set());
  #|           ports.get(pid).add(Number(port));
  #|           if (!addrs.has(pid)) addrs.set(pid, new Set());
  #|           addrs.get(pid).add(proto + " " + address);
  #|         }
  #|       };
  #|       add(tcp, "TCP");
  #|       add(udp, "UDP");
  #|       const rows = [];
  #|       for (const pid of new Set([...tcpPorts.keys(), ...udpPorts.keys()])) {
  #|         const p = info.get(pid) || {};
  #|         rows.push({
  #|           pid,
  #|           name: p.name || "",
  #|           ppid: p.ppid || "",
  #|           path: p.path || "",
  #|           tcp: [...(tcpPorts.get(pid) || [])].sort((a, b) => a - b).join(","),
  #|           udp: [...(udpPorts.get(pid) || [])].sort((a, b) => a - b).join(","),
  #|           started: p.etime === undefined ? "" : nowSeconds - parseEtime(p.etime),
  #|           cpu: p.cpu === undefined ? "" : p.cpu,
  #|           mem: p.mem === undefined ? "" : p.mem,
  #|           addrs: [...(addrs.get(pid) || [])].join(";"),
  #|         });
  #|       }
  #|       emit(rows);
  #|     }, failed);
  #|     return;
  #|   }
  #|   Promise.all([
  #|     run("ss", ["-H", "-l", "-n", "-t", "-u", "-p"]),
  #|     run("ps", ["-eo", "pid=,ppid=,etimes=,rss=,pcpu=,comm="]),
  #|   ]).then(([ssOut, psOut]) => {
  #|     const info = new Map();
  #|     for (const line of psOut.split(/\r?\n/)) {
  #|       const m = /^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+([\d.]+)\s+(.+)$/.exec(line);
  #|       if (!m) continue;
  #|       info.set(Number(m[1]), {
  #|         ppid: m[2],
  #|         name: m[6].trim(),
  #|         etime: Number(m[3]),
  #|         mem: Number(m[4]) || 0,
  #|         cpu: Math.round((Number(m[5]) || 0) * 10),
  #|       });
  #|     }
  #|     const tcpPorts = new Map();
  #|     const udpPorts = new Map();
  #|     const addrs = new Map();
  #|     for (const line of ssOut.split(/\r?\n/)) {
  #|       if (!line.trim()) continue;
  #|       const parts = line.trim().split(/\s+/);
  #|       const proto = (parts[0] || "").toUpperCase();
  #|       if (proto !== "TCP" && proto !== "UDP") continue;
  #|       const local = parts[4] || "";
  #|       const port = local.slice(local.lastIndexOf(":") + 1);
  #|       if (!/^\d+$/.test(port)) continue;
  #|       const re = /pid=(\d+)/g;
  #|       let m;
  #|       while ((m = re.exec(line))) {
  #|         const pid = Number(m[1]);
  #|         const ports = proto === "TCP" ? tcpPorts : udpPorts;
  #|         if (!ports.has(pid)) ports.set(pid, new Set());
  #|         ports.get(pid).add(Number(port));
  #|         if (!addrs.has(pid)) addrs.set(pid, new Set());
  #|         addrs.get(pid).add(proto + " " + local);
  #|       }
  #|     }
  #|     const rows = [];
  #|     for (const pid of new Set([...tcpPorts.keys(), ...udpPorts.keys()])) {
  #|       const p = info.get(pid) || {};
  #|       rows.push({
  #|         pid,
  #|         name: p.name || "",
  #|         ppid: p.ppid || "",
  #|         path: "",
  #|         tcp: [...(tcpPorts.get(pid) || [])].sort((a, b) => a - b).join(","),
  #|         udp: [...(udpPorts.get(pid) || [])].sort((a, b) => a - b).join(","),
  #|         started: p.etime === undefined ? "" : nowSeconds - p.etime,
  #|         cpu: p.cpu === undefined ? "" : p.cpu,
  #|         mem: p.mem === undefined ? "" : p.mem,
  #|         addrs: [...(addrs.get(pid) || [])].join(";"),
  #|       });
  #|     }
  #|     emit(rows);
  #|   }, failed);
  #| }

// Seconds `started_s` has been running, or -1 when the start is unknown.
extern "js" fn Host::running_seconds(self : Host, started_s : Int) -> Int =
  #| (h, started) => {
  #|   if (!started) return -1;
  #|   return Math.max(0, Math.floor(Date.now() / 1000) - started);
  #| }

// `started_s` as local `YYYY-MM-DD HH:MM`; an empty label when unknown.
extern "js" fn Host::format_time(self : Host, started_s : Int) -> String =
  #| (h, started) => {
  #|   if (!started) return "";
  #|   const d = new Date(started * 1000);
  #|   const pad = (n) => String(n).padStart(2, "0");
  #|   return d.getFullYear() + "-" + pad(d.getMonth() + 1) + "-" + pad(d.getDate()) +
  #|     " " + pad(d.getHours()) + ":" + pad(d.getMinutes());
  #| }

fn parse_number(value : StringView) -> Int {
  let mut number = 0
  let mut seen = false
  for char in value {
    let digit = char.to_int() - 48
    if digit < 0 || digit > 9 {
      break
    }
    number = number * 10 + digit
    seen = true
  }
  if seen {
    number
  } else {
    0
  }
}

fn parse_entries(text : String) -> Array[Entry] {
  let entries : Array[Entry] = []
  for raw in text.split("\n") {
    let line = raw.trim()
    if line.is_empty() {
      continue
    }
    let fields = line.split("|").to_array()
    if fields.length() >= 5 {
      entries.push({
        name: fields[0].trim().to_owned(),
        pid: parse_number(fields[1]),
        proto: fields[2].trim().to_owned(),
        addr: fields[4].trim().to_owned(),
      })
    }
  }
  entries
}

// A decimal (`0.3`, `12`) in tenths; -1 when the text has no digits.
fn parse_x10(value : StringView) -> Int {
  let mut whole = 0
  let mut frac = 0
  let mut seen = false
  let mut decimal = false
  let mut frac_digits = 0
  for char in value {
    let digit = char.to_int() - 48
    if digit >= 0 && digit <= 9 {
      if decimal {
        if frac_digits == 0 {
          frac = digit
        }
        frac_digits = frac_digits + 1
      } else {
        whole = whole * 10 + digit
      }
      seen = true
    } else if char == '.' {
      decimal = true
    } else {
      break
    }
  }
  if seen {
    whole * 10 + frac
  } else {
    -1
  }
}

fn parse_processes(text : String) -> Array[ProcessRow] {
  let rows : Array[ProcessRow] = []
  for raw in text.split("\n") {
    let line = raw.trim()
    if line.is_empty() {
      continue
    }
    let fields = line.split("|").to_array()
    if fields.length() >= 10 {
      let mem = parse_number(fields[8])
      rows.push({
        pid: parse_number(fields[0]),
        name: fields[1].trim().to_owned(),
        ppid: parse_number(fields[2]),
        path: fields[3].trim().to_owned(),
        tcp_ports: fields[4].trim().to_owned(),
        udp_ports: fields[5].trim().to_owned(),
        started_s: parse_number(fields[6]),
        cpu_x10: parse_x10(fields[7]),
        mem_kb: if mem > 0 { mem } else { -1 },
        listeners: fields[9].trim().to_owned(),
      })
    }
  }
  rows
}


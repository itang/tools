// One listener: the process owning it (name and pid may be unknown without
// permission), the protocol and the local address.
priv struct Entry {
  name : String
  pid : Int
  proto : String
  addr : String
}

// Splits one line of `lsof`/`ss` output into its non-empty fields.
fn words(line : StringView) -> Array[StringView] {
  line
    .split(" ")
    .flat_map(fn(chunk) { chunk.split("\t") })
    .filter(fn(part) { !part.is_empty() })
    .to_array()
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
    let proto_at = fields.search_by(fn(field) { field == "TCP" || field == "UDP" }).unwrap_or(-1)
    if proto_at < 0 || proto_at + 1 >= fields.length() {
      continue
    }
    let addr = fields[proto_at + 1].to_owned()
    if !addr_has_port(addr, port) {
      continue
    }
    entries.push({
      name: fields[0].to_owned(),
      pid: parse_number(fields[1]),
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
  self.run_quiet("lsof", ["-nP", "-iTCP:\{port}", "-sTCP:LISTEN"], 8000, fn(tcp) {
    self.run_quiet("lsof", ["-nP", "-iUDP:\{port}"], 8000, fn(udp) {
      done(lsof_entries(tcp, port) + lsof_entries(udp, port))
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
    self.native_port(
      port,
      fn(text) { done(parse_entries(text)) },
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
    self.native_kill(
      pid,
      fn(ok) {
        if ok {
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
// uses a native Rust module compiled into the host: it reads the Win32 TCP/UDP
// tables and process snapshots directly, so no PowerShell or child process is
// spawned and the nsh process allow-list does not apply. macOS pairs lsof with
// ps, Linux pairs ss with ps; a permission limit only empties fields. Rows are
// `pid|name|ppid|path|tcp_ports|udp_ports|started_s|cpu_x10|mem_kb|listeners`,
// the listeners being `PROTO addr` items joined by `;`.
// Runs several lookup commands and calls `done` once with their stdout in
// job order. A missing tool or a failed call contributes an empty string, so
// the parsers see a uniform shape — the same `Promise.all` behaviour the JS
// bridge used to provide, now expressed with the thin `run_command` bridge.
fn Host::run_many(
  self : Host,
  jobs : FixedArray[(String, FixedArray[String])],
  done : (FixedArray[String]) -> Unit,
) -> Unit {
  let n = jobs.length()
  if n == 0 {
    done([])
    return
  }
  let outputs : Array[String] = Array::make(n, "")
  let mut remaining = n
  let finish_one = fn() {
    remaining = remaining - 1
    if remaining == 0 {
      done(FixedArray::from_array(outputs))
    }
  }
  for i = 0; i < n; i = i + 1 {
    let (exe, args) = jobs[i]
    self.run_command(
      exe,
      args,
      20000,
      fn(_code, stdout, _stderr) {
        outputs[i] = stdout
        finish_one()
      },
      fn(_message) { finish_one() },
    )
  }
}

// Collapses a native scan's text to non-empty, newline-joined rows, dropping
// the stray `\r` Windows tools emit.
fn keep_lines(text : String) -> String {
  let lines : Array[String] = []
  for raw in text.split("\n") {
    let line = raw.trim()
    if !line.is_empty() {
      lines.push(line.to_owned())
    }
  }
  lines.join("\n")
}

fn all_digits(value : StringView) -> Bool {
  if value.is_empty() {
    return false
  }
  for c in value {
    let digit = c.to_int() - 48
    if digit < 0 || digit > 9 {
      return false
    }
  }
  true
}

fn is_decimal(value : StringView) -> Bool {
  if value.is_empty() {
    return false
  }
  let mut dot = false
  for c in value {
    if c == '.' {
      if dot {
        return false
      }
      dot = true
    } else {
      let digit = c.to_int() - 48
      if digit < 0 || digit > 9 {
        return false
      }
    }
  }
  true
}

fn leading_digits(value : StringView) -> StringView {
  let mut n : Int32 = 0
  for c in value {
    let digit = c.to_int() - 48
    if digit < 0 || digit > 9 {
      break
    }
    n = n + 1
  }
  value.get_view(end = n).unwrap_or("")
}

// Splits a ps line into leading whitespace fields, keeping the rest — the
// command, which may contain spaces — as the final field. ps' `comm=` column
// can hold a path with spaces, so a plain `words()` would lose it.
fn ps_fields(line : StringView) -> Array[StringView] {
  let fields : Array[StringView] = []
  let mut rest = line
  for _i = 0; _i < 5; _i = _i + 1 {
    let trimmed = rest.trim_start()
    if trimmed.is_empty() {
      break
    }
    match trimmed.find_by(fn(c) { c == ' ' || c == '\t' }) {
      Some(at) => {
        fields.push(trimmed.get_view(end = at).unwrap_or(""))
        rest = trimmed.get_view(start = at).unwrap_or("")
      }
      None => {
        fields.push(trimmed)
        rest = ""
      }
    }
  }
  let last = rest.trim()
  if !last.is_empty() {
    fields.push(last)
  }
  fields
}

// macOS `etime` (`1-02:03:04` or `02:03:04`) as whole seconds.
fn parse_etime(text : StringView) -> Int {
  let mut rest = text
  let mut days = 0
  match rest.find("-") {
    Some(at) => {
      days = parse_number(rest.get_view(end = at).unwrap_or(""))
      rest = rest.get_view(start = at + 1).unwrap_or("")
    }
    None => ()
  }
  let mut seconds = 0
  for part in rest.split(":") {
    seconds = seconds * 60 + parse_number(part)
  }
  days * 86400 + seconds
}

fn port_after_last_colon(addr : StringView) -> StringView {
  match addr.rev_find(":") {
    Some(at) => addr.get_view(start = at + 1).unwrap_or("")
    None => ""
  }
}

// ADDRESS is lsof's NAME column (from field 8 on); words() already made
// `(LISTEN)` its own token, so dropping a trailing one mirrors the regex.
fn lsof_address(fields : Array[StringView]) -> String {
  let n = fields.length()
  let parts : Array[String] = []
  for i = 8; i < n; i = i + 1 {
    let token = fields[i]
    if i == n - 1 && token == "(LISTEN)" {
      continue
    }
    parts.push(token.to_owned())
  }
  parts.join(" ")
}

// Every `pid=NNNN` in an ss line, as the JS global regex did.
fn ss_pids(line : StringView) -> Array[Int] {
  let out : Array[Int] = []
  for rest = line {
    match rest.find("pid=") {
      None => break
      Some(at) => {
        let tail = rest.get_view(start = at + 4).unwrap_or("")
        let digits = leading_digits(tail)
        if digits.is_empty() {
          continue tail
        }
        out.push(parse_number(digits))
        continue tail.get_view(start = digits.char_length()).unwrap_or("")
      }
    }
  }
  out
}

// Per-pid metadata the ps row supplies; ports and addresses accumulate in
// their own maps because they are merged from two commands per platform.
priv struct PsInfo {
  ppid : String
  name : String
  path : String
  started : String
  cpu : String
  mem : String
}

priv struct ScanRow {
  first_port : Int
  line : String
}

fn add_addr(addrs : Map[Int, Array[String]], pid : Int, label : String) -> Unit {
  let list = match addrs.get(pid) {
    Some(a) => a
    None => {
      let a : Array[String] = []
      addrs.set(pid, a)
      a
    }
  }
  if !list.contains(label) {
    list.push(label)
  }
}

// lsof's `-iTCP -sTCP:LISTEN` / `-iUDP` output: COMMAND PID USER FD TYPE
// DEVICE SIZE/OFF NODE NAME...
fn add_lsof(
  text : String,
  proto : String,
  tcp : Map[Int, Array[Int]],
  udp : Map[Int, Array[Int]],
  addrs : Map[Int, Array[String]],
) -> Unit {
  for raw in text.split("\n") {
    let fields = words(raw)
    if fields.length() < 9 || fields[0] == "COMMAND" {
      continue
    }
    let pid = parse_number(fields[1])
    if pid == 0 {
      continue
    }
    let address = lsof_address(fields)
    if address.contains("->") || !address.contains(":") {
      continue
    }
    let port_view = port_after_last_colon(address)
    if !all_digits(port_view) {
      continue
    }
    let port = parse_number(port_view)
    let ports = if proto == "TCP" { tcp } else { udp }
    let list = match ports.get(pid) {
      Some(p) => p
      None => {
        let p : Array[Int] = []
        ports.set(pid, p)
        p
      }
    }
    if !list.contains(port) {
      list.push(port)
    }
    add_addr(addrs, pid, proto + " " + address)
  }
}

fn join_ports_sorted(ports : Array[Int]) -> String {
  ports.sort()
  ports.map(fn(p) { p.to_string() }).join(",")
}

// Turns the accumulated maps into the `|`-delimited rows parse_processes
// reads, ordered by first port like the Rust scanner's output.
fn emit_rows(
  info : Map[Int, PsInfo],
  tcp : Map[Int, Array[Int]],
  udp : Map[Int, Array[Int]],
  addrs : Map[Int, Array[String]],
) -> String {
  let pids : Array[Int] = []
  for pid in tcp.keys() {
    if !pids.contains(pid) {
      pids.push(pid)
    }
  }
  for pid in udp.keys() {
    if !pids.contains(pid) {
      pids.push(pid)
    }
  }
  let rows : Array[ScanRow] = []
  for pid in pids {
    let tcp_ports = tcp.get(pid).unwrap_or([])
    let udp_ports = udp.get(pid).unwrap_or([])
    let tcp_text = join_ports_sorted(tcp_ports)
    let udp_text = join_ports_sorted(udp_ports)
    let first = if tcp_ports.length() > 0 {
      tcp_ports[0]
    } else if udp_ports.length() > 0 {
      udp_ports[0]
    } else {
      0
    }
    let name = match info.get(pid) {
      Some(p) => p.name
      None => ""
    }
    let ppid = match info.get(pid) {
      Some(p) => p.ppid
      None => ""
    }
    let path = match info.get(pid) {
      Some(p) => p.path
      None => ""
    }
    let started = match info.get(pid) {
      Some(p) => p.started
      None => ""
    }
    let cpu = match info.get(pid) {
      Some(p) => p.cpu
      None => ""
    }
    let mem = match info.get(pid) {
      Some(p) => p.mem
      None => ""
    }
    let addr_text = addrs.get(pid).unwrap_or([]).join(";")
    let line = "\{pid}|\{name}|\{ppid}|\{path}|\{tcp_text}|\{udp_text}|\{started}|\{cpu}|\{mem}|\{addr_text}"
    rows.push({ first_port: first, line })
  }
  rows.sort_by(fn(a, b) { a.first_port - b.first_port })
  rows.map(fn(r) { r.line }).join("\n")
}

// macOS: lsof (TCP listeners + UDP bindings) paired with ps. `etime` is a
// wall-clock delta, so the current second turns it into a start timestamp.
fn macos_rows(tcp_text : String, udp_text : String, ps_text : String, now_s : Int) -> String {
  let info : Map[Int, PsInfo] = Map([])
  for raw in ps_text.split("\n") {
    let fields = ps_fields(raw)
    if fields.length() < 6 {
      continue
    }
    if !all_digits(fields[0]) || !all_digits(fields[1]) || !is_decimal(fields[4]) {
      continue
    }
    let pid = parse_number(fields[0])
    if pid == 0 {
      continue
    }
    let comm = fields[5]
    let name = match comm.rev_find("/") {
      Some(at) => comm.get_view(start = at + 1).unwrap_or("").to_owned()
      None => comm.to_owned()
    }
    let path = match comm.rev_find("/") {
      Some(_) => comm.to_owned()
      None => ""
    }
    let started = now_s - parse_etime(fields[2])
    info.set(pid, {
      ppid: fields[1].to_owned(),
      name,
      path,
      started: started.to_string(),
      cpu: parse_x10(fields[4]).to_string(),
      mem: parse_number(fields[3]).to_string(),
    })
  }
  let tcp : Map[Int, Array[Int]] = Map([])
  let udp : Map[Int, Array[Int]] = Map([])
  let addrs : Map[Int, Array[String]] = Map([])
  add_lsof(tcp_text, "TCP", tcp, udp, addrs)
  add_lsof(udp_text, "UDP", tcp, udp, addrs)
  emit_rows(info, tcp, udp, addrs)
}

// Linux: ss paired with ps. `etimes` is already seconds; ps' `comm=` has no
// path on Linux, so `path` stays empty.
fn linux_rows(ss_text : String, ps_text : String, now_s : Int) -> String {
  let info : Map[Int, PsInfo] = Map([])
  for raw in ps_text.split("\n") {
    let fields = ps_fields(raw)
    if fields.length() < 6 {
      continue
    }
    if !all_digits(fields[0]) || !all_digits(fields[1]) ||
      !all_digits(fields[2]) || !is_decimal(fields[4]) {
      continue
    }
    let pid = parse_number(fields[0])
    if pid == 0 {
      continue
    }
    let started = now_s - parse_number(fields[2])
    info.set(pid, {
      ppid: fields[1].to_owned(),
      name: fields[5].to_owned(),
      path: "",
      started: started.to_string(),
      cpu: parse_x10(fields[4]).to_string(),
      mem: parse_number(fields[3]).to_string(),
    })
  }
  let tcp : Map[Int, Array[Int]] = Map([])
  let udp : Map[Int, Array[Int]] = Map([])
  let addrs : Map[Int, Array[String]] = Map([])
  for raw in ss_text.split("\n") {
    let fields = words(raw)
    if fields.length() < 5 {
      continue
    }
    let proto = fields[0].to_owned().to_upper()
    if proto != "TCP" && proto != "UDP" {
      continue
    }
    let local_addr = fields[4]
    let port_view = port_after_last_colon(local_addr)
    if !all_digits(port_view) {
      continue
    }
    let port = parse_number(port_view)
    let ports = if proto == "TCP" { tcp } else { udp }
    for pid in ss_pids(raw) {
      let list = match ports.get(pid) {
        Some(p) => p
        None => {
          let p : Array[Int] = []
          ports.set(pid, p)
          p
        }
      }
      if !list.contains(port) {
        list.push(port)
      }
      add_addr(addrs, pid, proto + " " + local_addr.to_owned())
    }
  }
  emit_rows(info, tcp, udp, addrs)
}

// Runs the platform's port lookup and reports the listeners it finds; a
// permission-limited lookup reports the socket with an empty name and pid 0.
fn Host::scan_processes(
  self : Host,
  done : (String) -> Unit,
  fail : (String) -> Unit,
) -> Unit {
  let os = self.platform()
  if os == "windows" {
    self.native_scan(fn(text) { done(keep_lines(text)) }, fail)
    return
  }
  let now_s = self.now_s()
  if os == "macos" {
    self.run_many(
      [
        ("lsof", ["-nP", "-iTCP", "-sTCP:LISTEN"]),
        ("lsof", ["-nP", "-iUDP"]),
        ("ps", ["-axo", "pid=,ppid=,etime=,rss=,pcpu=,comm="]),
      ],
      fn(out) { done(macos_rows(out[0], out[1], out[2], now_s)) },
    )
    return
  }
  self.run_many(
    [
      ("ss", ["-H", "-l", "-n", "-t", "-u", "-p"]),
      ("ps", ["-eo", "pid=,ppid=,etimes=,rss=,pcpu=,comm="]),
    ],
    fn(out) { done(linux_rows(out[0], out[1], now_s)) },
  )
}

// Seconds `started_s` has been running, or -1 when the start is unknown.
// The clock read is the only host fact; the subtraction is MoonBit.
fn Host::running_seconds(self : Host, started_s : Int) -> Int {
  if started_s == 0 {
    -1
  } else {
    let delta = self.now_s() - started_s
    if delta > 0 { delta } else { 0 }
  }
}

// `started_s` as local `YYYY-MM-DD HH:MM`; an empty label when unknown. The
// timezone-aware calendar fields come from the host; padding and assembly
// are MoonBit.
fn Host::format_time(self : Host, started_s : Int) -> String {
  if started_s == 0 {
    ""
  } else {
    let parts = self.local_time_parts(started_s)
    let pad = fn(n : Int) -> String { if n < 10 { "0\{n}" } else { "\{n}" } }
    "\{parts[0]}-\{pad(parts[1])}-\{pad(parts[2])} \{pad(parts[3])}:\{pad(parts[4])}"
  }
}

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
    let cpu_text = fields[7].trim()
    let mem_text = fields[8].trim()
    // "!" is emitted when a process is visible but its handle cannot be
    // opened; -2 renders as "受限" instead of the ambiguous "—".
    let cpu_x10 = if cpu_text == "!" { -2 } else { parse_x10(cpu_text) }
    let mem_kb = if mem_text == "!" {
      -2
    } else if mem > 0 {
      mem
    } else {
      -1
    }
    rows.push({
      pid: parse_number(fields[0]),
      name: fields[1].trim().to_owned(),
      ppid: parse_number(fields[2]),
      path: fields[3].trim().to_owned(),
      tcp_ports: fields[4].trim().to_owned(),
      udp_ports: fields[5].trim().to_owned(),
      started_s: parse_number(fields[6]),
      cpu_x10: cpu_x10,
      mem_kb: mem_kb,
      listeners: fields[9].trim().to_owned(),
    })
    }
  }
  rows
}


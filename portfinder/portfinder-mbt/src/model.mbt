///|
fn port_list(raw : String) -> String {
  raw
  .split(",")
  .map(fn(part) { part.trim() })
  .filter(fn(part) { !part.is_empty() })
  .join(", ")
}

// A process may listen on TCP and bind UDP at once, so the column says which
// list a port comes from.

///|
fn ports_label(tcp : String, udp : String) -> String {
  let tcp_text = port_list(tcp)
  let udp_text = port_list(udp)
  if tcp_text == "" && udp_text == "" {
    ""
  } else if udp_text == "" {
    "TCP " + tcp_text
  } else if tcp_text == "" {
    "UDP " + udp_text
  } else {
    "TCP " + tcp_text + " · UDP " + udp_text
  }
}

///|
fn cpu_label(cpu_x10 : Int32) -> String {
  if cpu_x10 == -2 {
    return "受限"
  }
  if cpu_x10 < 0 {
    return "—"
  }
  "\{cpu_x10 / 10}.\{cpu_x10 % 10}%"
}

///|
fn mem_label(mem_kb : Int32) -> String {
  if mem_kb == -2 {
    return "受限"
  }
  if mem_kb < 0 {
    return "—"
  }
  if mem_kb >= 1048576 {
    let whole = mem_kb / 1048576
    let tenth = mem_kb % 1048576 * 10 / 1048576
    "\{whole}.\{tenth} GB"
  } else {
    let whole = mem_kb / 1024
    let tenth = mem_kb % 1024 * 10 / 1024
    "\{whole}.\{tenth} MB"
  }
}

// IPv4 or IPv6 from the local address; empty when the socket is a wildcard
// the system does not say the family of.

///|
fn addr_family(addr : String) -> String {
  if addr.is_empty() {
    return ""
  }
  if addr.has_prefix("[") || addr.split(":").count() > 2 {
    "IPv6"
  } else if addr.contains("*") {
    ""
  } else {
    "IPv4"
  }
}

///|
fn entry_short(entry : Entry) -> String {
  let family = addr_family(entry.addr)
  let base = "\{entry.proto} \{entry.addr}"
  if family == "" {
    base
  } else {
    base + "（\{family}）"
  }
}

///|
fn since_label(seconds : Int32) -> String {
  if seconds < 0 {
    return ""
  }
  let minutes = seconds / 60
  let days = minutes / 1440
  let hours = minutes % 1440 / 60
  let mins = minutes % 60
  if days > 0 {
    if hours > 0 {
      "\{days}天\{hours}小时"
    } else {
      "\{days}天"
    }
  } else if hours > 0 {
    if mins > 0 {
      "\{hours}小时\{mins}分"
    } else {
      "\{hours}小时"
    }
  } else if mins > 0 {
    "\{mins}分"
  } else {
    "不到 1 分钟"
  }
}

///|
fn first_port(raw : String) -> Int32 {
  parse_number(raw.split(",").nth(0).unwrap_or("").trim())
}

///|
fn min_port(proc : ProcessRow) -> Int32 {
  let tcp = first_port(proc.tcp_ports)
  let udp = first_port(proc.udp_ports)
  if tcp == 0 {
    udp
  } else if udp == 0 {
    tcp
  } else if tcp < udp {
    tcp
  } else {
    udp
  }
}

// Rows the system withholds a start time for sort after the known ones.

///|
fn compare_start(a : ProcessRow, b : ProcessRow) -> Int32 {
  if a.started_s == 0 && b.started_s == 0 {
    0
  } else if a.started_s == 0 {
    1
  } else if b.started_s == 0 {
    -1
  } else {
    a.started_s - b.started_s
  }
}

// Unknown numbers (`-1`) sort after the known ones.

///|
fn compare_metric(a : Int32, b : Int32) -> Int32 {
  if a < 0 && b < 0 {
    0
  } else if a < 0 {
    1
  } else if b < 0 {
    -1
  } else {
    a - b
  }
}

///|
fn compare_rows(a : ProcessRow, b : ProcessRow, key : String) -> Int32 {
  match key {
    "name" => a.name.compare_ignore_ascii_case(b.name)
    "pid" => a.pid - b.pid
    "ports" => min_port(a) - min_port(b)
    "cpu" => compare_metric(a.cpu_x10, b.cpu_x10)
    "mem" => compare_metric(a.mem_kb, b.mem_kb)
    "uptime" => -compare_start(a, b)
    _ => min_port(a) - min_port(b)
  }
}

///|
fn index_of_pid(rows : Array[ProcessRow], pid : Int32) -> Int32 {
  rows.search_by(fn(row) { row.pid == pid }).unwrap_or(-1)
}

// The rows the query condition leaves: a case-insensitive substring of the
// process name, or every row when the field is empty.

///|
fn visible_rows(
  procs : Array[ProcessRow],
  filter : String,
) -> Array[ProcessRow] {
  let needle = filter.trim().to_lower()
  if needle.is_empty() {
    return procs
  }
  procs.filter(fn(proc) { proc.name.to_lower().contains(needle) })
}

// The list's default order (by first port) and the user's sort share one
// function, so a refresh keeps the order the table shows.

///|
fn sort_processes(
  rows : Array[ProcessRow],
  key : String,
  ascending : Bool,
) -> Unit {
  rows.sort_by(fn(a, b) {
    let order = compare_rows(a, b, key)
    if ascending {
      order
    } else {
      -order
    }
  })
}

///|
fn answer_text(entries : Array[Entry], port : Int32) -> String {
  if entries.is_empty() {
    return "端口 \{port} 空闲"
  }
  let owners : Array[String] = []
  let pids : Array[Int32] = []
  let any_tcp = entries.any(fn(entry) { entry.proto == "TCP" })
  for entry in entries {
    if !pids.contains(entry.pid) {
      pids.push(entry.pid)
      if entry.name.is_empty() {
        owners.push("PID \{entry.pid}")
      } else {
        owners.push("\{entry.name}（PID \{entry.pid}）")
      }
    }
  }
  let verb = if any_tcp { "监听" } else { "绑定" }
  "端口 \{port} 正被 " + owners.join("、") + " " + verb
}

///|
fn address_detail(entries : Array[Entry]) -> String {
  let text = entries.iter().take(3).map(entry_short).to_array().join(" · ")
  if entries.length() > 3 {
    text + " 等"
  } else {
    text
  }
}

///|

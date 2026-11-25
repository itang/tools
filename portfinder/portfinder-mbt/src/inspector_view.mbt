///|
fn socket_pairs(
  proc : ProcessRow,
  entries : Array[Entry],
) -> Array[(String, String)] {
  let pairs : Array[(String, String)] = []
  let mut owns_port = false
  for entry in entries {
    if entry.pid == proc.pid {
      owns_port = true
      pairs.push((entry.proto, entry.addr))
    }
  }
  if owns_port {
    return pairs
  }
  for raw in proc.listeners.split(";") {
    let item = raw.trim().to_owned()
    if item.is_empty() {
      continue
    }
    let parts = item.split(" ").to_array()
    if parts.length() >= 2 {
      pairs.push((parts[0].to_owned(), parts[1].to_owned()))
    }
  }
  pairs
}

///|
fn inspector_field(host : NshHost, label : String, value : String) -> Node {
  let field = el(host, "view", "shrink-0 flex-row items-center gap-1")
  host.append(field, txt(host, "text-xs text-muted-foreground", label))
  host.append(field, txt(host, "text-xs text-foreground tabular-nums", value))
  field
}

// One endpoint: protocol and verb, address, family. The chip shrinks and the
// address truncates, so a long IPv6 endpoint still fits the line.

///|
fn socket_chip(host : NshHost, pair : (String, String)) -> Node {
  let (proto, addr) = pair
  let chip = el(
    host, "view", "flex-row items-center gap-1 overflow-hidden whitespace-nowrap rounded-md border border-border bg-background px-1.5 py-0.5",
  )
  let verb = if proto == "TCP" { "监听" } else { "绑定" }
  host.append(
    chip,
    txt(host, "shrink-0 text-xs text-muted-foreground", "\{proto} \{verb}"),
  )
  host.append(
    chip,
    txt(host, "truncate text-xs text-foreground tabular-nums", addr),
  )
  let family = addr_family(addr)
  if family != "" {
    host.append(
      chip,
      txt(host, "shrink-0 text-xs text-muted-foreground", family),
    )
  }
  chip
}

// The path line: the executable's location, selectable, with a Copy button
// that reports success in place.

///|
fn path_line_view(
  host : NshHost,
  path : String,
  copyable : Bool,
  copied : Bool,
  copy : () -> Unit,
) -> Node {
  let row = el(
    host, "view", "w-full flex-row items-center gap-1.5 overflow-hidden",
  )
  host.append(
    row,
    txt(host, "shrink-0 text-xs text-muted-foreground", "路径"),
  )
  host.append(
    row,
    txt(host, "flex-1 truncate select-text text-xs text-muted-foreground", path),
  )
  if copyable {
    host.append(
      row,
      host.button(
        "shrink-0 rounded border border-border bg-background px-1.5 py-0.5 hover:bg-muted active:opacity-90",
        txt(
          host,
          "text-xs text-muted-foreground",
          if copied {
            "已复制"
          } else {
            "复制"
          },
        ),
        copy,
      ),
    )
  }
  row
}

// The selected row as a detail card: identity and provenance, the executable
// path, then its sockets as chips. The three lines stay in place when nothing
// is selected, so picking a row does not resize the table.

///|
fn inspector_view(
  host : NshHost,
  clock : Clock,
  rows : Array[ProcessRow],
  selected : Int32,
  entries : Array[Entry],
  port : Int32,
  copy_failed : (String) -> Unit,
) -> Node {
  let box = el(
    host, "view", "shrink-0 flex-col gap-1 overflow-hidden rounded-lg bg-muted/40 px-3 py-1.5",
  )
  let head = el(
    host, "view", "h-5 flex-row items-center gap-1.5 overflow-hidden",
  )
  let path_line = el(
    host, "view", "h-5 flex-row items-center gap-1.5 overflow-hidden",
  )
  let socket_line = el(
    host, "view", "h-5 flex-row items-center gap-1.5 overflow-hidden",
  )
  host.append(box, head)
  host.append(box, path_line)
  host.append(box, socket_line)
  if selected < 0 || selected >= rows.length() {
    host.append(
      head,
      txt(host, "text-sm font-medium text-muted-foreground", "未选择进程"),
    )
    host.append(
      path_line,
      txt(
        host, "text-xs text-muted-foreground", "在列表中选择一行，查看父进程、启动时间、路径与监听地址",
      ),
    )
    return box
  }
  let proc = rows[selected]
  let name = if proc.name.is_empty() { "未知进程" } else { proc.name }
  host.append(
    head,
    txt(host, "shrink-0 text-sm font-medium text-foreground", name),
  )
  let badge = el(
    host, "view", "shrink-0 rounded-md border border-border bg-background px-1.5 py-0.5",
  )
  host.append(
    badge,
    txt(host, "text-xs text-muted-foreground tabular-nums", "PID \{proc.pid}"),
  )
  host.append(head, badge)
  host.append(head, txt(host, "shrink-0 text-xs text-muted-foreground", "·"))
  let ppid = if proc.ppid > 0 { proc.ppid.to_string() } else { "—" }
  host.append(head, inspector_field(host, "父进程", ppid))
  host.append(head, txt(host, "shrink-0 text-xs text-muted-foreground", "·"))
  let started = format_time(clock, proc.started_s)
  let started_text = if started.is_empty() { "—" } else { started }
  host.append(head, inspector_field(host, "启动", started_text))
  let path = if proc.path.is_empty() {
    "可执行文件路径不可用"
  } else {
    proc.path
  }
  let copyable = !proc.path.is_empty()
  let copy_version = host.new_signal(0)
  let mut copied = false
  host.insert(path_line, fn() {
    let _ = copy_version.get()
    path_line_view(host, path, copyable, copied, fn() {
      host.write_clipboard(
        path,
        fn() {
          copied = true
          copy_version.bump()
        },
        copy_failed,
      )
    })
  })
  let pairs = socket_pairs(proc, entries)
  if pairs.is_empty() {
    host.append(
      socket_line,
      txt(host, "text-xs text-muted-foreground", "监听地址不可用"),
    )
    return box
  }
  let owns_port = entries.any(fn(entry) { entry.pid == proc.pid })
  let label = if owns_port { "端口 \{port}" } else { "监听地址" }
  host.append(
    socket_line,
    txt(host, "shrink-0 text-xs text-muted-foreground", label),
  )
  for pair in pairs.iter().take(3) {
    host.append(socket_line, socket_chip(host, pair))
  }
  if pairs.length() > 3 {
    host.append(
      socket_line,
      txt(
        host,
        "shrink-0 text-xs text-muted-foreground",
        "+" + (pairs.length() - 3).to_string(),
      ),
    )
  }
  box
}

// The three appearances the app offers: follow the system, always light,
// always dark. The labels are the user's words; the values are nsh's.

///|

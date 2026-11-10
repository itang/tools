fn el(host : Host, tag : String, class : String) -> Node {
  let node = host.element(tag)
  host.set_class(node, class)
  node
}

fn txt(host : Host, class : String, content : String) -> Node {
  let node = el(host, "text", class)
  host.append(node, host.text(content))
  node
}

fn port_list(raw : String) -> String {
  let mut label = ""
  for part in raw.split(",") {
    let port = part.trim().to_owned()
    if port.is_empty() {
      continue
    }
    if label != "" {
      label = label + ", "
    }
    label = label + port
  }
  label
}

// A process may listen on TCP and bind UDP at once, so the column says which
// list a port comes from.
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

fn cpu_label(cpu_x10 : Int) -> String {
  if cpu_x10 < 0 {
    return "—"
  }
  "\{cpu_x10 / 10}.\{cpu_x10 % 10}%"
}

fn mem_label(mem_kb : Int) -> String {
  if mem_kb < 0 {
    return "—"
  }
  if mem_kb >= 1048576 {
    let whole = mem_kb / 1048576
    let tenth = (mem_kb % 1048576) * 10 / 1048576
    "\{whole}.\{tenth} GB"
  } else {
    let whole = mem_kb / 1024
    let tenth = (mem_kb % 1024) * 10 / 1024
    "\{whole}.\{tenth} MB"
  }
}

// IPv4 or IPv6 from the local address; empty when the socket is a wildcard
// the system does not say the family of.
fn addr_family(addr : String) -> String {
  if addr.is_empty() {
    return ""
  }
  let mut bracket = false
  let mut colons = 0
  let mut star = false
  let mut first = true
  for char in addr {
    if first && char == '[' {
      bracket = true
    }
    if char == ':' {
      colons = colons + 1
    }
    if char == '*' {
      star = true
    }
    first = false
  }
  if bracket || colons > 1 {
    "IPv6"
  } else if star {
    ""
  } else {
    "IPv4"
  }
}

fn entry_short(entry : Entry) -> String {
  let family = addr_family(entry.addr)
  let base = "\{entry.proto} \{entry.addr}"
  if family == "" {
    base
  } else {
    base + "（\{family}）"
  }
}

fn since_label(seconds : Int) -> String {
  if seconds < 0 {
    return ""
  }
  let minutes = seconds / 60
  let days = minutes / 1440
  let hours = (minutes % 1440) / 60
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

fn first_port(raw : String) -> Int {
  let parts = raw.split(",").to_array()
  if parts.is_empty() {
    0
  } else {
    parse_number(parts[0].trim())
  }
}

fn min_port(proc : ProcessRow) -> Int {
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
fn compare_start(a : ProcessRow, b : ProcessRow) -> Int {
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
fn compare_metric(a : Int, b : Int) -> Int {
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

fn compare_rows(a : ProcessRow, b : ProcessRow, key : String) -> Int {
  match key {
    "name" => a.name.compare_ignore_ascii_case(b.name)
    "pid" => a.pid - b.pid
    "ports" => min_port(a) - min_port(b)
    "cpu" => compare_metric(a.cpu_x10, b.cpu_x10)
    "mem" => compare_metric(a.mem_kb, b.mem_kb)
    "uptime" => {
      let order = compare_start(a, b)
      if order == 0 { 0 } else { -order }
    }
    _ => min_port(a) - min_port(b)
  }
}

fn index_of_pid(rows : Array[ProcessRow], pid : Int) -> Int {
  let mut index = 0
  for row in rows {
    if row.pid == pid {
      return index
    }
    index = index + 1
  }
  -1
}

// The rows the query condition leaves: a case-insensitive substring of the
// process name, or every row when the field is empty.
fn visible_rows(procs : Array[ProcessRow], filter : String) -> Array[ProcessRow] {
  let needle = filter.trim().to_lower()
  if needle.is_empty() {
    return procs
  }
  let rows : Array[ProcessRow] = []
  for proc in procs {
    if proc.name.to_lower().contains(needle) {
      rows.push(proc)
    }
  }
  rows
}

// The list's default order (by first port) and the user's sort share one
// function, so a refresh keeps the order the table shows.
fn sort_processes(rows : Array[ProcessRow], key : String, ascending : Bool) -> Unit {
  rows.sort_by(fn(a, b) {
    let order = compare_rows(a, b, key)
    if ascending { order } else { -order }
  })
}

fn answer_text(entries : Array[Entry], port : Int) -> String {
  if entries.is_empty() {
    return "端口 \{port} 空闲"
  }
  let owners : Array[String] = []
  let pids : Array[Int] = []
  let mut any_tcp = false
  for entry in entries {
    if entry.proto == "TCP" {
      any_tcp = true
    }
    if !(pids.contains(entry.pid)) {
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

fn address_detail(entries : Array[Entry]) -> String {
  let items : Array[String] = []
  for entry in entries {
    if items.length() < 3 {
      items.push(entry_short(entry))
    }
  }
  let mut text = items.join(" · ")
  if entries.length() > 3 {
    text = text + " 等"
  }
  text
}

fn answer_strip(
  host : Host,
  status : Int,
  entries : Array[Entry],
  port : Int,
  error : String,
) -> Node {
  // No condition: the strip is gone until a port is queried again.
  if status == 0 {
    return el(host, "view", "h-0")
  }
  let strip = el(
    host,
    "view",
    "min-h-9 flex-row items-center gap-2 rounded-md bg-muted px-3 py-2",
  )
  let dot = if status == 3 {
    "w-2 h-2 rounded-full shrink-0 bg-destructive"
  } else if status == 1 {
    "w-2 h-2 rounded-full shrink-0 bg-muted-foreground"
  } else if entries.is_empty() {
    "w-2 h-2 rounded-full shrink-0 bg-muted-foreground"
  } else {
    "w-2 h-2 rounded-full shrink-0 bg-primary"
  }
  host.append(strip, el(host, "view", dot))
  if status == 1 {
    host.append(
      strip,
      txt(host, "flex-1 text-sm text-muted-foreground", "正在查询端口 \{port} …"),
    )
    return strip
  }
  if status == 3 {
    host.append(
      strip,
      txt(host, "flex-1 text-sm text-destructive", "查询失败：\{error}"),
    )
    return strip
  }
  let answer_class = if entries.is_empty() {
    "flex-1 truncate text-sm text-foreground"
  } else {
    "flex-1 truncate text-sm font-medium text-foreground"
  }
  host.append(strip, txt(host, answer_class, answer_text(entries, port)))
  let detail = address_detail(entries)
  if detail != "" {
    host.append(
      strip,
      txt(host, "shrink-0 text-xs text-muted-foreground", detail),
    )
  }
  strip
}

fn table_state(host : Host, message : String, is_error : Bool) -> Node {
  let box = el(
    host,
    "view",
    "h-[140px] items-center justify-center rounded-lg border border-border bg-background",
  )
  let class = if is_error {
    "text-sm text-destructive"
  } else {
    "text-sm text-muted-foreground"
  }
  host.append(box, txt(host, class, message))
  box
}

// nsh/base's DataTable: a virtualized table that owns the header, column
// widths, row hover/selection, sorting indicators and the keyboard. The app
// draws each cell from its own rows; the size comes from the room the layout
// gives the table, so a window resize shows more rows and wider columns.
extern "js" fn Host::process_table(
  self : Host,
  count : Int,
  height : Int,
  name_width : Int,
  ports_width : Int,
  sort_key : String,
  sort_ascending : Bool,
  selected : Int,
  cell : (Int, String) -> Node,
  on_sort : (String, String) -> Unit,
  on_row_press : (Int, String) -> Unit,
  on_select : (Int) -> Unit,
) -> Node =
  #| (h, count, height, nameWidth, portsWidth, sortKey, sortAscending, selected, cell, onSort, onRowPress, onSelect) => {
  #|   const columns = [
  #|     { key: "seq", title: "序号", width: 56, fixed: true, align: "end", cellClass: "text-right", headerClass: "text-right" },
  #|     { key: "name", title: "进程", width: nameWidth, fixed: true, sortable: true },
  #|     { key: "pid", title: "PID", width: 76, align: "end", sortable: true, cellClass: "text-right", headerClass: "text-right" },
  #|     { key: "ports", title: "端口", width: portsWidth, sortable: true },
  #|     { key: "cpu", title: "CPU", width: 76, align: "end", sortable: true, cellClass: "text-right", headerClass: "text-right" },
  #|     { key: "mem", title: "内存", width: 92, align: "end", sortable: true, cellClass: "text-right", headerClass: "text-right" },
  #|     { key: "uptime", title: "运行时长", width: 98, sortable: true },
  #|     { key: "action", title: "", width: 84, selectable: false, movable: false, resizable: false, cellClass: "px-2" },
  #|   ];
  #|   return h.rt.component(h.base.DataTable, {
  #|     columns,
  #|     data: Array.from({ length: count }, (_, row) => ({ row })),
  #|     rowHeight: 32,
  #|     headerHeight: 30,
  #|     rowHeader: false,
  #|     columnSelectable: false,
  #|     cellSelectable: false,
  #|     stripe: false,
  #|     sort: sortKey ? { key: sortKey, direction: sortAscending ? "ascending" : "descending" } : null,
  #|     selection: selected >= 0 ? { row: selected } : null,
  #|     accessibilityLabel: "占用端口的进程",
  #|     class: "h-[" + height + "px] rounded-lg border border-border bg-background",
  #|     classes: {
  #|       header: "border-b border-border bg-muted/40",
  #|       headerCell: "px-3 text-xs font-medium text-muted-foreground",
  #|       headerContent: "gap-1",
  #|       sortIndicator: "text-muted-foreground",
  #|       row: "h-8",
  #|       rowHover: "bg-muted/50",
  #|       rowSelected: "bg-primary/10",
  #|       cell: "px-3 text-sm text-foreground",
  #|     },
  #|     ref: (table) => {
  #|       h.__processTable = table;
  #|       if (table && typeof h.__revealRow === "number" && h.__revealRow >= 0) {
  #|         try {
  #|           table.select({ row: h.__revealRow });
  #|           table.scrollToRow(h.__revealRow, false);
  #|         } catch (e) {}
  #|       }
  #|     },
  #|     renderCell: (value) => cell(value.row, value.key),
  #|     onSortChange: (e) => {
  #|       const s = e && e.sort ? e.sort : null;
  #|       onSort(s ? s.key : "", s && s.direction ? s.direction : "");
  #|     },
  #|     onRowPress: (e) => {
  #|       const r = e && typeof e.row === "number" ? e.row : -1;
  #|       const c = e && typeof e.column === "number" ? e.column : -1;
  #|       if (r >= 0) onRowPress(r, c >= 0 && columns[c] ? columns[c].key : "");
  #|     },
  #|     onSelectionChange: (e) => {
  #|       const sel = e && e.selection ? e.selection : null;
  #|       if (!sel) return;
  #|       onSelect(typeof sel.row === "number" ? sel.row : -1);
  #|     },
  #|   });
  #| }

// Selects a row and brings it into view. The row stays remembered so a table
// rebuilt by a refresh re-applies it instead of coming back unselected;
// `-1` clears it.
extern "js" fn Host::reveal_row(self : Host, row : Int) -> Unit =
  #| (h, row) => {
  #|   h.__revealRow = row;
  #|   if (row >= 0 && h.__processTable) {
  #|     try {
  #|       h.__processTable.select({ row });
  #|       h.__processTable.scrollToRow(row, false);
  #|     } catch (e) {}
  #|   }
  #| }

// Remembers the row the user selected without touching the live table; a
// rebuild re-applies it from the same place `reveal_row` stores it.
extern "js" fn Host::remember_row(self : Host, row : Int) -> Unit =
  #| (h, row) => { h.__revealRow = row; }

fn process_cell(
  host : Host,
  row : Int,
  proc : ProcessRow,
  key : String,
  on_kill : (ProcessRow) -> Unit,
) -> Node {
  match key {
    "seq" => {
      txt(
        host,
        "w-full text-right text-xs text-muted-foreground tabular-nums",
        (row + 1).to_string(),
      )
    }
    "name" => {
      let name = if proc.name.is_empty() { "—" } else { proc.name }
      txt(host, "truncate text-sm font-medium text-foreground", name)
    }
    "pid" => txt(host, "w-full text-right text-sm text-foreground tabular-nums", proc.pid.to_string())
    "ports" => {
      let ports = ports_label(proc.tcp_ports, proc.udp_ports)
      let label = if ports.is_empty() { "—" } else { ports }
      txt(host, "truncate text-sm text-foreground tabular-nums", label)
    }
    "cpu" => {
      let label = cpu_label(proc.cpu_x10)
      txt(host, "w-full truncate whitespace-nowrap text-right text-sm text-foreground tabular-nums", label)
    }
    "mem" => {
      let label = mem_label(proc.mem_kb)
      txt(host, "w-full truncate whitespace-nowrap text-right text-sm text-foreground tabular-nums", label)
    }
    "uptime" => {
      let since = since_label(host.running_seconds(proc.started_s))
      let label = if since.is_empty() { "—" } else { since }
      txt(host, "truncate whitespace-nowrap text-sm text-muted-foreground tabular-nums", label)
    }
    "action" => {
      let cell = el(host, "view", "w-full flex-row items-center justify-center")
      host.append(
        cell,
        host.button(
          "group shrink-0 rounded-md border border-border bg-background px-2 py-0.5 hover:border-destructive/50 hover:bg-destructive/10 active:opacity-90",
          txt(
            host,
            "whitespace-nowrap text-xs font-medium text-muted-foreground group-hover:text-destructive",
            "结束",
          ),
          fn() { on_kill(proc) },
        ),
      )
      cell
    }
    _ => txt(host, "text-sm text-foreground", "")
  }
}

// The table fills the room the window gives it: its height leaves three rows
// at the least, and the two text columns share the extra width. The fixed
// columns (序号 56, PID 76, CPU 76, 内存 92, 运行时长 98, 结束 84) plus the
// ports column's base 244 and a 16-point scrollbar make the base 902.
fn table_metrics(area_w : Int, area_h : Int) -> (Int, Int, Int) {
  let rows = if area_h < 32 + 3 * 32 { 3 } else { (area_h - 32) / 32 }
  let height = 30 + rows * 32 + 2
  let extra = area_w - 16 - 902
  let ports = if extra > 0 { 244 + extra * 45 / 100 } else { 244 + extra }
  let ports = if ports < 140 { 140 } else { ports }
  let name = 176 + extra - (ports - 244)
  let name = if name < 120 { 120 } else { name }
  (height, name, ports)
}

fn process_list(
  host : Host,
  rows : Array[ProcessRow],
  height : Int,
  name_width : Int,
  ports_width : Int,
  sort_key : String,
  sort_ascending : Bool,
  selected : Int,
  on_sort : (String, String) -> Unit,
  on_select : (Int) -> Unit,
  on_kill : (ProcessRow) -> Unit,
) -> Node {
  host.process_table(
    rows.length(),
    height,
    name_width,
    ports_width,
    sort_key,
    sort_ascending,
    selected,
    fn(row, key) { process_cell(host, row, rows[row], key, on_kill) },
    on_sort,
    fn(row, key) {
      if key == "action" && row >= 0 && row < rows.length() {
        on_kill(rows[row])
      }
    },
    on_select,
  )
}

fn list_view(
  host : Host,
  status : Int,
  rows : Array[ProcessRow],
  filter : String,
  error : String,
  height : Int,
  name_width : Int,
  ports_width : Int,
  sort_key : String,
  sort_ascending : Bool,
  selected : Int,
  on_sort : (String, String) -> Unit,
  on_select : (Int) -> Unit,
  on_kill : (ProcessRow) -> Unit,
) -> Node {
  if status == 1 {
    return table_state(host, "正在读取系统进程表 …", false)
  }
  if status == 3 {
    return table_state(host, "进程列表获取失败：\{error}", true)
  }
  if rows.is_empty() {
    let empty = if filter.trim().is_empty() {
      "没有发现监听端口的进程"
    } else {
      "没有名称匹配「\{filter.trim()}」的进程"
    }
    return table_state(host, empty, false)
  }
  process_list(
    host,
    rows,
    height,
    name_width,
    ports_width,
    sort_key,
    sort_ascending,
    selected,
    on_sort,
    on_select,
    on_kill,
  )
}

// The sockets the inspector shows for the row: the queried port's endpoints
// when the row owns it, otherwise everything the process listens on or binds.
fn socket_pairs(proc : ProcessRow, entries : Array[Entry]) -> Array[(String, String)] {
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

fn inspector_field(host : Host, label : String, value : String) -> Node {
  let field = el(host, "view", "shrink-0 flex-row items-center gap-1")
  host.append(field, txt(host, "text-xs text-muted-foreground", label))
  host.append(field, txt(host, "text-xs text-foreground tabular-nums", value))
  field
}

// One endpoint: protocol and verb, address, family. The chip shrinks and the
// address truncates, so a long IPv6 endpoint still fits the line.
fn socket_chip(host : Host, pair : (String, String)) -> Node {
  let (proto, addr) = pair
  let chip = el(
    host,
    "view",
    "flex-row items-center gap-1 overflow-hidden whitespace-nowrap rounded-md border border-border bg-background px-1.5 py-0.5",
  )
  let verb = if proto == "TCP" { "监听" } else { "绑定" }
  host.append(chip, txt(host, "shrink-0 text-xs text-muted-foreground", "\{proto} \{verb}"))
  host.append(chip, txt(host, "truncate text-xs text-foreground tabular-nums", addr))
  let family = addr_family(addr)
  if family != "" {
    host.append(chip, txt(host, "shrink-0 text-xs text-muted-foreground", family))
  }
  chip
}

// The path line: the executable's location, selectable, with a Copy button
// that reports success in place.
fn path_line_view(
  host : Host,
  path : String,
  copyable : Bool,
  copied : Bool,
  copy : () -> Unit,
) -> Node {
  let row = el(host, "view", "w-full flex-row items-center gap-1.5 overflow-hidden")
  host.append(row, txt(host, "shrink-0 text-xs text-muted-foreground", "路径"))
  host.append(
    row,
    txt(host, "flex-1 truncate select-text text-xs text-muted-foreground", path),
  )
  if copyable {
    host.append(
      row,
      host.button(
        "shrink-0 rounded border border-border bg-background px-1.5 py-0.5 hover:bg-muted active:opacity-90",
        txt(host, "text-xs text-muted-foreground", if copied { "已复制" } else { "复制" }),
        copy,
      ),
    )
  }
  row
}

// The selected row as a detail card: identity and provenance, the executable
// path, then its sockets as chips. The three lines stay in place when nothing
// is selected, so picking a row does not resize the table.
fn inspector_view(
  host : Host,
  rows : Array[ProcessRow],
  selected : Int,
  entries : Array[Entry],
  port : Int,
  copy_failed : (String) -> Unit,
) -> Node {
  let box = el(
    host,
    "view",
    "shrink-0 flex-col gap-1 overflow-hidden rounded-lg bg-muted/40 px-3 py-1.5",
  )
  let head = el(host, "view", "h-5 flex-row items-center gap-1.5 overflow-hidden")
  let path_line = el(host, "view", "h-5 flex-row items-center gap-1.5 overflow-hidden")
  let socket_line = el(host, "view", "h-5 flex-row items-center gap-1.5 overflow-hidden")
  host.append(box, head)
  host.append(box, path_line)
  host.append(box, socket_line)
  if selected < 0 || selected >= rows.length() {
    host.append(head, txt(host, "text-sm font-medium text-muted-foreground", "未选择进程"))
    host.append(
      path_line,
      txt(
        host,
        "text-xs text-muted-foreground",
        "在列表中选择一行，查看父进程、启动时间、路径与监听地址",
      ),
    )
    return box
  }
  let proc = rows[selected]
  let name = if proc.name.is_empty() { "未知进程" } else { proc.name }
  host.append(head, txt(host, "shrink-0 text-sm font-medium text-foreground", name))
  let badge = el(
    host,
    "view",
    "shrink-0 rounded-md border border-border bg-background px-1.5 py-0.5",
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
  let started = host.format_time(proc.started_s)
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
    host.append(socket_line, txt(host, "text-xs text-muted-foreground", "监听地址不可用"))
    return box
  }
  let mut owns_port = false
  for entry in entries {
    if entry.pid == proc.pid {
      owns_port = true
    }
  }
  let label = if owns_port { "端口 \{port}" } else { "监听地址" }
  host.append(socket_line, txt(host, "shrink-0 text-xs text-muted-foreground", label))
  let mut shown = 0
  for pair in pairs {
    if shown >= 3 {
      break
    }
    host.append(socket_line, socket_chip(host, pair))
    shown = shown + 1
  }
  if pairs.length() > shown {
    host.append(
      socket_line,
      txt(
        host,
        "shrink-0 text-xs text-muted-foreground",
        "+" + (pairs.length() - shown).to_string(),
      ),
    )
  }
  box
}

fn footer_view(host : Host, count : Int, filter : String, notice : String) -> Node {
  let line = el(host, "view", "h-5 flex-row items-center overflow-hidden")
  if notice != "" {
    host.append(line, txt(host, "truncate text-xs text-destructive", notice))
  } else if !filter.trim().is_empty() {
    host.append(
      line,
      txt(
        host,
        "truncate text-xs text-muted-foreground",
        "共 \{count} 个进程 · 名称匹配「\{filter.trim()}」",
      ),
    )
  } else {
    host.append(
      line,
      txt(
        host,
        "text-xs text-muted-foreground",
        "共 \{count} 个进程 · 含 TCP 监听与 UDP 绑定",
      ),
    )
  }
  line
}

pub fn app(host : Host) -> Node {
  let version = host.new_signal(0)
  let proc_version = host.new_signal(0)
  let notice_version = host.new_signal(0)
  let selection_version = host.new_signal(0)
  let layout_version = host.new_signal(0)
  // Bumped only by 清空: the uncontrolled port and name fields rebuild empty.
  let port_clear_version = host.new_signal(0)
  let name_clear_version = host.new_signal(0)
  let mut port = 8080
  let mut status = 1
  let mut entries : Array[Entry] = []
  let mut error = ""
  let mut procs : Array[ProcessRow] = []
  let mut proc_status = 1
  let mut proc_error = ""
  let mut notice = ""
  let mut sort_key = ""
  let mut sort_ascending = true
  let mut name_filter = ""
  let mut name_draft = ""
  let mut selected_row = -1
  let mut selected_pid = -1
  let mut table_height = 30 + 9 * 32 + 2
  let mut table_name_width = 176
  let mut table_ports_width = 244

  // The app draws its own bar; the platform keeps its caption buttons in
  // the area `title_bar_metrics` reports (Windows' three, macOS' three).
  host.set_title_bar(44)
  let metrics = host.title_bar_metrics()
  let bar_height = if metrics[0] > 0 { metrics[0] } else { 44 }
  let platform = host.platform()
  let pad_left = if metrics[1] > 0 {
    metrics[1] + 12
  } else if platform == "macos" {
    88
  } else {
    16
  }
  let pad_right = if metrics[2] > 0 {
    metrics[2] + 12
  } else if platform == "macos" {
    16
  } else {
    150
  }

  let mut sequence = 0
  let mut proc_sequence = 0
  let mut proc_inflight = 0
  let mut pending_reveal_pid = 0

  // A refresh keeps the selected process selected (matched by pid) and
  // applies a reveal the query asked for while it was in flight. Its result
  // rebuilds the table, so the selection is re-applied to the new instance.
  let refresh_processes = fn() {
    proc_sequence = proc_sequence + 1
    let current = proc_sequence
    proc_inflight = proc_inflight + 1
    host.scan_processes(
      fn(text) {
        proc_inflight = proc_inflight - 1
        if current == proc_sequence {
          let wanted = if pending_reveal_pid != 0 {
            pending_reveal_pid
          } else {
            selected_pid
          }
          pending_reveal_pid = 0
          procs = parse_processes(text)
          sort_processes(procs, sort_key, sort_ascending)
          proc_status = 2
          proc_error = ""
          let visible = visible_rows(procs, name_filter)
          let target = index_of_pid(visible, wanted)
          selected_row = target
          selected_pid = if target >= 0 { visible[target].pid } else { -1 }
          proc_version.bump()
          selection_version.bump()
          host.reveal_row(target)
        }
      },
      fn(message) {
        proc_inflight = proc_inflight - 1
        if current == proc_sequence {
          pending_reveal_pid = 0
          proc_error = message
          if procs.is_empty() {
            proc_status = 3
            proc_version.bump()
          } else {
            // Keep the rows in view and report the failed refresh beside them.
            notice = "进程列表刷新失败：\{message}"
            notice_version.bump()
          }
        }
      },
    )
  }

  // The query answers the port question and points the table at its owner:
  // the matching row is selected and scrolled into view, or a refresh is
  // asked for when the list lacks it.
  let search = fn() {
    sequence = sequence + 1
    let current = sequence
    pending_reveal_pid = 0
    status = 1
    version.bump()
    host.scan_port(
      port,
      fn(found) {
        if current == sequence {
          entries = found
          status = 2
          let mut reveal = -1
          if found.is_empty() {
            selected_row = -1
            selected_pid = -1
          } else {
            let pid = found[0].pid
            let visible = visible_rows(procs, name_filter)
            let target = index_of_pid(visible, pid)
            selected_row = target
            selected_pid = if target >= 0 { pid } else { -1 }
            reveal = target
            // A row the name condition hides stays hidden; only a pid the
            // list lacks is worth a refresh.
            if target < 0 && index_of_pid(procs, pid) < 0 {
              pending_reveal_pid = pid
              // Let a scan already on its way resolve the pid instead of
              // racing it with a second one.
              if proc_inflight == 0 {
                refresh_processes()
              }
            }
          }
          version.bump()
          selection_version.bump()
          host.reveal_row(reveal)
        }
      },
      fn(message) {
        if current == sequence {
          error = message
          status = 3
          selected_row = -1
          selected_pid = -1
          pending_reveal_pid = 0
          version.bump()
          selection_version.bump()
          host.reveal_row(-1)
        }
      },
    )
  }

  let clear_notice = fn() {
    if notice != "" {
      notice = ""
      notice_version.bump()
    }
  }

  let copy_failed = fn(message : String) {
    notice = "复制路径失败：\{message}"
    notice_version.bump()
  }

  // Both conditions run together: the port lookup answers the strip, the
  // name narrows the list. Changing the name drops the old selection first,
  // then the lookup re-selects its owner when it passes the condition.
  let query = fn() {
    let next_filter = name_draft.trim().to_owned()
    if next_filter != name_filter {
      name_filter = next_filter
      selected_row = -1
      selected_pid = -1
      pending_reveal_pid = 0
      proc_version.bump()
      selection_version.bump()
      host.reveal_row(-1)
    }
    clear_notice()
    if port <= 0 {
      // No port asked: the strip stays empty, the name condition still applies.
      sequence = sequence + 1
      entries = []
      error = ""
      status = 0
      version.bump()
      return
    }
    search()
  }

  let refresh_all = fn() {
    clear_notice()
    if port > 0 {
      search()
    }
    refresh_processes()
  }

  // 清空: both fields empty, the port answer gone, the list back to every
  // process with nothing selected.
  let clear_query = fn() {
    sequence = sequence + 1
    port = 0
    name_draft = ""
    name_filter = ""
    entries = []
    error = ""
    status = 0
    selected_row = -1
    selected_pid = -1
    pending_reveal_pid = 0
    clear_notice()
    port_clear_version.bump()
    name_clear_version.bump()
    version.bump()
    proc_version.bump()
    selection_version.bump()
    host.remember_row(-1)
    refresh_processes()
  }

  let apply_sort = fn(key : String, direction : String) {
    let ascending = direction != "descending"
    if key != sort_key || ascending != sort_ascending {
      sort_key = key
      sort_ascending = ascending
      sort_processes(procs, sort_key, sort_ascending)
      selected_row = -1
      selected_pid = -1
      proc_version.bump()
      selection_version.bump()
      host.reveal_row(-1)
    }
  }

  let select_row = fn(row : Int) {
    let visible = visible_rows(procs, name_filter)
    let pid = if row >= 0 && row < visible.length() { visible[row].pid } else { -1 }
    if row != selected_row || pid != selected_pid {
      selected_row = row
      selected_pid = pid
      host.remember_row(row)
      selection_version.bump()
    }
  }

  let kill_proc = fn(proc : ProcessRow) {
    let name = if proc.name.is_empty() { "该进程" } else { proc.name }
    host.confirm_destructive(
      "结束进程",
      "确定结束 \{name}（PID \{proc.pid}）？它占用的端口会随之释放。",
      "结束进程",
      fn() {
        host.kill_process(
          proc.pid,
          fn() {
            refresh_processes()
            search()
          },
          fn(message) {
            notice = "结束 PID \{proc.pid} 失败：\{message}"
            notice_version.bump()
          },
        )
      },
    )
  }

  let root = el(host, "view", "flex-1 bg-background flex flex-col overflow-hidden")
  let bar = el(
    host,
    "view",
    "shrink-0 flex-row items-center justify-between border-b border-border bg-background h-[\{bar_height}px] pl-[\{pad_left}px] pr-[\{pad_right}px]",
  )
  host.drag_window(bar)
  let bar_left = el(host, "view", "flex-row items-center gap-2")
  host.append(bar_left, host.app_icon("w-[20px] h-[20px]"))
  host.append(bar_left, txt(host, "text-xs text-muted-foreground", "端口雷达"))
  host.append(bar, bar_left)
  host.append(root, bar)

  let content = el(host, "view", "flex-1 flex-col p-4 overflow-hidden")
  let body = el(host, "view", "w-full flex-1 flex flex-col gap-3 overflow-hidden")

  let toolbar = el(host, "view", "shrink-0 flex-row items-center gap-2")
  host.append(toolbar, txt(host, "text-xs text-muted-foreground", "端口"))
  let port_slot = el(host, "view", "flex-row items-center")
  host.insert(port_slot, fn() {
    let _ = port_clear_version.get()
    host.number_input(
      "w-36 rounded-md border border-border bg-background focus-within:border-primary",
      "px-3 py-1.5 text-sm text-foreground",
      port,
      fn(value) { port = value },
      fn(value) {
        port = value
        query()
      },
    )
  })
  host.append(toolbar, port_slot)
  host.append(toolbar, txt(host, "text-xs text-muted-foreground", "名称"))
  let name_slot = el(host, "view", "flex-row items-center")
  host.insert(name_slot, fn() {
    let _ = name_clear_version.get()
    host.text_input(
      "w-40 rounded-md border border-border bg-background px-3 py-1.5 text-sm text-foreground focus:border-primary",
      "进程名",
      "进程名称",
      fn(value) { name_draft = value },
      fn(value) {
        name_draft = value
        query()
      },
    )
  })
  host.append(toolbar, name_slot)
  host.append(
    toolbar,
    host.button(
      "rounded-md bg-primary px-4 py-1.5 hover:bg-primary/90 active:opacity-90",
      txt(host, "text-sm font-medium text-primary-foreground", "查询"),
      fn() { query() },
    ),
  )
  host.append(
    toolbar,
    host.button(
      "rounded-md border border-border px-3 py-1.5 hover:bg-muted active:opacity-90",
      txt(host, "text-sm font-medium text-muted-foreground", "清空"),
      fn() { clear_query() },
    ),
  )
  host.append(body, toolbar)

  let answer_slot = el(host, "view", "shrink-0")
  host.insert(answer_slot, fn() {
    let _ = version.get()
    answer_strip(host, status, entries, port, error)
  })
  host.append(body, answer_slot)

  let section = el(host, "view", "shrink-0 flex-row items-center justify-between")
  host.append(
    section,
    txt(host, "text-sm font-semibold text-foreground", "开放端口的进程"),
  )
  host.append(
    section,
    host.button(
      "rounded-md border border-border px-2.5 py-1 hover:bg-muted active:opacity-90",
      txt(host, "text-xs text-muted-foreground", "刷新"),
      fn() { refresh_all() },
    ),
  )
  host.append(body, section)

  let table_slot = el(host, "view", "w-full flex-1 overflow-hidden")
  host.insert(table_slot, fn() {
    // Selection is the table's own: rebuilding it here would destroy a cell
    // button mid-press, so only the data and the geometry rebuild it.
    let _ = proc_version.get()
    let _ = layout_version.get()
    let rows = visible_rows(procs, name_filter)
    list_view(
      host,
      proc_status,
      rows,
      name_filter,
      proc_error,
      table_height,
      table_name_width,
      table_ports_width,
      sort_key,
      sort_ascending,
      selected_row,
      apply_sort,
      select_row,
      kill_proc,
    )
  })
  host.append(body, table_slot)

  let inspector_slot = el(host, "view", "shrink-0")
  host.insert(inspector_slot, fn() {
    let _ = selection_version.get()
    let rows = visible_rows(procs, name_filter)
    inspector_view(host, rows, selected_row, entries, port, copy_failed)
  })
  host.append(body, inspector_slot)

  let footer_slot = el(host, "view", "shrink-0")
  host.insert(footer_slot, fn() {
    let _ = proc_version.get()
    let _ = notice_version.get()
    footer_view(host, visible_rows(procs, name_filter).length(), name_filter, notice)
  })
  host.append(body, footer_slot)

  host.append(content, body)
  host.append(root, content)

  // A roomy fixed default; everything below the toolbar stretches with the
  // window, so a resize or the maximize button gives the table the room.
  host.set_window_size(1000, 720)
  host.on_layout(table_slot, fn(width, height) {
    if width > 0 && height > 0 {
      let (next_height, next_name, next_ports) = table_metrics(width, height)
      if next_height != table_height ||
        next_name != table_name_width ||
        next_ports != table_ports_width {
        table_height = next_height
        table_name_width = next_name
        table_ports_width = next_ports
        layout_version.bump()
      }
    }
  })

  search()
  refresh_processes()
  root
}

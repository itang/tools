///|
fn table_state(host : NshHost, message : String, is_error : Bool) -> Node {
  let box = el(
    host, "view", "h-[140px] items-center justify-center rounded-lg border border-border bg-background",
  )
  let class = if is_error {
    "text-sm text-destructive"
  } else {
    "text-sm text-muted-foreground"
  }
  host.append(box, txt(host, class, message))
  box
}

// Loading state for the process table: an indeterminate bar that animates
// while the native scan runs on a background thread, plus a short label.

///|
fn table_progress(host : NshHost, message : String) -> Node {
  let box = el(
    host, "view", "h-[140px] flex-col items-center justify-center gap-3 rounded-lg border border-border bg-background",
  )
  host.append(
    box,
    host.progress(
      "w-56", "h-1.5 w-full rounded-full bg-border", "h-1.5 w-1/3 rounded-full bg-primary",
    ),
  )
  host.append(box, txt(host, "text-sm text-muted-foreground", message))
  box
}

///|
fn process_cell(
  host : NshHost,
  clock : Clock,
  row : Int32,
  proc : ProcessRow,
  key : String,
  on_kill : (ProcessRow) -> Unit,
) -> Node {
  match key {
    "seq" =>
      txt(
        host,
        "w-full text-right text-xs text-muted-foreground tabular-nums",
        (row + 1).to_string(),
      )
    "name" => {
      let name = if proc.name.is_empty() { "—" } else { proc.name }
      txt(host, "truncate text-sm font-medium text-foreground", name)
    }
    "pid" =>
      txt(
        host,
        "w-full text-right text-sm text-foreground tabular-nums",
        proc.pid.to_string(),
      )
    "ports" => {
      let ports = ports_label(proc.tcp_ports, proc.udp_ports)
      let label = if ports.is_empty() { "—" } else { ports }
      txt(host, "truncate text-sm text-foreground tabular-nums", label)
    }
    "cpu" => {
      let label = cpu_label(proc.cpu_x10)
      txt(
        host, "w-full truncate whitespace-nowrap text-right text-sm text-foreground tabular-nums",
        label,
      )
    }
    "mem" => {
      let label = mem_label(proc.mem_kb)
      txt(
        host, "w-full truncate whitespace-nowrap text-right text-sm text-foreground tabular-nums",
        label,
      )
    }
    "uptime" => {
      let since = since_label(running_seconds(clock, proc.started_s))
      let label = if since.is_empty() { "—" } else { since }
      txt(
        host, "truncate whitespace-nowrap text-sm text-muted-foreground tabular-nums",
        label,
      )
    }
    "action" => {
      let cell = el(host, "view", "w-full flex-row items-center justify-center")
      host.append(
        cell,
        host.button(
          "group shrink-0 rounded-md border border-border bg-background px-2 py-0.5 hover:border-destructive/50 hover:bg-destructive/10 active:opacity-90",
          txt(
            host, "whitespace-nowrap text-xs font-medium text-muted-foreground group-hover:text-destructive",
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

///|
fn table_metrics(area_w : Int32, area_h : Int32) -> (Int32, Int32, Int32) {
  let rows = if area_h < 32 + 3 * 32 { 3 } else { (area_h - 32) / 32 }
  let height = 30 + rows * 32 + 2
  let extra = area_w - 16 - 902
  let ports = if extra > 0 { 244 + extra * 45 / 100 } else { 244 + extra }
  let ports = if ports < 140 { 140 } else { ports }
  let name = 176 + extra - (ports - 244)
  let name = if name < 120 { 120 } else { name }
  (height, name, ports)
}

///|
fn process_list(
  host : NshHost,
  clock : Clock,
  table : TableState,
  rows : Array[ProcessRow],
  height : Int32,
  name_width : Int32,
  ports_width : Int32,
  sort_key : String,
  sort_ascending : Bool,
  selected : Int32,
  on_sort : (String, String) -> Unit,
  on_select : (Int32) -> Unit,
  on_kill : (ProcessRow) -> Unit,
) -> Node {
  host.process_table(
    table,
    rows.length(),
    height,
    name_width,
    ports_width,
    sort_key,
    sort_ascending,
    selected,
    fn(row, key) { process_cell(host, clock, row, rows[row], key, on_kill) },
    on_sort,
    fn(row, key) {
      if key == "action" && row >= 0 && row < rows.length() {
        on_kill(rows[row])
      }
    },
    on_select,
  )
}

///|
fn list_view(
  host : NshHost,
  clock : Clock,
  table : TableState,
  status : Int32,
  rows : Array[ProcessRow],
  filter : String,
  error : String,
  height : Int32,
  name_width : Int32,
  ports_width : Int32,
  sort_key : String,
  sort_ascending : Bool,
  selected : Int32,
  on_sort : (String, String) -> Unit,
  on_select : (Int32) -> Unit,
  on_kill : (ProcessRow) -> Unit,
) -> Node {
  if status == 1 {
    return table_progress(host, "正在读取系统进程表 …")
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
    host, clock, table, rows, height, name_width, ports_width, sort_key, sort_ascending,
    selected, on_sort, on_select, on_kill,
  )
}

// The sockets the inspector shows for the row: the queried port's endpoints
// when the row owns it, otherwise everything the process listens on or binds.

///|
